//! The diagram a `mermaid` fence becomes, and every check that stands before it.
//!
//! merman lays the block out and draws it to SVG in process — no browser, no
//! filesystem, no clock and no network, on `wasm32` as natively — and the
//! emitter writes that SVG into the Typst source inline, as the first argument
//! of the look's `diagram` function. Everything merman-shaped lives here: the
//! types the dialect allows, the scan that refuses author configuration, the
//! configuration `core` renders with, the width the look sizes by, and the
//! translation of every failure into [`Error::Diagram`] at the line it belongs
//! to. `specs/diagrams_mermaid_spec.md` (`mpdf-012`) records why each is what
//! it is.

use merman::svg::SvgPipeline;
use merman::{
    Engine, MermaidConfig, OperationControl, ParseOptions, RenderError, RenderOutput,
    RenderRequest, Renderer, SvgRequest,
};

use crate::{Error, Location, Result};

/// The diagram types the dialect allows, keyed on the id merman's detection
/// reports, with the alt text the image carries.
///
/// **Keyed on the id and not on the keyword**, which is what folds `graph`
/// into `flowchart` without a second table: both reach `flowchart-v2`.
/// `flowchart-elk` reports an id of its own, so it is refused here, and the
/// ELK layout is not compiled in anyway. A type joins with a fixture, one at a
/// time, the way `math.rs`'s commands do.
const ALLOWED: [(&str, &str); 2] = [
    ("flowchart-v2", "flowchart"),
    ("sequence", "sequence diagram"),
];

/// The keywords that reach [`ALLOWED`], as a refusal lists them.
const KEYWORDS: &str = "flowchart, graph and sequenceDiagram";

/// The size merman draws a label at, in px, which the look's `diagram` scales
/// to its caption size.
///
/// **It is merman's own default, and `core` does not change it**: the render
/// configuration below sets no `fontSize`, and the scan refuses the directives
/// and front matter that could. It is passed to the look rather than assumed
/// there so that the look carries no hidden constant of the renderer's. The
/// test at the foot of this file holds it against the CSS merman writes.
pub(crate) const LABEL_PX: &str = "16";

/// A rendered diagram, as the emitter writes it.
pub(crate) struct Diagram {
    /// The SVG document, whole.
    pub(crate) svg: String,
    /// The SVG's width in px, **copied verbatim** from its root `viewBox`, so
    /// no float formatting of this crate's enters the Typst source.
    pub(crate) width: String,
    /// What the image says to a screen reader: the type, from [`ALLOWED`].
    pub(crate) alt: &'static str,
}

/// Renders one `mermaid` block, whose fence stands at `fence`.
///
/// The block's first line is the one after the fence, so its line *k*, from
/// zero, is `fence + 1 + k` — which is how every error below finds the line
/// it names.
pub(crate) fn render(content: &str, fence: usize) -> Result<Diagram> {
    scan(content, fence)?;

    let engine = Engine::new().with_site_config(site_config());
    // Detection runs before any layout, so a type outside the list is refused
    // by name even when its body would not parse.
    let detected = match engine.parse_metadata_sync(content) {
        Ok(metadata) => metadata.diagram_type,
        Err(merman::Error::DetectType(_)) => return Err(no_type(fence)),
        Err(error) => return Err(translate(RenderError::from(error), content, fence)),
    };
    let Some(&(_, alt)) = ALLOWED.iter().find(|(id, _)| *id == detected) else {
        let (line, keyword) = keyword(content, fence).unwrap_or((fence, &detected));
        return Err(Error::Diagram {
            location: Location::at(line),
            problem: format!(
                "diagram type '{keyword}' is not supported; the supported types are {KEYWORDS}"
            ),
        });
    };

    // **Never `Engine::try_native`.** `Engine::new` fixes the runtime policy —
    // the clock at the Unix epoch, the time zone at UTC, the seed constant —
    // and the renderer takes its policy from the engine it is given. That is
    // what makes the same block draw the same bytes on every day and machine.
    let renderer = Renderer::new()
        .with_engine(engine)
        .with_parse_options(ParseOptions::strict());
    // The resvg-safe pipeline, always: merman's default SVG puts flowchart
    // labels in `<foreignObject>` HTML, which usvg, and so Typst, does not
    // draw. Every other field keeps merman's default — the widths the spec's
    // fixture was measured at depend on its `viewbox_padding` and its layout.
    let request = SvgRequest {
        pipeline: Some(SvgPipeline::resvg_safe()),
        ..Default::default()
    };
    let output = renderer
        .render(RenderRequest::svg(
            content,
            OperationControl::new(),
            request,
        ))
        .map_err(|error| translate(error, content, fence))?;
    let RenderOutput::Svg(Some(output)) = output else {
        return Err(no_type(fence));
    };

    let svg = output.svg().to_string();
    let width = view_box_width(&svg)
        .ok_or_else(|| Error::Internal("the renderer's SVG carries no viewBox width".to_string()))?
        .to_string();
    Ok(Diagram { svg, width, alt })
}

/// The one configuration every diagram is drawn with: merman's site config,
/// the host-owned layer beside the author's source.
///
/// - **`neutral`** is merman's greyscale theme, which both looks are.
/// - **The spacing is for print.** Mermaid's defaults are sized for a screen;
///   these made the spike's flowchart 18% narrower and its sequence diagram
///   24%, and `mirrorActors: false` drops the second row of actor boxes a
///   sequence diagram repeats at its foot.
/// - **No `fontFamily`.** Mermaid names families `core` does not bundle, so
///   Typst draws every label in the look's own text font, and a look with a
///   different font gets it with nothing here knowing.
fn site_config() -> MermaidConfig {
    MermaidConfig::from_value(serde_json::json!({
        "theme": "neutral",
        "flowchart": {
            "nodeSpacing": 30,
            "rankSpacing": 30,
            "padding": 8,
            "diagramPadding": 4,
        },
        "sequence": {
            "actorMargin": 20,
            "messageMargin": 25,
            "diagramMarginX": 10,
            "width": 100,
            "mirrorActors": false,
        },
    }))
}

/// Refuses what an author may not write inside a block: a directive, or YAML
/// front matter. Either can set the theme, the font size or the spacing, and
/// the look's sizing rule assumes the label size [`LABEL_PX`] names.
///
/// - **`%%{` anywhere in the block**, not only at a line's start. merman looks
///   for directives across the whole input and applies `init` wherever it finds
///   one — after diagram text on the same line, or inside a `%%` comment.
/// - **A first non-blank line that is `---` once trimmed.** merman's own
///   front-matter reader accepts indentation, trailing whitespace and a CRLF
///   around the dashes, so a literal comparison would pass `--- `.
///
/// **Textual, not a reading of merman's parsed config**: a malformed directive
/// is silently ignored by merman and the diagram renders, so a check on the
/// parsed result would pass exactly the case an author most needs told about.
fn scan(content: &str, fence: usize) -> Result<()> {
    let mut seen_text = false;
    for (index, line) in content.lines().enumerate() {
        let problem = if !seen_text && line.trim() == "---" {
            "front matter is not allowed in a diagram"
        } else if line.contains("%%{") {
            "directive '%%{' is not allowed in a diagram"
        } else {
            seen_text |= !line.trim().is_empty();
            continue;
        };
        return Err(Error::Diagram {
            location: Location::at(fence + 1 + index),
            problem: problem.to_string(),
        });
    }
    Ok(())
}

/// The keyword the author wrote and its line: the first word of the block's
/// first non-blank line that does not begin with `%%`.
///
/// Detection reports only the id, so this is how a refusal names what the
/// author typed. It is the line merman's own comment clean-up leaves first,
/// and by the time it is asked [`scan`] has refused every directive, so no
/// `%%{` line can be mistaken for a comment.
fn keyword(content: &str, fence: usize) -> Option<(usize, &str)> {
    content.lines().enumerate().find_map(|(index, line)| {
        let line = line.trim_start();
        if line.is_empty() || line.starts_with("%%") {
            return None;
        }
        let word = line.split_whitespace().next()?;
        Some((fence + 1 + index, word))
    })
}

/// A misspelt keyword, an empty block, or one holding only comments.
fn no_type(fence: usize) -> Error {
    Error::Diagram {
        location: Location::at(fence),
        problem: "no diagram type is recognised".to_string(),
    }
}

/// Every other failure merman reports, in merman's own words, at its line.
///
/// **A syntax error's line is computed, not guessed.** merman reports it with a
/// byte span into the block, already mapped back through its own preprocessing
/// to the text it was given, so the line is the fence's plus one plus the
/// newlines before the span's start. A diagnostic without a span — and any
/// other render failure, a resource budget among them — names the fence's line.
fn translate(error: RenderError, content: &str, fence: usize) -> Error {
    let line = match &error {
        RenderError::Parse(diagnostic) => diagnostic
            .terminal_diagnostic_details()
            .span
            .and_then(|span| content.get(..span.start))
            .map_or(fence, |before| fence + 1 + before.matches('\n').count()),
        _ => fence,
    };
    Error::Diagram {
        location: Location::at(line),
        problem: error.to_string(),
    }
}

/// The third number of the root element's `viewBox`, as merman wrote it.
///
/// It is written into the Typst source unquoted, so it is checked to be a plain
/// decimal before it is trusted there; anything else is `None`, which the
/// caller reports as the renderer's failure rather than the author's.
fn view_box_width(svg: &str) -> Option<&str> {
    let root = &svg[svg.find("<svg")?..];
    let root = &root[..root.find('>')?];
    let values = &root[root.find(" viewBox=\"")? + " viewBox=\"".len()..];
    let values = &values[..values.find('"')?];
    let width = values.split_whitespace().nth(2)?;
    let (whole, fraction) = width.split_once('.').unwrap_or((width, "0"));
    let digits = |part: &str| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit());
    (digits(whole) && digits(fraction)).then_some(width)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FLOWCHART: &str = "flowchart LR\n    a[Read] --> b[Walk] --> c[Write]";
    const SEQUENCE: &str = "sequenceDiagram\n    A->>B: hello\n    B-->>A: hi";

    /// [`LABEL_PX`] is the size merman actually drew at, for each allowed type.
    ///
    /// The look scales by it, so a merman whose default moved would set every
    /// label at the wrong size while every other test passed. The root style
    /// merman writes carries the size its labels were laid out at.
    #[test]
    fn the_label_size_passed_is_the_one_merman_drew_at() {
        for source in [FLOWCHART, SEQUENCE] {
            let diagram = render(source, 1).unwrap();
            assert!(
                diagram.svg.contains(&format!("font-size:{LABEL_PX}px")),
                "{source:?} was not drawn at {LABEL_PX} px"
            );
        }
    }

    /// Each allowed type yields a width the Typst source can carry, and the
    /// alt text its row gives it.
    #[test]
    fn each_allowed_type_yields_a_width_and_its_alt_text() {
        for (source, alt) in [(FLOWCHART, "flowchart"), (SEQUENCE, "sequence diagram")] {
            let diagram = render(source, 1).unwrap();
            assert_eq!(diagram.alt, alt);
            assert!(
                diagram.width.parse::<f64>().unwrap() > 0.0,
                "{source:?} gave {}",
                diagram.width
            );
        }
    }

    /// The same block draws the same bytes twice, which is the determinism the
    /// deterministic runtime policy exists for.
    #[test]
    fn the_same_block_draws_the_same_bytes() {
        for source in [FLOWCHART, SEQUENCE] {
            assert_eq!(
                render(source, 1).unwrap().svg,
                render(source, 1).unwrap().svg
            );
        }
    }

    /// The width is read from the root element and nowhere else, and only as
    /// a plain decimal.
    #[test]
    fn the_width_is_the_root_view_boxs_third_number_verbatim() {
        let root = r#"<svg id="d" style="max-width: 993.1999999999998px;" viewBox="-8 -8 993.1999999999998 400">"#;
        assert_eq!(view_box_width(root), Some("993.1999999999998"));
        assert_eq!(
            view_box_width(r#"<svg viewBox="0 0 470 309">"#),
            Some("470")
        );
        assert_eq!(
            view_box_width(r#"<svg width="100%"><g viewBox="0 0 1 1">"#),
            None
        );
        assert_eq!(view_box_width(r#"<svg viewBox="0 0 1e3 5">"#), None);
        assert_eq!(view_box_width(r#"<svg viewBox="0 0 12) 5">"#), None);
    }
}
