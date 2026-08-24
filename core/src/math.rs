//! The LaTeX a math span may hold, and the check that enforces it.
//!
//! `mitex` converts far more LaTeX than this dialect accepts, and some of what
//! it converts escapes the pipeline's own contracts: `\includegraphics{fig.png}`
//! becomes an `#image` call for a path no check ever saw, `\label{eq}` becomes
//! the empty string, and `\begin{itemize}` becomes markup-mode list syntax that
//! Typst then reads as an operator. None of those are visible in the converted
//! output — the first has no unresolved identifier, the second is not empty
//! input, the third is neither — so the check runs on **what the author wrote**,
//! before conversion, against a closed list. The error then names the LaTeX the
//! author typed rather than some Typst identifier they have never seen.
//!
//! The scan is also this crate's guard against `mitex`'s own panics. Its
//! converter carries five `expect` sites that user input reaches — `\label` with
//! no argument, `\includegraphics` with no group, an environment with no name,
//! `\caption` and `tabular` — and every one of them is unreachable only because
//! the command that leads there is off the list below. A rule here that
//! recovered from a malformed shape instead of refusing it would open them.

use crate::{Error, Location, Result};

/// Every control sequence the dialect accepts, without its backslash.
///
/// The membership is editorial — the constructs an author of technical prose
/// actually reaches for — and it grows one fixture at a time: every entry here
/// has a span in `tests/fixtures/math.md`, and the test at the foot of this file
/// is what holds the two together.
///
/// `\text` is deliberately absent. Its argument reaches the page as Typst
/// *markup* rather than as math, where `\text{= head}` sets a heading and
/// `\text{a \alpha b}` sets the literal words "a alpha b"; the exit is ordinary
/// markdown prose beside a closed formula, and `mpdf-004` OQ-6 carries the work.
pub(crate) const COMMANDS: &[&str] = &[
    // The Greek letters, in both cases, with the variant shapes an author
    // reaches for. LaTeX defines no uppercase for the letters whose capital is a
    // Latin one, so there are eleven rather than twenty-three.
    "alpha",
    "beta",
    "gamma",
    "delta",
    "epsilon",
    "varepsilon",
    "zeta",
    "eta",
    "theta",
    "vartheta",
    "iota",
    "kappa",
    "lambda",
    "mu",
    "nu",
    "xi",
    "pi",
    "varpi",
    "rho",
    "varrho",
    "sigma",
    "tau",
    "upsilon",
    "phi",
    "varphi",
    "chi",
    "psi",
    "omega",
    "Gamma",
    "Delta",
    "Theta",
    "Lambda",
    "Xi",
    "Pi",
    "Sigma",
    "Upsilon",
    "Phi",
    "Psi",
    "Omega",
    // The relations.
    "leq",
    "geq",
    "neq",
    "approx",
    "equiv",
    "sim",
    "propto",
    "to",
    "gets",
    "mapsto",
    // The operators.
    "pm",
    "mp",
    "times",
    "div",
    "cdot",
    "ast",
    "star",
    "circ",
    // The set and logic commands.
    "in",
    "notin",
    "subset",
    "subseteq",
    "supset",
    "cup",
    "cap",
    "setminus",
    "emptyset",
    "forall",
    "exists",
    "neg",
    "land",
    "lor",
    // The large operators.
    "sum",
    "prod",
    "int",
    "oint",
    "lim",
    // The constants and the ellipses.
    "infty",
    "partial",
    "nabla",
    "ldots",
    "cdots",
    "dots",
    // The structural commands.
    "frac",
    "sqrt",
    "binom",
    "left",
    "right",
    // The accents.
    "hat",
    "bar",
    "vec",
    "tilde",
    "dot",
    "overline",
    "underline",
    // The font commands.
    "mathbb",
    "mathbf",
    "mathrm",
    "mathcal",
    "mathit",
    // The named operators.
    "sin",
    "cos",
    "tan",
    "log",
    "ln",
    "exp",
    "min",
    "max",
    "det",
    "gcd",
];

/// Every control symbol the dialect accepts, without its backslash.
///
/// The first five are spacing. The rest are the escapes, and `\%` is why an
/// unescaped `%` can be refused without costing the author anything: a percent
/// sign that means a percent sign is one character away.
pub(crate) const SYMBOLS: &[char] = &['\\', ',', ';', ':', '!', '{', '}', '%', '&', '_', '#', '$'];

/// Every environment the dialect accepts.
///
/// Matched case-sensitively: `mitex` also knows `Bmatrix` and `Vmatrix`, and
/// they are not on this list.
pub(crate) const ENVIRONMENTS: &[&str] = &[
    "matrix", "pmatrix", "bmatrix", "vmatrix", "cases", "aligned",
];

/// One math span's LaTeX, as Typst math markup.
///
/// The result goes between `$` and `$` unescaped: it is markup by the time it
/// is returned, exactly as a code span's content travels as a string, and
/// escaping it would break every formula.
pub(crate) fn convert(latex: &str, line: usize) -> Result<String> {
    let refuse = |problem: String| Error::Math {
        location: Location::at(line),
        problem,
    };

    scan(latex).map_err(refuse)?;

    let markup = mitex::convert_math(latex, None).map_err(refuse)?;
    Ok(normalise(&markup))
}

/// One thing the scan recognises in a span's LaTeX.
///
/// The walk below decides what these are; membership is decided by whoever
/// consumes them. That is what lets the completeness test at the foot of this
/// file read the fixture through the same traversal the emitter refuses with,
/// rather than through a second one that could disagree with it.
enum Token<'a> {
    /// `\` and one or more letters, longest match.
    Command(&'a str),
    /// `\` and exactly one non-letter.
    Symbol(char),
    /// The name inside `\begin{…}` or `\end{…}`.
    Environment(&'a str),
    /// A `%` the author did not escape.
    Percent,
}

/// Refuse anything the dialect does not accept, naming what the author typed.
fn scan(latex: &str) -> std::result::Result<(), String> {
    walk(latex, |token| match token {
        // A percent sign opens a LaTeX comment, so `mitex` discards the rest of
        // the line: `$x = 100 % of y$` converts to `x = 1 0 0` and the PDF shows
        // truncated prose with no error anywhere. That is the silent drop this
        // dialect exists to prevent, reachable by typing `100%` in a formula, so
        // it is refused — and `\%` is the one-character exit, which is what
        // makes this a redirection rather than a ban.
        Token::Percent => Err("unescaped '%' — write '\\%' for a percent sign".to_string()),
        Token::Command(name) if !COMMANDS.contains(&name) => {
            Err(format!("unsupported command '\\{name}'"))
        }
        Token::Symbol(symbol) if !SYMBOLS.contains(&symbol) => {
            Err(format!("unsupported command '\\{symbol}'"))
        }
        Token::Environment(name) if !ENVIRONMENTS.contains(&name) => {
            Err(format!("unsupported environment '{name}'"))
        }
        _ => Ok(()),
    })
}

/// Hand every control sequence, control symbol, environment name and unescaped
/// `%` in a span to `visit`, in the order they are written.
///
/// Every other character passes untouched — letters, digits, operators, braces,
/// `^`, `_`, `&`. Nothing here recovers from a shape it does not recognise: a
/// malformed `\begin` is handed over as the command `\begin`, which no caller
/// accepts, and that refusal is what keeps `mitex`'s unnamed-environment panic
/// out of reach.
fn walk(
    latex: &str,
    mut visit: impl FnMut(Token<'_>) -> std::result::Result<(), String>,
) -> std::result::Result<(), String> {
    let bytes = latex.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'%' => {
                visit(Token::Percent)?;
                i += 1;
            }
            b'\\' => {
                let rest = &latex[i + 1..];
                let letters = rest
                    .find(|c: char| !c.is_ascii_alphabetic())
                    .unwrap_or(rest.len());

                if letters == 0 {
                    // A control symbol is exactly one non-letter character. A
                    // trailing backslash is neither that nor a control
                    // sequence, so it is refused rather than passed on.
                    let Some(symbol) = rest.chars().next() else {
                        return Err("a trailing '\\'".to_string());
                    };
                    visit(Token::Symbol(symbol))?;
                    i += 1 + symbol.len_utf8();
                    continue;
                }

                let name = &rest[..letters];
                if name == "begin" || name == "end" {
                    // `\begin` and `\end` are consumed here rather than allowed
                    // as commands, so the environment name is what gets
                    // checked. The shape is required exactly: a brace straight
                    // after the keyword, letters and `*` inside it, a closing
                    // brace. `mitex`'s own lexer is looser — it skips whitespace
                    // and comments around all three — and being tighter is the
                    // safe direction, because it only ever refuses more.
                    //
                    // Pairing and nesting are deliberately not tracked. `mitex`
                    // closes an environment on any `\end` without comparing
                    // names, an unclosed one keeps its content, and a stray
                    // `\end` is already an error from the converter, so every
                    // case is either visible on the page or named — which is the
                    // same bargain §2 strikes over what `mitex` repairs.
                    let named = rest[letters..]
                        .strip_prefix('{')
                        .and_then(|open| open.split_once('}'))
                        .map(|(name, _)| name)
                        .filter(|name| {
                            !name.is_empty()
                                && name.chars().all(|c| c.is_ascii_alphabetic() || c == '*')
                        });
                    let Some(environment) = named else {
                        visit(Token::Command(name))?;
                        i += 1 + letters;
                        continue;
                    };
                    visit(Token::Environment(environment))?;
                    // Past the backslash, the keyword, and the braced name.
                    i += 1 + letters + 1 + environment.len() + 1;
                    continue;
                }

                visit(Token::Command(name))?;
                i += 1 + letters;
            }
            _ => i += 1,
        }
    }

    Ok(())
}

/// One line of markup, with no trailing backslash.
///
/// Two shapes make this necessary rather than cosmetic. A math span may carry a
/// newline, which `mitex` passes through, and a converted span that broke across
/// lines would put trailing whitespace inside a list item or a block quote,
/// which `prefixed` promises never happens. And `\\` converts to a backslash
/// followed by a space: trimmed to a bare `\`, it would escape the `$` that
/// closes the equation and the compile would fail naming generated source.
fn normalise(markup: &str) -> String {
    let mut out = String::with_capacity(markup.len());
    let mut space = false;

    for ch in markup.chars() {
        if ch.is_whitespace() {
            space = !out.is_empty();
            continue;
        }
        if space {
            out.push(' ');
            space = false;
        }
        out.push(ch);
    }

    if out.ends_with('\\') {
        out.push(' ');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    /// The fixture that sets one formula per command the dialect allows.
    const FIXTURE: &str = include_str!("../../tests/fixtures/math.md");

    /// Every math span in a document, read the way the emitter reads it.
    fn spans(md: &str) -> Vec<String> {
        pulldown_cmark::Parser::new_ext(md, crate::emit::options())
            .filter_map(|event| match event {
                pulldown_cmark::Event::InlineMath(latex) => Some(latex.to_string()),
                _ => None,
            })
            .collect()
    }

    /// The fixture sets every command, symbol and environment the dialect
    /// allows, and nothing it does not.
    ///
    /// This is the mechanical half of the phase's exit gate. The other half is
    /// the golden-file test, which compiles that fixture: a symbol the bundled
    /// prelude misses fails there. Together they mean a command added to a table
    /// above without a formula beside it cannot ship — and the fixture is read
    /// through `walk`, the same traversal the refusal uses, so the two cannot
    /// drift apart the way a substring search over the file would.
    #[test]
    fn the_fixture_sets_every_command_the_dialect_allows() {
        let (mut commands, mut symbols, mut environments) =
            (BTreeSet::new(), BTreeSet::new(), BTreeSet::new());

        for span in spans(FIXTURE) {
            walk(&span, |token| {
                match token {
                    Token::Command(name) => drop(commands.insert(name.to_string())),
                    Token::Symbol(symbol) => drop(symbols.insert(symbol)),
                    Token::Environment(name) => drop(environments.insert(name.to_string())),
                    Token::Percent => panic!("the fixture holds an unescaped '%': {span:?}"),
                }
                Ok(())
            })
            .expect("the fixture is written in the dialect");
            scan(&span).unwrap_or_else(|problem| panic!("{span:?}: {problem}"));
        }

        assert_eq!(
            commands,
            COMMANDS
                .iter()
                .map(|name| name.to_string())
                .collect::<BTreeSet<_>>(),
            "the fixture and the command table disagree"
        );
        assert_eq!(
            symbols,
            SYMBOLS.iter().copied().collect::<BTreeSet<_>>(),
            "the fixture and the control-symbol table disagree"
        );
        assert_eq!(
            environments,
            ENVIRONMENTS
                .iter()
                .map(|name| name.to_string())
                .collect::<BTreeSet<_>>(),
            "the fixture and the environment table disagree"
        );
    }

    /// Each shape the scan refuses names what the author typed.
    #[test]
    fn each_refusal_names_the_latex_it_refuses() {
        for (latex, problem) in [
            (
                r"\includegraphics{fig.png}",
                r"unsupported command '\includegraphics'",
            ),
            (r"\label{eq}", r"unsupported command '\label'"),
            (r"\notacommand", r"unsupported command '\notacommand'"),
            (r"\text{a}", r"unsupported command '\text'"),
            (
                r"\begin{itemize}\item a\end{itemize}",
                "unsupported environment 'itemize'",
            ),
            // A shape the environment rule does not recognise is refused as the
            // keyword itself, rather than recovered from.
            (r"\begin {matrix}", r"unsupported command '\begin'"),
            (r"\begin", r"unsupported command '\begin'"),
            (r"a \| b", r"unsupported command '\|'"),
            (
                "x = 100 % of y",
                "unescaped '%' — write '\\%' for a percent sign",
            ),
            ("x \\", "a trailing '\\'"),
        ] {
            assert_eq!(scan(latex), Err(problem.to_string()), "for {latex:?}");
        }
    }

    /// `Bmatrix` is `mitex`'s, not this dialect's, and case decides it.
    #[test]
    fn the_environment_table_is_matched_case_sensitively() {
        assert_eq!(
            scan(r"\begin{Bmatrix} a \end{Bmatrix}"),
            Err("unsupported environment 'Bmatrix'".to_string())
        );
    }

    /// The converted markup is one line, and never ends in a backslash.
    ///
    /// A span may hold a newline, and `\\` converts to a backslash and a space:
    /// left as they arrive, the first breaks the line inside a list item and the
    /// second escapes the `$` that closes the equation.
    #[test]
    fn the_markup_is_one_line_that_cannot_escape_its_delimiter() {
        assert_eq!(convert("\\alpha +\n\\beta", 1).unwrap(), "alpha + beta");
        assert!(convert(r"a \\", 1).unwrap().ends_with("\\ "));
        for span in spans(FIXTURE) {
            let markup = convert(&span, 1).unwrap();
            assert!(!markup.contains('\n'), "{span:?} converts to two lines");
            assert!(!markup.ends_with('\\'), "{span:?} ends in a backslash");
        }
    }
}
