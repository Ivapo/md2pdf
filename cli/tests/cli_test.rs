//! The exit gates that reach the binary, at the binary level.
//!
//! These run the real `md2pdf` binary, so they cover the argument contract and
//! the exit codes that the library tests cannot reach.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_md2pdf");

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../tests/fixtures")
        .join(name)
}

fn golden(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../tests/golden")
        .join(name);
    std::fs::read_to_string(path).unwrap()
}

fn run(args: &[&std::ffi::OsStr]) -> Output {
    Command::new(BIN).args(args).output().unwrap()
}

/// A scratch directory that this test process owns, so runs do not collide.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("md2pdf-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir.join(name)
}

/// A scratch directory of its own, for a test that needs a document and the
/// files beside it rather than one file.
fn scratch_dir(name: &str) -> PathBuf {
    let dir = scratch(name);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn emit_typst_prints_the_golden_file() {
    for (fixture_name, golden_name) in [
        ("basic.md", "basic.typ"),
        ("hostile.md", "hostile.typ"),
        ("frontmatter.md", "frontmatter.typ"),
        ("single_column.md", "single_column.typ"),
        ("inline.md", "inline.typ"),
        ("hostile_code.md", "hostile_code.typ"),
        ("blocks.md", "blocks.typ"),
        ("list_spacing.md", "list_spacing.typ"),
        ("links.md", "links.typ"),
        ("table.md", "table.typ"),
        // Emission reads paths and no bytes. This fixture names `fig#2.png`,
        // which no directory holds, and it still prints its golden file.
        ("images.md", "images.typ"),
        ("footnotes.md", "footnotes.typ"),
        // This one names `dot.png`, which emission reads no bytes from either.
        ("strikethrough.md", "strikethrough.typ"),
    ] {
        let out = run(&[fixture(fixture_name).as_ref(), "--emit-typst".as_ref()]);
        assert!(out.status.success(), "{fixture_name} did not exit 0");
        assert_eq!(String::from_utf8(out.stdout).unwrap(), golden(golden_name));
    }
}

#[test]
fn the_o_flag_writes_a_pdf() {
    let output = scratch("explicit.pdf");
    let out = run(&[fixture("basic.md").as_ref(), "-o".as_ref(), output.as_ref()]);
    assert!(out.status.success(), "the run failed: {:?}", out);

    let bytes = std::fs::read(&output).unwrap();
    assert!(bytes.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(!bytes.is_empty());
}

#[test]
fn without_o_the_pdf_lands_beside_the_input() {
    // Copy the fixture, so the default output path stays inside the scratch
    // directory and the repository stays clean.
    let input = scratch("default.md");
    std::fs::copy(fixture("basic.md"), &input).unwrap();

    let out = run(&[input.as_ref()]);
    assert!(out.status.success(), "the run failed: {:?}", out);

    let expected = input.with_extension("pdf");
    let bytes = std::fs::read(&expected).unwrap();
    assert!(bytes.starts_with(b"%PDF"), "the output is not a PDF");
}

/// Rejection survives the widening. Images left the out-of-dialect list in
/// `mpdf-002`'s Phase 1, and a raw HTML block took over this gate.
#[test]
fn a_raw_html_block_exits_non_zero_and_names_it() {
    let out = run(&[fixture("unsupported_html.md").as_ref()]);
    assert!(!out.status.success(), "the run should have failed");

    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(stderr.contains("raw HTML block"), "stderr: {stderr}");
    assert!(stderr.contains("line 5"), "stderr: {stderr}");
}

/// Math and a task list marker exit non-zero and name themselves.
///
/// Each was an unreachable arm until this phase set its parser option, so each
/// printed its markers on the page while the code claimed to refuse it. Math is
/// refused in both its forms.
#[test]
fn math_and_a_task_list_marker_exit_non_zero_and_name_themselves() {
    for (fixture_name, construct) in [
        ("unsupported_math.md", "math"),
        ("unsupported_display_math.md", "math"),
        ("unsupported_task_list.md", "task list marker"),
    ] {
        let out = run(&[fixture(fixture_name).as_ref()]);
        assert!(!out.status.success(), "{fixture_name} should have failed");

        let stderr = String::from_utf8(out.stderr).unwrap();
        assert!(
            stderr.contains(construct),
            "{fixture_name} stderr: {stderr}"
        );
        assert!(stderr.contains("line 3"), "{fixture_name} stderr: {stderr}");
    }
}

/// A document and the files it names convert together.
///
/// The images live in the scratch directory and the test runs from the crate
/// directory, so this pins that a path resolves against the input file rather
/// than against the current directory. The nested `figures/mark.svg` pins that
/// a path in a subdirectory reaches the compiler too.
#[test]
fn a_document_and_its_images_convert() {
    let dir = scratch_dir("figure-doc");
    let input = dir.join("figure.md");
    std::fs::copy(fixture("figure.md"), &input).unwrap();
    std::fs::copy(fixture("dot.png"), dir.join("dot.png")).unwrap();
    std::fs::create_dir_all(dir.join("figures")).unwrap();
    std::fs::copy(fixture("mark.svg"), dir.join("figures/mark.svg")).unwrap();

    let out = run(&[input.as_ref()]);
    assert!(out.status.success(), "the run failed: {:?}", out);

    let bytes = std::fs::read(input.with_extension("pdf")).unwrap();
    assert!(bytes.starts_with(b"%PDF"), "the output is not a PDF");
}

/// The same document beside no images at all. `dot.png` sits next to the
/// fixture, `figures/mark.svg` does not, so the second reference fails.
#[test]
fn a_missing_image_file_names_the_path_the_line_and_the_reason() {
    let out = run(&[fixture("figure.md").as_ref()]);
    assert!(!out.status.success(), "the run should have failed");

    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(stderr.contains("mark.svg"), "stderr: {stderr}");
    assert!(stderr.contains("line 5"), "stderr: {stderr}");
    assert!(stderr.contains("os error"), "stderr: {stderr}");
}

/// Emission needs paths and no bytes, so the flag works on the same document
/// whose second image is absent.
#[test]
fn emit_typst_reads_no_image() {
    let out = run(&[fixture("figure.md").as_ref(), "--emit-typst".as_ref()]);
    assert!(out.status.success(), "the run failed: {:?}", out);

    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.contains(r#"#image("dot.png""#), "stdout: {stdout}");
    assert!(
        stdout.contains(r#"#box(image("figures/mark.svg""#),
        "stdout: {stdout}"
    );
}

#[test]
fn an_unknown_frontmatter_key_exits_non_zero_and_names_it() {
    let out = run(&[fixture("unknown_key.md").as_ref()]);
    assert!(!out.status.success(), "the run should have failed");

    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(stderr.contains("subtitle"), "stderr: {stderr}");
    assert!(stderr.contains("line 3"), "stderr: {stderr}");
}

/// A frontmatter document writes a PDF, not just Typst source.
#[test]
fn a_frontmatter_document_writes_a_pdf() {
    let output = scratch("columns.pdf");
    let out = run(&[
        fixture("single_column.md").as_ref(),
        "-o".as_ref(),
        output.as_ref(),
    ]);
    assert!(out.status.success(), "the run failed: {:?}", out);

    let bytes = std::fs::read(&output).unwrap();
    assert!(bytes.starts_with(b"%PDF"), "the output is not a PDF");
}
