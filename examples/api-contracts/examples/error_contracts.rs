use api_contracts::{Note, find_note, parse_note_line, preview_note_line};

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
    let found = find_note(&notes, &parsed.title).expect("note must exist");
    let preview = preview_note_line(&notes, "Trait | contracts over inheritance")
        .expect("preview should succeed");

    println!("{parsed:?} / {found:?} / {preview}");
}
// #endregion error-main
