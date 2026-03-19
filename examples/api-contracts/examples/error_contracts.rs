use std::error::Error;

use api_contracts::{Note, find_note, parse_catalog_note, parse_note_line, preview_note_line};

// #region error-main
fn main() {
    let notes = vec![
        Note {
            title: "Trait".to_string(),
            body: "contracts over inheritance".to_string(),
        },
        Note {
            title: "Result".to_string(),
            body: "recoverable failures".to_string(),
        },
    ];

    let parsed = parse_note_line("Result | recoverable failures").expect("valid input");
    let parsed_with_context =
        parse_catalog_note("Trait | contract driven design").expect("valid catalog input");
    let found = find_note(&notes, &parsed.title).expect("note must exist");
    let preview = preview_note_line(&notes, "Trait | contracts over inheritance")
        .expect("preview should succeed");
    let layered_error = parse_catalog_note("broken input").expect_err("invalid input");

    println!("{parsed:?} / {parsed_with_context:?} / {found:?} / {preview}");
    if let Some(source) = layered_error.source() {
        println!("cause: {source}");
    }
}
// #endregion error-main
