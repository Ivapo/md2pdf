//! Parses the leading YAML frontmatter block.
//!
//! The schema is six keys, so this is a hand-written parser over a documented
//! YAML subset rather than a dependency. It follows the same policy the emitter
//! applies to markdown: anything outside the subset is an error that names the
//! offending key and its line, never a guess.

use crate::{Error, Result};

/// The bundled looks a document may select.
///
/// Each variant owns three facts: the name the author writes, the file the
/// emitter imports, and the column count the look's convention gives. The
/// emitter and the world both read this one enum, so a look cannot exist under
/// a name that binds no file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Template {
    Article,
    PressRelease,
}

impl Template {
    /// Every bundled look. The world binds all of them.
    pub const ALL: [Template; 2] = [Template::Article, Template::PressRelease];

    /// The name a document writes in the `template` key.
    pub fn name(self) -> &'static str {
        match self {
            Template::Article => "article",
            Template::PressRelease => "press-release",
        }
    }

    /// The file the emitter imports for this look.
    ///
    /// The article look keeps the shipped filename, so the import line of a
    /// document written before the key existed does not move.
    pub fn file(self) -> &'static str {
        match self {
            Template::Article => "template.typ",
            Template::PressRelease => "press-release.typ",
        }
    }

    /// The column count this look's convention gives.
    ///
    /// An article runs in two columns and a press release in one. This applies
    /// only where the document left `columns` out.
    pub fn columns(self) -> u8 {
        match self {
            Template::Article => 2,
            Template::PressRelease => 1,
        }
    }

    fn from_name(name: &str) -> Option<Self> {
        Template::ALL.into_iter().find(|t| t.name() == name)
    }

    /// The accepted names, for the error a name outside the set raises.
    fn names() -> String {
        Template::ALL.map(Template::name).join(" or ")
    }
}

/// Whether a document's display equations are numbered.
///
/// A name checked against a closed set rather than a boolean, so that a later
/// per-section or per-chapter scheme is a new name here rather than a second
/// key. The author decides *whether*; the look decides *how* a number is
/// formatted and where it sits, which is why this crosses to the template as a
/// name and carries no format of its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Equations {
    Plain,
    Numbered,
}

impl Equations {
    /// Every accepted name. The default sits first, as `Template::ALL`'s does.
    pub const ALL: [Equations; 2] = [Equations::Plain, Equations::Numbered];

    /// The name a document writes in the `equations` key, and the string the
    /// emitter hands the look.
    pub fn name(self) -> &'static str {
        match self {
            Equations::Plain => "plain",
            Equations::Numbered => "numbered",
        }
    }

    fn from_name(name: &str) -> Option<Self> {
        Equations::ALL.into_iter().find(|e| e.name() == name)
    }

    /// The accepted names, for the error a name outside the set raises.
    fn names() -> String {
        Equations::ALL.map(Equations::name).join(" or ")
    }
}

/// The layout keys a document may carry.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Frontmatter {
    pub title: Option<String>,
    pub author: Option<String>,
    pub columns: u8,
    pub template: Template,
    pub date: Option<String>,
    pub equations: Equations,
}

impl Default for Frontmatter {
    /// The article look, in the two columns its convention gives.
    ///
    /// The schema is the home of every shipped default, never a template:
    /// `template.typ` names its own fallbacks, but only for a hand-written
    /// call, because the emitter passes every argument on every real call.
    ///
    /// The column count is the one default this struct cannot hold alone. It
    /// follows the selected look, so `parse` resolves it once the whole block
    /// is read, and the value below is what a document with no `template` key
    /// lands on.
    ///
    /// `equations` defaults to the name that numbers nothing, so a document
    /// written before the key existed compiles to the same page it always did.
    fn default() -> Self {
        Self {
            title: None,
            author: None,
            columns: Template::Article.columns(),
            template: Template::Article,
            date: None,
            equations: Equations::Plain,
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
    // The column count follows the selected look, and `template` may sit below
    // `columns` in the block. So the value waits here and resolves after the
    // loop, once both keys are known.
    let mut columns: Option<u8> = None;

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
            // The date is a free string that the template typesets verbatim.
            // Nothing here parses it or formats it, and no clock is read.
            "date" => out.date = non_empty(value),
            "columns" => {
                columns = Some(match value {
                    "1" => 1,
                    "2" => 2,
                    other => {
                        return Err(problem(
                            line,
                            format!("key 'columns' takes 1 or 2, not '{other}'"),
                        ));
                    }
                })
            }
            "template" => {
                let Some(template) = Template::from_name(value) else {
                    return Err(problem(
                        line,
                        format!("key 'template' takes {}, not '{value}'", Template::names()),
                    ));
                };
                out.template = template;
            }
            "equations" => {
                let Some(equations) = Equations::from_name(value) else {
                    return Err(problem(
                        line,
                        format!(
                            "key 'equations' takes {}, not '{value}'",
                            Equations::names()
                        ),
                    ));
                };
                out.equations = equations;
            }
            other => return Err(problem(line, format!("unknown key '{other}'"))),
        }
    }

    // An explicit count wins. An absent one takes the selected look's
    // convention, so a press release is single-column without saying so.
    out.columns = columns.unwrap_or(out.template.columns());
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
    fn both_look_names_parse() {
        for (name, template) in [
            ("article", Template::Article),
            ("press-release", Template::PressRelease),
        ] {
            let out = parse(&format!("template: {name}\n"), 2).unwrap();
            assert_eq!(out.template, template);
        }
    }

    /// An absent `columns` takes the selected look's convention.
    #[test]
    fn the_look_gives_the_column_count_the_document_left_out() {
        assert_eq!(parse("template: article\n", 2).unwrap().columns, 2);
        assert_eq!(parse("template: press-release\n", 2).unwrap().columns, 1);
    }

    /// An explicit count wins over the convention, whichever order they sit in.
    #[test]
    fn an_explicit_column_count_wins_over_the_convention() {
        let below = "template: press-release\ncolumns: 2\n";
        let above = "columns: 2\ntemplate: press-release\n";
        for block in [below, above] {
            assert_eq!(parse(block, 2).unwrap().columns, 2, "block was: {block}");
        }
    }

    /// A name outside the set names the key and lists what it accepts.
    ///
    /// The author who guessed a name needs both halves: which key was wrong,
    /// and which names would have worked.
    #[test]
    fn a_look_outside_the_set_lists_the_names_it_accepts() {
        match parse("template: ieee\n", 2) {
            Err(Error::Frontmatter { problem, .. }) => {
                assert!(problem.contains("template"), "problem was: {problem}");
                assert!(problem.contains("article"), "problem was: {problem}");
                assert!(problem.contains("press-release"), "problem was: {problem}");
            }
            other => panic!("expected a Frontmatter error, got {other:?}"),
        }
    }

    #[test]
    fn both_equation_names_parse() {
        for (name, equations) in [
            ("plain", Equations::Plain),
            ("numbered", Equations::Numbered),
        ] {
            let out = parse(&format!("equations: {name}\n"), 2).unwrap();
            assert_eq!(out.equations, equations);
        }
    }

    /// A document that leaves the key out numbers nothing.
    #[test]
    fn an_absent_equations_key_numbers_nothing() {
        assert_eq!(parse("title: A\n", 2).unwrap().equations, Equations::Plain);
    }

    /// A name outside the set names the key and lists what it accepts, exactly
    /// as the `template` key does. One mechanism, not two.
    #[test]
    fn an_equations_name_outside_the_set_lists_the_names_it_accepts() {
        match parse("equations: yes\n", 2) {
            Err(Error::Frontmatter { problem, .. }) => {
                assert!(problem.contains("equations"), "problem was: {problem}");
                assert!(problem.contains("plain"), "problem was: {problem}");
                assert!(problem.contains("numbered"), "problem was: {problem}");
            }
            other => panic!("expected a Frontmatter error, got {other:?}"),
        }
    }

    #[test]
    fn the_date_is_kept_as_it_was_written() {
        let out = parse("date: 10 August 2026\n", 2).unwrap();
        assert_eq!(out.date.as_deref(), Some("10 August 2026"));
    }

    #[test]
    fn errors_name_the_key_and_the_line() {
        for (block, needle) in [
            ("title: A\nsubtitle: B\n", "subtitle"),
            ("title: A\ncolumns: 3\n", "columns"),
            ("title: A\ntemplate: ieee\n", "press-release"),
            ("title: A\nequations: yes\n", "numbered"),
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
