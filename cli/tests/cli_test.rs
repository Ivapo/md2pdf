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

fn sample(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../samples")
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
        ("math.md", "math.typ"),
        ("display_math.md", "display_math.typ"),
        ("numbered_equations.md", "numbered_equations.typ"),
        ("captions.md", "captions.typ"),
        ("captioned_blocks.md", "captioned_blocks.typ"),
        ("cross_references.md", "cross_references.typ"),
        ("equation_names.md", "equation_names.typ"),
        ("plain_equation_names.md", "plain_equation_names.typ"),
        ("groups.md", "groups.typ"),
        ("sectioned_figures.md", "sectioned_figures.typ"),
        ("table.md", "table.typ"),
        // Emission reads paths and no bytes. This fixture names `fig#2.png`,
        // which no directory holds, and it still prints its golden file.
        ("images.md", "images.typ"),
        ("footnotes.md", "footnotes.typ"),
        // This one names `dot.png`, which emission reads no bytes from either.
        ("strikethrough.md", "strikethrough.typ"),
        ("dated.md", "dated.typ"),
        // The only fixture whose import line names a look other than the
        // default one.
        ("press_release.md", "press_release.typ"),
        // The only fixture that is four files. `--emit-typst` reads no image and
        // no bibliography, but it does read the sections: a master is not a
        // document until they are joined in, so there would be nothing to emit.
        ("multi_file.md", "multi_file.typ"),
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
/// printed its markers on the page while the code claimed to refuse it.
///
/// Math was refused in both its forms when this phase shipped, and `mpdf-004`
/// took both of them into the dialect — the inline form in its Phase 1 and the
/// display form in its Phase 2. So what the first fixture names is no longer the
/// span but the LaTeX inside it, and the marker is the only construct left here
/// that is refused whole.
#[test]
fn math_and_a_task_list_marker_exit_non_zero_and_name_themselves() {
    for (fixture_name, construct) in [
        ("unsupported_math.md", r"\includegraphics"),
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

/// A document, its bibliography and the PDF that carries the reference list.
///
/// The second channel the caller reads, tested the way the image channel
/// already is: the file travels beside the document, and the binary is what
/// opens it.
#[test]
fn a_document_and_its_bibliography_convert() {
    let dir = scratch_dir("citations-doc");
    let input = dir.join("citations.md");
    std::fs::copy(fixture("citations.md"), &input).unwrap();
    std::fs::copy(fixture("refs.yml"), dir.join("refs.yml")).unwrap();

    let out = run(&[input.as_ref()]);
    assert!(out.status.success(), "the run failed: {:?}", out);

    let bytes = std::fs::read(input.with_extension("pdf")).unwrap();
    assert!(bytes.starts_with(b"%PDF"), "the output is not a PDF");
}

/// The same document beside no bibliography at all.
///
/// `refs.yml` sits in `tests/fixtures` next to the fixture, so this runs from a
/// scratch directory holding the document alone.
#[test]
fn a_missing_bibliography_file_names_the_path_the_line_and_the_reason() {
    let dir = scratch_dir("citations-alone");
    let input = dir.join("citations.md");
    std::fs::copy(fixture("citations.md"), &input).unwrap();

    let out = run(&[input.as_ref()]);
    assert!(!out.status.success(), "the run should have failed");

    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(stderr.contains("refs.yml"), "stderr: {stderr}");
    assert!(stderr.contains("line 3"), "stderr: {stderr}");
    assert!(stderr.contains("os error"), "stderr: {stderr}");
}

/// Emission needs paths and no bytes on either channel, so the flag works on
/// the document whose bibliography is not beside it.
#[test]
fn emit_typst_reads_no_bibliography() {
    let dir = scratch_dir("citations-emit");
    let input = dir.join("citations.md");
    std::fs::copy(fixture("citations.md"), &input).unwrap();

    let out = run(&[input.as_ref(), "--emit-typst".as_ref()]);
    assert!(out.status.success(), "the run failed: {:?}", out);

    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(
        stdout.contains(r#"#cite(label("DBLP:books/lib/Knuth86a"))"#),
        "stdout: {stdout}"
    );
    assert!(
        stdout.contains(r#"#bibliography("refs.yml", title: none)"#),
        "stdout: {stdout}"
    );
}

/// A key the bibliography does not hold exits non-zero and names its line.
///
/// **On a compile run, and that is the assertion.** Both of this phase's
/// refusals need the bibliography's bytes, so neither can run on `--emit-typst`
/// — which is correct, and which `emit_typst_reads_no_bibliography` above pins
/// from the other side. Without the check the same run prints Typst's own
/// ``typst compilation failed: citation key `nosuchkey` is not present in the
/// bibliography``: the key, and no line.
#[test]
fn an_absent_key_exits_non_zero_and_names_the_key_and_the_line() {
    let dir = scratch_dir("citations-absent");
    let input = dir.join("absent.md");
    std::fs::write(
        &input,
        "---\ntitle: T\nbibliography: refs.yml\n---\n\n# H\n\nA cite [@nosuchkey] here.\n",
    )
    .unwrap();
    std::fs::copy(fixture("refs.yml"), dir.join("refs.yml")).unwrap();

    let out = run(&[input.as_ref()]);
    assert!(!out.status.success(), "the run should have failed");

    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(
        stderr.contains("citation error at line 8"),
        "stderr: {stderr}"
    );
    assert!(stderr.contains("nosuchkey"), "stderr: {stderr}");
    assert!(
        !stderr.contains("typst compilation failed"),
        "the Typst diagnostic still escapes: {stderr}"
    );
}

/// Each citation the dialect refuses exits non-zero and prints its sentence.
///
/// Read here as a user reads them, rather than only as an `Error` value: the
/// three payloads Pandoc spells and this dialect does not read, and a citation
/// in a document that names no bibliography.
#[test]
fn each_refused_citation_exits_non_zero_and_names_its_payload() {
    for (body, needle) in [
        ("A cite [@smith2020] here.", "names no bibliography"),
        ("Several [@a; @b] here.", "cites several sources at once"),
        ("A locator [@k, p. 33] here.", "carries a locator"),
        ("Suppressed [-@k] here.", "suppresses the author"),
    ] {
        let input = scratch(&format!("refused-{}.md", needle.replace(' ', "-")));
        std::fs::write(&input, format!("# H\n\n{body}\n")).unwrap();

        let out = run(&[input.as_ref()]);
        assert!(!out.status.success(), "the run should have failed: {body}");

        let stderr = String::from_utf8(out.stderr).unwrap();
        assert!(
            stderr.contains("citation error at line 3"),
            "stderr: {stderr}"
        );
        assert!(stderr.contains(needle), "stderr: {stderr}");
    }
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

/// A look outside the set exits non-zero, names the key, and lists the names.
///
/// The author who guessed a name needs both halves at the terminal: which key
/// was wrong, and which names would have worked.
#[test]
fn a_template_name_outside_the_set_exits_non_zero_and_lists_the_names() {
    let out = run(&[fixture("unknown_template.md").as_ref()]);
    assert!(!out.status.success(), "the run should have failed");

    let stderr = String::from_utf8(out.stderr).unwrap();
    for needle in ["template", "article", "press-release", "line 3"] {
        assert!(stderr.contains(needle), "stderr: {stderr}");
    }
}

/// A press release writes a PDF, not just Typst source.
///
/// This is the phase's observable at the binary level: one changed frontmatter
/// line, a second look on the page.
#[test]
fn a_press_release_writes_a_pdf() {
    let output = scratch("press-release.pdf");
    let out = run(&[
        fixture("press_release.md").as_ref(),
        "-o".as_ref(),
        output.as_ref(),
    ]);
    assert!(out.status.success(), "the run failed: {:?}", out);

    let bytes = std::fs::read(&output).unwrap();
    assert!(bytes.starts_with(b"%PDF"), "the output is not a PDF");
}

/// The file the binary writes is the bytes the library makes, exactly.
///
/// This is one half of `mpdf-003`'s Phase 3 gate, and it lives here because
/// `CARGO_BIN_EXE_md2pdf` is set only for integration tests of the package that
/// defines that binary. The other half is in `app/src/preview.rs`, where the
/// desktop export writes what its pane holds; the two meet at an in-test
/// `md2pdf_core::md_to_pdf` call, which each of them makes over assets it read
/// itself. The claim they compose is that both front ends produce one file.
///
/// It runs in a scratch directory because the binary's default output path and
/// the desktop export's default output path are the same path.
#[test]
fn the_binarys_file_is_byte_identical_to_the_librarys_bytes() {
    let dir = scratch_dir("byte-identity");
    let input = dir.join("article.md");
    std::fs::copy(sample("article.md"), &input).unwrap();
    std::fs::copy(sample("pipeline.svg"), dir.join("pipeline.svg")).unwrap();
    std::fs::copy(sample("check.svg"), dir.join("check.svg")).unwrap();

    let out = run(&[input.as_ref()]);
    assert!(out.status.success(), "the run failed: {:?}", out);
    let written = std::fs::read(input.with_extension("pdf")).unwrap();

    // The assets are read here rather than by `read_assets`, which is the
    // reader under test: a comparison fed by it would only prove it agrees
    // with itself. `samples/article.md` names each figure once, so there is no
    // dedup subtlety to mirror.
    let markdown = std::fs::read_to_string(&input).unwrap();
    let assets: Vec<md2pdf_core::Asset> = ["pipeline.svg", "check.svg"]
        .into_iter()
        .map(|name| md2pdf_core::Asset {
            path: name.to_string(),
            bytes: std::fs::read(dir.join(name)).unwrap(),
        })
        .collect();

    assert_eq!(written, md2pdf_core::md_to_pdf(&markdown, &assets).unwrap());
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

/// A document with formulas converts, and the PDF carries them.
///
/// `mpdf-004`'s observable at the CLI level: a document that exited non-zero
/// before this phase now produces a PDF.
#[test]
fn a_document_with_formulas_writes_a_pdf() {
    let out_path = scratch("math.pdf");
    let out = run(&[
        fixture("math.md").as_ref(),
        "-o".as_ref(),
        out_path.as_ref(),
    ]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let bytes = std::fs::read(&out_path).unwrap();
    assert!(bytes.starts_with(b"%PDF"), "the output is not a PDF");
}

/// Every shape the dialect refuses inside a formula exits non-zero at the CLI,
/// naming the LaTeX the author typed and its line.
///
/// The library test of the same name proves the error; this one proves the user
/// sees it. The documents are written here rather than kept as fixtures because
/// each is one line long and the set is the point, not the files.
#[test]
fn each_refused_formula_exits_non_zero_and_names_its_latex() {
    for (name, span, named) in [
        (
            "math_includegraphics.md",
            r"\includegraphics{fig.png}",
            r"\includegraphics",
        ),
        ("math_label.md", r"\label{eq}", r"\label"),
        (
            "math_itemize.md",
            r"\begin{itemize}\item a\end{itemize}",
            "itemize",
        ),
        ("math_unknown.md", r"\notacommand", r"\notacommand"),
        ("math_percent.md", "x = 100 % of y", "%"),
        ("math_text.md", r"\text{a}", r"\text"),
    ] {
        let path = scratch(name);
        std::fs::write(&path, format!("# H\n\nA ${span}$ here.\n")).unwrap();

        let out = run(&[path.as_ref()]);
        assert!(!out.status.success(), "{name} should have failed");

        let stderr = String::from_utf8(out.stderr).unwrap();
        assert!(stderr.contains(named), "{name} stderr: {stderr}");
        assert!(stderr.contains("line 3"), "{name} stderr: {stderr}");
    }
}

/// A master and the three files it names, converted from the disk they sit on.
///
/// The library gate covers the join; this one covers the pass that finds the
/// files. `read_sections` resolves each path against the master's directory and
/// reads them before the images, so the whole document exists before anything is
/// asked about it.
#[test]
fn a_master_and_its_sections_convert() {
    let dir = scratch_dir("multi-file");
    std::fs::create_dir_all(dir.join("sections")).unwrap();

    let input = dir.join("multi_file.md");
    std::fs::copy(fixture("multi_file.md"), &input).unwrap();
    for name in ["introduction.md", "method.md", "results.md"] {
        std::fs::copy(
            fixture(&format!("sections/{name}")),
            dir.join("sections").join(name),
        )
        .unwrap();
    }
    // Every path still resolves against the master's directory, a section's own
    // images included. That is Phase 2's job, and this is what it looks like
    // until then.
    for name in ["dot.png", "mark.svg"] {
        std::fs::copy(fixture(name), dir.join(name)).unwrap();
    }

    let out = run(&[input.as_ref()]);
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    let pdf = std::fs::read(dir.join("multi_file.pdf")).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "not a PDF");
}

/// The same master beside no sections at all.
///
/// The third of the same sentence the image and the bibliography already print:
/// the resolved path, the line the master named it on, and the message the OS
/// gave. Not made byte-exact, for the reason the other two are not — the tail is
/// the operating system's own words.
#[test]
fn a_missing_section_file_names_the_path_the_line_and_the_reason() {
    let dir = scratch_dir("multi-file-alone");
    let input = dir.join("multi_file.md");
    std::fs::copy(fixture("multi_file.md"), &input).unwrap();

    let out = run(&[input.as_ref()]);
    assert!(!out.status.success(), "the run should have failed");

    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(
        stderr.contains("sections/introduction.md"),
        "stderr: {stderr}"
    );
    assert!(stderr.contains("line 7"), "stderr: {stderr}");
    assert!(stderr.contains("os error"), "stderr: {stderr}");
}

/// An error inside a section reaches the terminal naming that section's file.
///
/// The whole sentence, because the phrasing is what the phase delivers: the
/// author is sent to a line they can find, in a file they wrote.
#[test]
fn an_error_inside_a_section_reaches_stderr_naming_that_file() {
    let dir = scratch_dir("multi-file-refused");
    std::fs::create_dir_all(dir.join("sections")).unwrap();

    let input = dir.join("master.md");
    std::fs::write(&input, "[](sections/one.md)\n\n[](sections/two.md)\n").unwrap();
    std::fs::write(dir.join("sections/one.md"), "# One\n\nA paragraph.\n").unwrap();
    std::fs::write(
        dir.join("sections/two.md"),
        "# Two\n\nA formula $\\undefinedcmd$ here.\n",
    )
    .unwrap();

    let out = run(&[input.as_ref()]);
    assert!(!out.status.success(), "the run should have failed");

    assert_eq!(
        String::from_utf8(out.stderr).unwrap(),
        "error: math error in sections/two.md at line 3: unsupported command '\\undefinedcmd'\n"
    );
}
