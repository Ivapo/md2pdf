//! The Phase 1 exit gate, at the binary level.
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

#[test]
fn emit_typst_prints_the_golden_file() {
    for (fixture_name, golden_name) in [("basic.md", "basic.typ"), ("hostile.md", "hostile.typ")] {
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

#[test]
fn an_unsupported_construct_exits_non_zero_and_names_it() {
    let out = run(&[fixture("unsupported_list.md").as_ref()]);
    assert!(!out.status.success(), "the run should have failed");

    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(stderr.contains("bullet list"), "stderr: {stderr}");
    assert!(stderr.contains("line 5"), "stderr: {stderr}");
}

#[test]
fn frontmatter_warns_on_stderr_but_still_succeeds() {
    let input = scratch("frontmatter.md");
    std::fs::write(&input, "---\ntitle: Ignored\n---\n\n# Heading\n\nBody.\n").unwrap();

    let out = run(&[input.as_ref(), "--emit-typst".as_ref()]);
    assert!(out.status.success(), "the run failed: {:?}", out);

    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(stderr.contains("frontmatter"), "stderr: {stderr}");

    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(
        !stdout.contains("Ignored"),
        "the frontmatter leaked into the body"
    );
}
