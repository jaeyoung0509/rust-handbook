use ownership_playbook::{state_message, PublishingState};

// #region enum-match-main
fn main() {
    let states = [
        PublishingState::Draft,
        PublishingState::InReview { reviewers: 2 },
        PublishingState::Published,
    ];

    for state in states {
        println!("{}", state_message(state));
    }
}
// #endregion enum-match-main
