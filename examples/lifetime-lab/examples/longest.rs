use lifetime_lab::{Excerpt, highlight_keyword, longest, split_note, summarize_note};

// #region lifetime-main
fn main() {
    let article = String::from("borrow scopes stay local\nlifetimes describe relationships");
    let excerpt = Excerpt { source: &article };
    let note = "Design review\n\nborrowed data stays close";

    let summary = longest(excerpt.first_line(), "Rust");
    let keyword = highlight_keyword(&article, "lifetimes").unwrap_or("not found");
    let parsed = split_note(note).expect("note should split into title/body");
    let title = summarize_note(note).unwrap_or("not found");

    println!("{summary} / {keyword} / {} / {title}", parsed.body);
}
// #endregion lifetime-main
