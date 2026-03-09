use api_contracts::{Note, display_badge, parse_label, render_summary};

// #region trait-main
fn main() {
    let note = Note {
        title: "Trait".to_string(),
        body: "describes capabilities".to_string(),
    };

    let rendered = render_summary(&note);
    let label = parse_label("  Deep Dive  ");
    let badge = display_badge(label);

    println!("{rendered} / {badge}");
}
// #endregion trait-main
