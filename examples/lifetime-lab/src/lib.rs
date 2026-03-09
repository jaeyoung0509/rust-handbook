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

#[cfg(test)]
mod tests {
    use super::{Excerpt, highlight_keyword, longest};

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
}
