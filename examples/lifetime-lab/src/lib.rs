//! Lifetime-oriented examples used by the handbook.

// #region longest-signature
pub fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}
// #endregion longest-signature

// #region borrowed-note
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BorrowedNote<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

pub fn split_note<'a>(source: &'a str) -> Option<BorrowedNote<'a>> {
    let (title, body) = source.split_once("\n\n")?;
    Some(BorrowedNote { title, body })
}
// #endregion borrowed-note

// #region excerpt-struct
#[derive(Debug, Clone, Copy)]
pub struct Excerpt<'a> {
    pub source: &'a str,
}
// #endregion excerpt-struct

impl<'a> Excerpt<'a> {
    // #region excerpt-method
    pub fn first_line(self) -> &'a str {
        self.source.lines().next().unwrap_or(self.source)
    }
    // #endregion excerpt-method
}

// #region keyword-search
pub fn highlight_keyword<'a>(source: &'a str, keyword: &str) -> Option<&'a str> {
    source.lines().find(|line| line.contains(keyword))
}
// #endregion keyword-search

// #region zero-copy-summary
pub fn summarize_note<'a>(source: &'a str) -> Option<&'a str> {
    split_note(source).map(|note| note.title)
}
// #endregion zero-copy-summary

#[cfg(test)]
mod tests {
    use super::{BorrowedNote, Excerpt, highlight_keyword, longest, split_note, summarize_note};

    #[test]
    fn lifetime_signature_returns_the_longer_borrow() {
        assert_eq!(longest("borrow", "ownership"), "ownership");
    }

    #[test]
    fn struct_keeps_reference_relationship_explicit() {
        let article = String::from("line one\nline two");
        let excerpt = Excerpt { source: &article };

        assert_eq!(excerpt.first_line(), "line one");
        assert_eq!(highlight_keyword(&article, "two"), Some("line two"));
    }

    #[test]
    fn borrowed_note_keeps_zero_copy_boundaries_visible() {
        let note = String::from("Design review\n\nborrowed data stays close");
        let parsed = split_note(&note).expect("note should split into title/body");

        assert_eq!(
            parsed,
            BorrowedNote {
                title: "Design review",
                body: "borrowed data stays close",
            }
        );
        assert_eq!(summarize_note(&note), Some("Design review"));
    }
}
