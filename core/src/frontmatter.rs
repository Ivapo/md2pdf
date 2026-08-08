//! Parses the leading YAML frontmatter block.
//!
//! The schema is three keys, so this is a hand-written parser over a documented
//! YAML subset rather than a dependency. It follows the same policy the emitter
//! applies to markdown: anything outside the subset is an error that names the
//! offending key and its line, never a guess.

use crate::{Error, Result};

/// The layout keys a document may carry.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Frontmatter {
    pub title: Option<String>,
    pub author: Option<String>,
    pub columns: u8,
}

impl Default for Frontmatter {
    /// Two columns. This is the shipped default's one home.
    ///
    /// `template.typ` names `2` as well, but only as the fallback for a
    /// hand-written call. The emitter passes `columns` on every real call, so
    /// the value below is the one every document gets.
    fn default() -> Self {
        Self {
            title: None,
            author: None,
            columns: 2,
        }
    }
}

/// Parse one frontmatter block.
///
/// `block` is the content between the `---` delimiters. `first_line` is that
/// content's 1-based line in the user's file, so every error names a line the
/// user can find.
pub(crate) fn parse(block: &str, first_line: usize) -> Result<Frontmatter> {
    let mut out = Frontmatter::default();
    let mut seen: Vec<&str> = Vec::new();

    for (offset, raw) in block.lines().enumerate() {
        let line = first_line + offset;
        let trimmed = raw.trim();

        // A blank line and a comment line carry nothing.
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // An indented line means nesting, which this subset does not accept.
        if raw.starts_with([' ', '\t']) {
            return Err(problem(line, "nested keys are not supported"));
        }

        let Some((key, value)) = trimmed.split_once(':') else {
            return Err(problem(line, "not a 'key: value' pair"));
        };
        let key = key.trim();
        let value = unquote(value.trim());

        if seen.contains(&key) {
            return Err(problem(line, format!("duplicate key '{key}'")));
        }
        seen.push(key);

        match key {
            // An empty value means the key is absent, so the template omits it.
            "title" => out.title = non_empty(value),
            "author" => out.author = non_empty(value),
            "columns" => {
                out.columns = match value {
                    "1" => 1,
                    "2" => 2,
                    other => {
                        return Err(problem(
                            line,
                            format!("key 'columns' takes 1 or 2, not '{other}'"),
                        ));
                    }
                }
            }
            other => return Err(problem(line, format!("unknown key '{other}'"))),
        }
    }

    Ok(out)
}

fn problem(line: usize, problem: impl Into<String>) -> Error {
    Error::Frontmatter {
        line,
        problem: problem.into(),
    }
}

/// Strip one matching pair of quotes, if the value carries them.
fn unquote(value: &str) -> &str {
    for quote in ['"', '\''] {
        if value.len() >= 2 && value.starts_with(quote) && value.ends_with(quote) {
            return &value[1..value.len() - 1];
        }
    }
    value
}

fn non_empty(value: &str) -> Option<String> {
    (!value.is_empty()).then(|| value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_key_is_optional() {
        assert_eq!(parse("", 2).unwrap(), Frontmatter::default());
    }

    #[test]
    fn quotes_comments_and_blank_lines_are_handled() {
        let block = "# a comment\ntitle: \"A Quoted Title\"\n\nauthor: 'Iva Po'\n";
        let out = parse(block, 2).unwrap();
        assert_eq!(out.title.as_deref(), Some("A Quoted Title"));
        assert_eq!(out.author.as_deref(), Some("Iva Po"));
    }

    #[test]
    fn an_empty_value_means_the_key_is_absent() {
        assert_eq!(parse("title:\n", 2).unwrap().title, None);
    }

    #[test]
    fn a_colon_inside_the_value_stays_in_the_value() {
        let out = parse("title: Typst: A Study\n", 2).unwrap();
        assert_eq!(out.title.as_deref(), Some("Typst: A Study"));
    }

    #[test]
    fn errors_name_the_key_and_the_line() {
        for (block, needle) in [
            ("title: A\nsubtitle: B\n", "subtitle"),
            ("title: A\ncolumns: 3\n", "columns"),
            ("title: A\ntitle: B\n", "title"),
            ("title: A\njust a line\n", "key: value"),
            ("title: A\n  nested: B\n", "nested keys"),
        ] {
            match parse(block, 2) {
                Err(Error::Frontmatter { line, problem }) => {
                    assert_eq!(line, 3, "wrong line for {needle}");
                    assert!(problem.contains(needle), "problem was: {problem}");
                }
                other => panic!("expected a Frontmatter error, got {other:?}"),
            }
        }
    }
}
