//! Parses the leading YAML frontmatter block.
//!
//! The schema is ten keys, so this is a hand-written parser over a documented
//! YAML subset rather than a dependency. It follows the same policy the emitter
//! applies to markdown: anything outside the subset is an error that names the
//! offending key and its line, never a guess.
//!
//! Two of the ten take a *list*, and the parser refusing an indented line is why
//! they take it inside one: a list is a `;`-separated value rather than YAML's
//! own nesting.

use crate::emit::portable_path;
use crate::{BibliographyRef, Error, Location, Result};

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

/// Whether a figure's number carries the section it stands in.
///
/// A name against a closed set, on `Equations`' own shape and for its own
/// reason: a per-chapter restart without a prefix is the same mechanism minus an
/// argument, so it is a third name here rather than a third key. The author
/// decides *whether* a number carries a section; the look decides *how* one is
/// formatted, which is why this crosses to the template as a name and carries no
/// numbering string of its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Figures {
    Flat,
    Sectioned,
}

impl Figures {
    /// Every accepted name. The default sits first, as `Equations::ALL`'s does.
    pub const ALL: [Figures; 2] = [Figures::Flat, Figures::Sectioned];

    /// The name a document writes in the `figures` key, and the string the
    /// emitter hands the look.
    pub fn name(self) -> &'static str {
        match self {
            Figures::Flat => "flat",
            Figures::Sectioned => "sectioned",
        }
    }

    fn from_name(name: &str) -> Option<Self> {
        Figures::ALL.into_iter().find(|f| f.name() == name)
    }

    /// The accepted names, for the error a name outside the set raises.
    fn names() -> String {
        Figures::ALL.map(Figures::name).join(" or ")
    }
}

/// How deep a document numbers its headings.
///
/// A name against a closed set, on `Equations`' and `Figures`' shape, with seven
/// members rather than two. The value is the deepest level that carries a
/// number, because the question is genuinely not binary: a boolean is what
/// leaves an author unable to say "the sections but not their subsections", and
/// a depth is the `secnumdepth` an author already knows. `numbered` is
/// deliberately not an eighth name — it would be a second spelling of `6`, and
/// this schema has no synonyms — so an author who guesses it meets an error
/// that lists the way through.
///
/// The author decides *how deep*; the look decides *how* a number is formatted
/// and where it sits, which is why this crosses to the template as the name it
/// was written as and carries no numbering string of its own. The look converts
/// it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Headings {
    Plain,
    Depth1,
    Depth2,
    Depth3,
    Depth4,
    Depth5,
    Depth6,
}

impl Headings {
    /// Every accepted name. The default sits first, as `Figures::ALL`'s does.
    pub const ALL: [Headings; 7] = [
        Headings::Plain,
        Headings::Depth1,
        Headings::Depth2,
        Headings::Depth3,
        Headings::Depth4,
        Headings::Depth5,
        Headings::Depth6,
    ];

    /// The name a document writes in the `headings` key, and the string the
    /// emitter hands the look.
    ///
    /// The depths are unit variants rather than a `Depth(u8)` carrying its
    /// number, so this match is exhaustive over what exists and needs no
    /// unreachable arm for a depth the schema cannot produce.
    pub fn name(self) -> &'static str {
        match self {
            Headings::Plain => "plain",
            Headings::Depth1 => "1",
            Headings::Depth2 => "2",
            Headings::Depth3 => "3",
            Headings::Depth4 => "4",
            Headings::Depth5 => "5",
            Headings::Depth6 => "6",
        }
    }

    fn from_name(name: &str) -> Option<Self> {
        Headings::ALL.into_iter().find(|h| h.name() == name)
    }

    /// The accepted names, for the error a name outside the set raises.
    fn names() -> String {
        Headings::ALL.map(Headings::name).join(" or ")
    }
}

/// One author, and the affiliations they belong to.
///
/// The markers ride the name because nothing else in a flat schema carries the
/// relation. An affiliation is a relation between two lists rather than a third
/// string, and one `key: value` pair per line leaves the name itself as the only
/// place to write the join.
///
/// `markers` indexes [`Frontmatter::affiliation`] in written order, from 1, and
/// is empty where the author wrote none — which the schema permits at exactly
/// one affiliation, and which the look reads to decide whether a marker reaches
/// the page at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Author {
    pub name: String,
    pub markers: Vec<usize>,
}

/// A list-valued key, and the line the document wrote it on.
///
/// The line is kept for the reason [`crate::BibliographyRef`] keeps one, reached
/// from a different direction: two of the four refusals below are *cross-key*,
/// and `affiliation` may sit above or below `author`, so neither can be checked
/// until the whole block is read. By then the line loop is over and the line is
/// only here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Listed<T> {
    pub list: Vec<T>,
    pub location: Location,
}

/// The keys a document may carry.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Frontmatter {
    pub title: Option<String>,
    /// The authors, in written order, each with the affiliations it names.
    pub author: Option<Listed<Author>>,
    /// The affiliations the markers index, in written order.
    ///
    /// Singular for the reason `author` is: this schema has no synonyms, so one
    /// key takes several values rather than a second key spelling the plural.
    pub affiliation: Option<Listed<String>>,
    pub columns: u8,
    pub template: Template,
    pub date: Option<String>,
    pub equations: Equations,
    pub figures: Figures,
    pub headings: Headings,
    /// The bibliography file this document names, with the line that named it.
    ///
    /// The only key that names a *file*, which is why it alone carries a line:
    /// a missing image is refused at the line the markdown drew it on, and this
    /// is the only place a bibliography's own position is ever known.
    ///
    /// The records are deliberately not in the block. Carrying them here would
    /// cost no new channel, and it would put a bibliography in every document
    /// that cites it rather than in one file several documents share — and it
    /// would mean inventing a record format, or embedding Hayagriva's inside a
    /// YAML subset this hand-written reader parses a line at a time.
    pub bibliography: Option<BibliographyRef>,
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
    /// `equations` defaults to the name that numbers nothing, `figures` to the
    /// name that sections nothing, and `headings` to the name that numbers no
    /// heading, so a document written before any of the three existed compiles
    /// to the same page it always did.
    fn default() -> Self {
        Self {
            title: None,
            author: None,
            affiliation: None,
            columns: Template::Article.columns(),
            template: Template::Article,
            date: None,
            equations: Equations::Plain,
            figures: Figures::Flat,
            headings: Headings::Plain,
            bibliography: None,
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
            // A list, and the one whose elements carry a relation of their own.
            // An empty value means the key is absent, exactly as it does for a
            // string key: `author:` and nothing else names nobody.
            "author" => {
                out.author = match non_empty(value) {
                    Some(value) => Some(Listed {
                        list: entries(&value, key, line)?
                            .iter()
                            .map(|entry| author(entry, line))
                            .collect::<Result<Vec<_>>>()?,
                        location: Location::at(line),
                    }),
                    None => None,
                }
            }
            // The other list. Each element is a whole affiliation, commas and
            // all — `Anthropic, San Francisco` is one place, which is exactly
            // why the separator is a `;`.
            "affiliation" => {
                out.affiliation = match non_empty(value) {
                    Some(value) => Some(Listed {
                        list: entries(&value, key, line)?,
                        location: Location::at(line),
                    }),
                    None => None,
                }
            }
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
            "figures" => {
                let Some(figures) = Figures::from_name(value) else {
                    return Err(problem(
                        line,
                        format!("key 'figures' takes {}, not '{value}'", Figures::names()),
                    ));
                };
                out.figures = figures;
            }
            "headings" => {
                let Some(headings) = Headings::from_name(value) else {
                    return Err(problem(
                        line,
                        format!("key 'headings' takes {}, not '{value}'", Headings::names()),
                    ));
                };
                out.headings = headings;
            }
            // The one key that names a file. Its value takes the shape rule
            // every path in this dialect takes — a document and the files it
            // names travel as one folder — phrased for a key rather than for an
            // image. An empty value means the key is absent, so a document that
            // wrote `bibliography:` and nothing else names none.
            "bibliography" => {
                out.bibliography = match non_empty(value) {
                    Some(path) => {
                        portable_path(&path).map_err(|shape| {
                            problem(
                                line,
                                format!(
                                    "key 'bibliography' takes a path beside the document, not {}",
                                    shape.key()
                                ),
                            )
                        })?;
                        Some(BibliographyRef {
                            path,
                            location: Location::at(line),
                        })
                    }
                    None => None,
                }
            }
            other => return Err(problem(line, format!("unknown key '{other}'"))),
        }
    }

    // An explicit count wins. An absent one takes the selected look's
    // convention, so a press release is single-column without saying so.
    out.columns = columns.unwrap_or(out.template.columns());
    check_affiliations(&out)?;
    Ok(out)
}

/// The two refusals that read both list keys at once.
///
/// They cannot run inside the loop above, because `affiliation` may sit below
/// `author` — the shape `columns` and `template` already take. Each names the
/// line of the key the author would have to edit, which for the first is the
/// `author` line the marker was written on and for the second the `affiliation`
/// line whose relation is unstated.
fn check_affiliations(out: &Frontmatter) -> Result<()> {
    let count = out
        .affiliation
        .as_ref()
        .map_or(0, |listed| listed.list.len());

    // A marker naming an affiliation the document does not carry. Typst cannot
    // catch this: by the time it sees one it is a number in a list, and the
    // failure it would ship is a superscript pointing at nothing.
    //
    // A document with *no* affiliation at all gets its own sentence, naming both
    // ways out. It is the commonest way to meet this error — an `affiliation`
    // line commented out or not yet written — and "the document carries 0" reads
    // as though an affiliation were always required, which is the opposite of
    // what the schema says.
    if let Some(authors) = &out.author {
        for marker in authors.list.iter().flat_map(|author| &author.markers) {
            if *marker == 0 || *marker > count {
                let problem = match count {
                    0 => format!(
                        "key 'author' marks a name '^{marker}' and the document names no 'affiliation'; add that key, or drop the markers"
                    ),
                    _ => format!(
                        "key 'author' points at affiliation {marker}, and 'affiliation' carries {count}"
                    ),
                };
                return Err(self::problem(authors.location.line, problem));
            }
        }
    }

    // An `affiliation` key whose relation to the authors is unstated. This
    // begins at *two*: with exactly one affiliation the markers are optional and
    // every author belongs to it, so the commonest real paper — one lab, several
    // authors — is written without a lone marker on every name.
    if count >= 2 {
        let marked = out
            .author
            .iter()
            .flat_map(|authors| &authors.list)
            .any(|author| !author.markers.is_empty());
        if !marked {
            let listed = out.affiliation.as_ref().expect("count is above zero");
            return Err(problem(
                listed.location.line,
                format!(
                    "key 'affiliation' carries {count} entries and no author points at one with '^'; a marker is optional only where there is exactly one"
                ),
            ));
        }
    }

    Ok(())
}

/// Split a list-valued key on `;`, trimming every element.
///
/// An empty element is refused rather than dropped. `author: Iva Po;` splits to
/// a second, empty element, and `affiliation: MIT;` leaves a blank second
/// affiliation that a `^2` would then point at without tripping the check above.
/// Either would reach the look as a dangling separator, which is the silent
/// flattening this dialect exists to forbid.
fn entries(value: &str, key: &str, line: usize) -> Result<Vec<String>> {
    value
        .split(';')
        .map(str::trim)
        .map(|entry| match entry.is_empty() {
            true => Err(problem(
                line,
                format!("key '{key}' has an empty entry; ';' separates them"),
            )),
            false => Ok(entry.to_string()),
        })
        .collect()
}

/// Split one author into the name and the markers riding it.
///
/// The split is at the *first* `^`, so `A^B^1` is refused by the marker rule
/// naming `B^1` rather than guessed into a name `A^B`: a `^` inside a name is
/// rarer than a typo in a marker, and this dialect refuses rather than guesses.
///
/// A digit run too long for a `usize` saturates rather than raising here. It
/// cannot index any list, so `check_affiliations` refuses it as the marker
/// naming an affiliation the document does not carry, which is what it is.
fn author(entry: &str, line: usize) -> Result<Author> {
    let Some((name, markers)) = entry.split_once('^') else {
        return Ok(Author {
            name: entry.to_string(),
            markers: Vec::new(),
        });
    };

    let name = name.trim();
    if name.is_empty() {
        return Err(problem(
            line,
            format!("key 'author' has an entry with no name before its '^': '{entry}'"),
        ));
    }

    let markers = markers
        .split(',')
        .map(str::trim)
        .map(|marker| {
            match !marker.is_empty() && marker.bytes().all(|byte| byte.is_ascii_digit()) {
                true => Ok(marker.parse::<usize>().unwrap_or(usize::MAX)),
                false => Err(problem(
                    line,
                    format!("key 'author' takes a number after '^', not '{marker}'"),
                )),
            }
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(Author {
        name: name.to_string(),
        markers,
    })
}

fn problem(line: usize, problem: impl Into<String>) -> Error {
    Error::Frontmatter {
        location: Location::at(line),
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

    /// Every name a block writes, in order, with the markers dropped.
    fn names(out: &Frontmatter) -> Vec<&str> {
        out.author
            .iter()
            .flat_map(|authors| &authors.list)
            .map(|author| author.name.as_str())
            .collect()
    }

    #[test]
    fn quotes_comments_and_blank_lines_are_handled() {
        let block = "# a comment\ntitle: \"A Quoted Title\"\n\nauthor: 'Iva Po'\n";
        let out = parse(block, 2).unwrap();
        assert_eq!(out.title.as_deref(), Some("A Quoted Title"));
        assert_eq!(names(&out), ["Iva Po"]);
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
    fn both_figure_names_parse() {
        for (name, figures) in [("flat", Figures::Flat), ("sectioned", Figures::Sectioned)] {
            let out = parse(&format!("figures: {name}\n"), 2).unwrap();
            assert_eq!(out.figures, figures);
        }
    }

    /// A document that leaves the key out sections nothing.
    #[test]
    fn an_absent_figures_key_sections_nothing() {
        assert_eq!(parse("title: A\n", 2).unwrap().figures, Figures::Flat);
    }

    /// A name outside the set names the key and lists what it accepts, exactly
    /// as `template` and `equations` do. One mechanism, not three.
    #[test]
    fn a_figures_name_outside_the_set_lists_the_names_it_accepts() {
        match parse("figures: numbered\n", 2) {
            Err(Error::Frontmatter { problem, .. }) => {
                assert!(problem.contains("figures"), "problem was: {problem}");
                assert!(problem.contains("flat"), "problem was: {problem}");
                assert!(problem.contains("sectioned"), "problem was: {problem}");
            }
            other => panic!("expected a Frontmatter error, got {other:?}"),
        }
    }

    /// All seven names parse, the depths included.
    ///
    /// The depths are the whole reason this key departs from `equations`' and
    /// `figures`' two-name shape, so the loop names each one rather than
    /// sampling.
    #[test]
    fn every_headings_name_parses() {
        for (name, headings) in [
            ("plain", Headings::Plain),
            ("1", Headings::Depth1),
            ("2", Headings::Depth2),
            ("3", Headings::Depth3),
            ("4", Headings::Depth4),
            ("5", Headings::Depth5),
            ("6", Headings::Depth6),
        ] {
            let out = parse(&format!("headings: {name}\n"), 2).unwrap();
            assert_eq!(out.headings, headings, "for {name}");
        }
    }

    /// A document that leaves the key out numbers no heading.
    #[test]
    fn an_absent_headings_key_numbers_nothing() {
        assert_eq!(parse("title: A\n", 2).unwrap().headings, Headings::Plain);
    }

    /// A name outside the set names the key and lists what it accepts, exactly
    /// as `template`, `equations` and `figures` do. One mechanism, not four.
    ///
    /// Both bad values are deliberate. `numbered` is the guess an author who
    /// knows the other two numbering keys makes, and this schema has no synonym
    /// for `6`; `7` is the boundary of the depth, one past the six levels
    /// markdown has.
    #[test]
    fn a_headings_name_outside_the_set_lists_the_names_it_accepts() {
        for value in ["numbered", "7"] {
            match parse(&format!("headings: {value}\n"), 2) {
                Err(Error::Frontmatter { problem, .. }) => {
                    assert!(problem.contains("headings"), "problem was: {problem}");
                    assert!(problem.contains("plain"), "problem was: {problem}");
                    assert!(problem.contains("6"), "problem was: {problem}");
                }
                other => panic!("expected a Frontmatter error for {value}, got {other:?}"),
            }
        }
    }

    /// The one key that names a file keeps the line it was named on.
    #[test]
    fn the_bibliography_key_carries_its_path_and_its_line() {
        let out = parse("title: A\nbibliography: refs.yml\n", 2).unwrap();
        assert_eq!(
            out.bibliography,
            Some(BibliographyRef {
                path: "refs.yml".to_string(),
                location: Location::at(3),
            })
        );

        // An empty value means the key is absent, as it does for every other
        // string key.
        assert_eq!(parse("bibliography:\n", 2).unwrap().bibliography, None);
    }

    /// Every shape the path rule refuses names the key and says what it takes.
    ///
    /// The rule is `emit::portable_path`'s, shared with the image arm, so this
    /// asserts the phrasing a key gets rather than the rule itself: an author who
    /// wrote a URL needs to be told the key takes a path beside the document,
    /// where an author who wrote one on an image is told about the image.
    #[test]
    fn a_bibliography_path_outside_the_shape_rule_names_the_key() {
        for (value, needle) in [
            ("https://example.com/refs.yml", "not a URL"),
            ("/etc/refs.yml", "not an absolute path"),
            (
                "../refs.yml",
                "not a path that leaves the document's folder",
            ),
            ("refs\\bib.yml", "not a path with a backslash"),
        ] {
            match parse(&format!("bibliography: {value}\n"), 2) {
                Err(Error::Frontmatter { location, problem }) => {
                    assert_eq!(location.line, 2, "wrong line for {value}");
                    assert!(problem.contains("bibliography"), "problem was: {problem}");
                    assert!(problem.contains(needle), "problem was: {problem}");
                }
                other => panic!("expected a Frontmatter error for {value}, got {other:?}"),
            }
        }
    }

    /// A `..` that lands back inside the folder is a path beside the document.
    ///
    /// The key reads the same rule the image arm does, and that rule is about
    /// where a path *lands* rather than about the segments it is spelled with.
    /// `parse` holds no `Sources`, so a key's written path is where it lands and
    /// the whole of `portable_path` applies to it unchanged.
    #[test]
    fn a_bibliography_path_that_climbs_back_inside_is_accepted() {
        let out = parse("bibliography: figures/../refs.bib\n", 2).unwrap();
        assert_eq!(
            out.bibliography.map(|named| named.path).as_deref(),
            Some("figures/../refs.bib")
        );
    }

    #[test]
    fn the_date_is_kept_as_it_was_written() {
        let out = parse("date: 10 August 2026\n", 2).unwrap();
        assert_eq!(out.date.as_deref(), Some("10 August 2026"));
    }

    /// The list splits on `;`, every element is trimmed, and a name keeps its
    /// own commas.
    ///
    /// `Po, Iva` is the case the separator was chosen for: a comma is an
    /// ordinary way to write one person's name, and splitting on one would turn
    /// that person into two silently.
    #[test]
    fn the_author_list_splits_on_semicolons_and_trims() {
        for block in [
            "author: Po, Iva; Someone Else\n",
            "author:   Po, Iva  ;Someone Else  \n",
        ] {
            let out = parse(block, 2).unwrap();
            assert_eq!(names(&out), ["Po, Iva", "Someone Else"], "for {block}");
        }
    }

    /// A name splits from its markers at the first `^`, and the markers trim.
    ///
    /// `^1, 2` with the space is what an author actually types, and it is the
    /// same document as `^1,2`.
    #[test]
    fn markers_ride_the_name_and_index_from_one() {
        let block = "author: A^1; B^2; C^1, 2\naffiliation: One; Two\n";
        let out = parse(block, 2).unwrap();
        let markers: Vec<&[usize]> = out
            .author
            .as_ref()
            .unwrap()
            .list
            .iter()
            .map(|author| author.markers.as_slice())
            .collect();
        assert_eq!(markers, [&[1][..], &[2][..], &[1, 2][..]]);
    }

    /// At exactly one affiliation the markers are optional, and a document that
    /// writes none is valid.
    ///
    /// This is OQ-11's resolution, and it is what makes the commonest real paper
    /// — one lab, several authors — writable without a lone marker on every
    /// name. A marker written anyway is honoured, because the author wrote it.
    #[test]
    fn one_affiliation_makes_the_marker_optional() {
        for block in [
            "author: A; B\naffiliation: One Lab\n",
            "author: A^1; B^1\naffiliation: One Lab\n",
            "author: A^1; B\naffiliation: One Lab\n",
        ] {
            assert!(parse(block, 2).is_ok(), "refused: {block}");
        }
    }

    /// An author with no marker, under two affiliations, is deliberately not
    /// refused.
    ///
    /// Three authors from one lab and a fourth from none is a real document, and
    /// refusing it would break something nobody asked to have broken.
    #[test]
    fn an_unmarked_author_beside_marked_ones_is_accepted() {
        let block = "author: A^1; B^2; C\naffiliation: One; Two\n";
        assert!(parse(block, 2).is_ok());
    }

    /// Every refusal the two list keys add names the line the author would have
    /// to edit.
    ///
    /// The four are: a marker naming an affiliation the document does not carry;
    /// an `affiliation` key whose relation is unstated, which begins at *two*;
    /// a marker that is not a number; and an empty element, in **either** list.
    ///
    /// The line each names is the key the author would edit rather than the key
    /// the check happens to read, which is why the second names `affiliation`
    /// where the other three name the key the fault is written in. Both are
    /// asserted, on `errors_name_the_key_and_the_line`'s shape.
    #[test]
    fn every_affiliation_refusal_names_its_own_line() {
        // (block, the line the error names, a fragment of the sentence)
        for (block, line, needle) in [
            // A marker pointing past the affiliations the document carries, and
            // the `^0` that points before them.
            ("affiliation: One; Two\nauthor: A^1; B^3\n", 3, "carries 2"),
            ("author: A^1\n", 2, "names no 'affiliation'"),
            ("author: A^1\n", 2, "drop the markers"),
            (
                "affiliation: One; Two\nauthor: A^0; B^1\n",
                3,
                "affiliation 0",
            ),
            // Two affiliations and no marker anywhere: the relation is unstated.
            // The `affiliation` line is named, not the `author` line.
            (
                "author: A; B\naffiliation: One; Two\n",
                3,
                "no author points at one",
            ),
            (
                "affiliation: One; Two\nauthor: A; B\n",
                2,
                "no author points at one",
            ),
            // A marker that is not a number. `A^B^1` splits at the *first* `^`,
            // so what is refused is `B^1` rather than a guessed name `A^B`.
            ("author: A^x\n", 2, "not 'x'"),
            ("author: A^B^1\n", 2, "not 'B^1'"),
            ("author: A^\n", 2, "not ''"),
            ("author: A^1,\n", 2, "not ''"),
            // An empty element, in either list. A dangling separator would reach
            // the look as a nameless author or a blank affiliation a `^2` could
            // then point at.
            ("author: Iva Po;\n", 2, "key 'author' has an empty entry"),
            (
                "author: A^1\naffiliation: MIT;\n",
                3,
                "key 'affiliation' has an empty entry",
            ),
            ("author: ^1\n", 2, "no name before its '^'"),
        ] {
            match parse(block, 2) {
                Err(Error::Frontmatter { location, problem }) => {
                    assert_eq!(location.line, line, "wrong line for {block:?}");
                    assert!(problem.contains(needle), "problem was: {problem}");
                }
                other => panic!("expected a Frontmatter error for {block:?}, got {other:?}"),
            }
        }
    }

    #[test]
    fn errors_name_the_key_and_the_line() {
        for (block, needle) in [
            ("title: A\nsubtitle: B\n", "subtitle"),
            ("title: A\ncolumns: 3\n", "columns"),
            ("title: A\ntemplate: ieee\n", "press-release"),
            ("title: A\nequations: yes\n", "numbered"),
            ("title: A\nfigures: yes\n", "sectioned"),
            ("title: A\nheadings: yes\n", "plain or 1"),
            ("title: A\nbibliography: /refs.yml\n", "beside the document"),
            ("title: A\ntitle: B\n", "title"),
            ("title: A\njust a line\n", "key: value"),
            ("title: A\n  nested: B\n", "nested keys"),
        ] {
            match parse(block, 2) {
                Err(Error::Frontmatter { location, problem }) => {
                    assert_eq!(location.line, 3, "wrong line for {needle}");
                    assert!(problem.contains(needle), "problem was: {problem}");
                }
                other => panic!("expected a Frontmatter error, got {other:?}"),
            }
        }
    }
}
