use shared_ownership_lab::{
    annotate_then_count, build_review_timeline, new_draft_log, summarize_release_steps,
};

#[test]
fn shared_ownership_examples_follow_the_book_flow() {
    let draft = new_draft_log("draft created");
    let count = annotate_then_count(&draft, "review requested");
    let timeline = build_review_timeline();
    let (owners, summary) = summarize_release_steps();

    assert_eq!(count, 2);
    assert_eq!(
        timeline,
        vec![
            "draft created".to_string(),
            "review requested".to_string(),
            "comments addressed".to_string(),
        ]
    );
    assert_eq!(owners, 1);
    assert_eq!(summary, "lint -> test -> deploy");
}
