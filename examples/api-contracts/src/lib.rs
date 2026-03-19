//! Trait and generic examples used by the handbook.

use std::error::Error;
use std::fmt::{self, Debug, Display};

// #region summary-trait
pub trait Summary {
    fn summary(&self) -> String;
}
// #endregion summary-trait

// #region note-struct
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Note {
    pub title: String,
    pub body: String,
}

impl Summary for Note {
    fn summary(&self) -> String {
        format!("{}: {}", self.title, self.body)
    }
}
// #endregion note-struct

// #region note-parse-error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoteParseError {
    MissingSeparator,
    EmptyTitle,
    EmptyBody,
}

impl Display for NoteParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSeparator => write!(f, "missing `|` separator"),
            Self::EmptyTitle => write!(f, "title is empty"),
            Self::EmptyBody => write!(f, "body is empty"),
        }
    }
}

impl Error for NoteParseError {}
// #endregion note-parse-error

// #region catalog-error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogError {
    Parse {
        input: String,
        source: NoteParseError,
    },
    NoteNotFound {
        title: String,
    },
}

impl CatalogError {
    pub fn parse(input: impl Into<String>, source: NoteParseError) -> Self {
        Self::Parse {
            input: input.into(),
            source,
        }
    }
}

impl From<(String, NoteParseError)> for CatalogError {
    fn from((input, source): (String, NoteParseError)) -> Self {
        Self::Parse { input, source }
    }
}

impl Display for CatalogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse { input, source } => {
                write!(f, "parse error in `{input}`: {source}")
            }
            Self::NoteNotFound { title } => write!(f, "note not found: {title}"),
        }
    }
}

impl Error for CatalogError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Parse { source, .. } => Some(source),
            Self::NoteNotFound { .. } => None,
        }
    }
}
// #endregion catalog-error

// #region find-note
pub fn find_note<'a>(notes: &'a [Note], title: &str) -> Option<&'a Note> {
    notes.iter().find(|note| note.title == title)
}
// #endregion find-note

// #region parse-note-line
pub fn parse_note_line(line: &str) -> Result<Note, NoteParseError> {
    let (title, body) = line
        .split_once('|')
        .ok_or(NoteParseError::MissingSeparator)?;

    let title = title.trim();
    if title.is_empty() {
        return Err(NoteParseError::EmptyTitle);
    }

    let body = body.trim();
    if body.is_empty() {
        return Err(NoteParseError::EmptyBody);
    }

    Ok(Note {
        title: title.to_string(),
        body: body.to_string(),
    })
}
// #endregion parse-note-line

// #region catalog-parse-note-line
pub fn parse_catalog_note(line: &str) -> Result<Note, CatalogError> {
    parse_note_line(line).map_err(|error| (line.to_string(), error).into())
}
// #endregion catalog-parse-note-line

// #region preview-note-line
pub fn preview_note_line(notes: &[Note], line: &str) -> Result<String, CatalogError> {
    let parsed = parse_catalog_note(line)?;
    let note = find_note(notes, &parsed.title).ok_or_else(|| CatalogError::NoteNotFound {
        title: parsed.title.clone(),
    })?;

    Ok(render_summary(note))
}
// #endregion preview-note-line

// #region render-summary
pub fn render_summary<T>(value: &T) -> String
where
    T: Summary + Debug,
{
    format!("{value:?} => {}", value.summary())
}
// #endregion render-summary

// #region parse-label
pub fn parse_label<T>(input: T) -> String
where
    T: AsRef<str>,
{
    input.as_ref().trim().to_lowercase()
}
// #endregion parse-label

// #region display-badge
pub fn display_badge<T>(value: T) -> String
where
    T: Display,
{
    format!("[{value}]")
}
// #endregion display-badge

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::{
        CatalogError, Note, NoteParseError, Summary, display_badge, find_note, parse_catalog_note,
        parse_label, parse_note_line, preview_note_line, render_summary,
    };

    #[test]
    fn trait_bounds_make_capabilities_explicit() {
        let note = Note {
            title: "Trait".to_string(),
            body: "contracts over inheritance".to_string(),
        };

        assert_eq!(note.summary(), "Trait: contracts over inheritance");
        assert!(render_summary(&note).contains("Trait: contracts over inheritance"));
        assert_eq!(parse_label("  Rust Handbook  "), "rust handbook");
        assert_eq!(display_badge(42), "[42]");
    }

    #[test]
    fn option_and_result_flow_distinguish_absence_from_failure() {
        let notes = vec![
            Note {
                title: "Trait".to_string(),
                body: "contracts over inheritance".to_string(),
            },
            Note {
                title: "Result".to_string(),
                body: "recoverable failure".to_string(),
            },
        ];

        assert!(find_note(&notes, "missing").is_none());
        assert_eq!(
            parse_catalog_note("Option | absence")
                .expect("valid note")
                .title,
            "Option"
        );
        assert_eq!(
            preview_note_line(&notes, "Trait | contracts over inheritance")
                .expect("preview should succeed"),
            r#"Note { title: "Trait", body: "contracts over inheritance" } => Trait: contracts over inheritance"#
        );

        let err = parse_note_line("broken input").expect_err("missing separator");
        assert_eq!(err, NoteParseError::MissingSeparator);

        let lookup_err = preview_note_line(&notes, "Missing | note").expect_err("missing note");
        assert_eq!(
            lookup_err,
            CatalogError::NoteNotFound {
                title: "Missing".to_string()
            }
        );
        let layered_err = parse_catalog_note("broken input").expect_err("missing separator");
        assert_eq!(
            layered_err,
            CatalogError::Parse {
                input: "broken input".to_string(),
                source: NoteParseError::MissingSeparator,
            }
        );
        assert_eq!(
            layered_err.source().map(ToString::to_string),
            Some("missing `|` separator".to_string())
        );

        let preview_parse_err =
            preview_note_line(&notes, "broken input").expect_err("missing separator");
        assert_eq!(
            preview_parse_err,
            CatalogError::Parse {
                input: "broken input".to_string(),
                source: NoteParseError::MissingSeparator,
            }
        );
        assert_eq!(
            preview_parse_err.source().map(ToString::to_string),
            Some("missing `|` separator".to_string())
        );

        let preview_err = preview_note_line(&notes, "Missing | note").expect_err("missing note");
        assert_eq!(
            preview_err.to_string(),
            "note not found: Missing".to_string()
        );
    }
}
