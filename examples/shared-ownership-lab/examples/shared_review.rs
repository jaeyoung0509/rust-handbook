use shared_ownership_lab::{
    annotate_then_count, build_review_timeline, new_draft_log, summarize_release_steps,
};

// #region shared-review-main
fn main() {
    let draft = new_draft_log("draft created");
    let count = annotate_then_count(&draft, "review requested");
    let timeline = build_review_timeline();
    let (owners, summary) = summarize_release_steps();

    println!("{count} / {timeline:?} / {owners} / {summary}");
}
// #endregion shared-review-main
