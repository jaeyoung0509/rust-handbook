use lifetime_lab::{Excerpt, highlight_keyword, longest};

// #region lifetime-main
fn main() {
    let article = String::from("borrow scopes stay local\nlifetimes describe relationships");
    let excerpt = Excerpt { source: &article };

    let summary = longest(excerpt.first_line(), "Rust");
    let keyword = highlight_keyword(&article, "lifetimes").unwrap_or("not found");

    println!("{summary} / {keyword}");
}
// #endregion lifetime-main
