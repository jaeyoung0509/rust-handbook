//! Trait and generic examples used by the handbook.

use std::fmt::{Debug, Display};

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
    use super::{Note, Summary, display_badge, parse_label, render_summary};

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
}
