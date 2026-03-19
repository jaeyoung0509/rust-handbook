use ownership_playbook::{publication_banner, state_message, PublishingState};

// #region enum-match-main
fn main() {
    let states = [
        PublishingState::Draft,
        PublishingState::InReview { reviewers: 0 },
        PublishingState::InReview { reviewers: 2 },
        PublishingState::Published,
    ];

    for state in states {
        println!("{}", state_message(state));
    }

    println!(
        "{}",
        publication_banner("ownership-backed fundamentals", PublishingState::Published)
    );
}
// #endregion enum-match-main
