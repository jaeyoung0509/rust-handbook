use lifetime_lab::{Excerpt, longest};

#[test]
fn first_line_and_longest_share_the_same_borrowed_source() {
    let source = String::from("async borrow\nsync fallbacks");
    let excerpt = Excerpt { source: &source };

    assert_eq!(longest(excerpt.first_line(), "tokio"), "async borrow");
}
