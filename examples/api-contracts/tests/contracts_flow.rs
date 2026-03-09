use api_contracts::{Note, display_badge, parse_label, render_summary};

#[test]
fn contract_examples_match_the_book_flow() {
    let note = Note {
        title: "Debug".to_string(),
        body: "lets the formatter inspect state".to_string(),
    };

    assert!(render_summary(&note).contains("Debug"));
    assert_eq!(parse_label("  Trait Bounds "), "trait bounds");
    assert_eq!(display_badge("Send"), "[Send]");
}
