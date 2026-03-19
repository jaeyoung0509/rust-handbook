use ownership_playbook::{
    append_tag, bump_first_two, describe_score_window, normalize_username, promote_title,
    publication_banner, state_message, sum_first_two, PublishingState,
};

#[test]
fn ownership_examples_follow_the_book_narrative() {
    let mut title = promote_title("why borrowing matters");
    append_tag(&mut title, "no hidden copies");
    let mut scores = vec![5, 8, 13];
    let bumped = bump_first_two(&mut scores, 1);

    assert_eq!(title, "why borrowing matters::deep-dive | no hidden copies");
    assert_eq!(sum_first_two(&[8, 13, 21]), Some(21));
    assert_eq!(bumped, Some((6, 9)));
    assert_eq!(scores, vec![6, 9, 13]);
    assert_eq!(normalize_username("  Rust  "), "rust");
    assert_eq!(describe_score_window(&[3, 5, 8]), "앞의 두 점수: 3, 5 (총 3개)");
    assert_eq!(
        state_message(PublishingState::InReview { reviewers: 1 }),
        "검토 중: 리뷰어 1명"
    );
    assert_eq!(
        publication_banner("ownership", PublishingState::Published),
        "ownership는 이미 배포되었다"
    );
}
