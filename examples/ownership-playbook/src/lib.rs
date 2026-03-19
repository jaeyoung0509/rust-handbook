//! Ownership-focused examples used by the handbook.

// #region borrowed-slice
pub fn sum_first_two(values: &[i32]) -> Option<i32> {
    values
        .first()
        .zip(values.get(1))
        .map(|(left, right)| *left + *right)
}
// #endregion borrowed-slice

// #region promote-title
pub fn promote_title(title: &str) -> String {
    format!("{title}::deep-dive")
}
// #endregion promote-title

// #region normalize-username
pub fn normalize_username(input: &str) -> String {
    input.trim().to_lowercase()
}
// #endregion normalize-username

// #region describe-score-window
pub fn describe_score_window(scores: &[i32]) -> String {
    match scores {
        [] => "점수 없음".to_string(),
        [only] => format!("점수 1개: {only}"),
        [first, second, ..] => format!("앞의 두 점수: {first}, {second} (총 {}개)", scores.len()),
    }
}
// #endregion describe-score-window

// #region mutable-borrow
pub fn append_tag(buffer: &mut String, tag: &str) {
    if !buffer.is_empty() {
        buffer.push_str(" | ");
    }

    buffer.push_str(tag);
}
// #endregion mutable-borrow

// #region split-first-two
pub fn bump_first_two(scores: &mut [i32], delta: i32) -> Option<(i32, i32)> {
    let (first, rest) = scores.split_first_mut()?;
    let second = rest.first_mut()?;

    *first += delta;
    *second += delta;

    Some((*first, *second))
}
// #endregion split-first-two

// #region publishing-state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublishingState {
    Draft,
    InReview { reviewers: usize },
    Published,
}

pub fn state_message(state: PublishingState) -> String {
    match state {
        PublishingState::Draft => "초안".to_string(),
        PublishingState::InReview { reviewers } => format!("검토 중: 리뷰어 {reviewers}명"),
        PublishingState::Published => "배포 완료".to_string(),
    }
}
// #endregion publishing-state

// #region publication-banner
pub fn publication_banner(title: &str, state: PublishingState) -> String {
    match state {
        PublishingState::Draft => format!("{title}는 초안이다"),
        PublishingState::InReview { reviewers: 0 } => {
            format!("{title}는 아직 리뷰어가 배정되지 않았다")
        }
        PublishingState::InReview { reviewers } => {
            format!("{title}는 리뷰어 {reviewers}명과 함께 검토 중이다")
        }
        PublishingState::Published => format!("{title}는 이미 배포되었다"),
    }
}
// #endregion publication-banner

#[cfg(test)]
mod tests {
    use super::{
        append_tag, bump_first_two, describe_score_window, normalize_username, promote_title,
        publication_banner, state_message, sum_first_two, PublishingState,
    };

    #[test]
    fn borrowed_slice_reads_without_taking_ownership() {
        assert_eq!(sum_first_two(&[3, 4, 8]), Some(7));
        assert_eq!(sum_first_two(&[3]), None);
    }

    #[test]
    fn mutable_borrow_updates_in_place() {
        let mut title = promote_title("ownership");
        append_tag(&mut title, "slices");
        append_tag(&mut title, "borrow checker");

        assert_eq!(title, "ownership::deep-dive | slices | borrow checker");
    }

    #[test]
    fn borrow_splitting_updates_disjoint_elements_without_clone() {
        let mut scores = vec![3, 5, 8, 13];
        let updated = bump_first_two(&mut scores, 2);

        assert_eq!(updated, Some((5, 7)));
        assert_eq!(scores, vec![5, 7, 8, 13]);
    }

    #[test]
    fn string_and_slice_helpers_keep_borrows_clear() {
        assert_eq!(normalize_username("  Alice  "), "alice");
        assert_eq!(describe_score_window(&[]), "점수 없음");
        assert_eq!(describe_score_window(&[12]), "점수 1개: 12");
        assert_eq!(
            describe_score_window(&[7, 9, 11]),
            "앞의 두 점수: 7, 9 (총 3개)"
        );
    }

    #[test]
    fn enum_match_helpers_encode_state_explicitly() {
        assert_eq!(state_message(PublishingState::Draft), "초안");
        assert_eq!(
            state_message(PublishingState::InReview { reviewers: 2 }),
            "검토 중: 리뷰어 2명"
        );
        assert_eq!(state_message(PublishingState::Published), "배포 완료");
        assert_eq!(
            publication_banner("ownership", PublishingState::Draft),
            "ownership는 초안이다"
        );
        assert_eq!(
            publication_banner("ownership", PublishingState::InReview { reviewers: 0 }),
            "ownership는 아직 리뷰어가 배정되지 않았다"
        );
        assert_eq!(
            publication_banner("ownership", PublishingState::InReview { reviewers: 2 }),
            "ownership는 리뷰어 2명과 함께 검토 중이다"
        );
        assert_eq!(
            publication_banner("ownership", PublishingState::Published),
            "ownership는 이미 배포되었다"
        );
    }
}
