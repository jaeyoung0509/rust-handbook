//! Shared ownership and interior mutability examples used by the handbook.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

pub type DraftLog = Rc<RefCell<Vec<String>>>;

// #region draft-log
pub fn new_draft_log(seed: &str) -> DraftLog {
    Rc::new(RefCell::new(vec![seed.to_string()]))
}

pub fn append_revision(log: &DraftLog, note: &str) {
    log.borrow_mut().push(note.to_string());
}

pub fn snapshot(log: &DraftLog) -> Vec<String> {
    log.borrow().clone()
}
// #endregion draft-log

// #region shared-review
pub fn build_review_timeline() -> Vec<String> {
    let draft = new_draft_log("draft created");
    let reviewer = draft.clone();
    let editor = draft.clone();

    append_revision(&editor, "review requested");
    append_revision(&reviewer, "comments addressed");

    snapshot(&draft)
}
// #endregion shared-review

// #region scoped-borrow
pub fn annotate_then_count(log: &DraftLog, note: &str) -> usize {
    {
        let mut entries = log.borrow_mut();
        entries.push(note.to_string());
    }

    log.borrow().len()
}
// #endregion scoped-borrow

// #region arc-thread-share
pub fn summarize_release_steps() -> (usize, String) {
    let steps = Arc::new(vec![
        "lint".to_string(),
        "test".to_string(),
        "deploy".to_string(),
    ]);
    let worker = steps.clone();

    let summary = thread::spawn(move || worker.join(" -> "))
        .join()
        .expect("worker thread should finish");

    (Arc::strong_count(&steps), summary)
}
// #endregion arc-thread-share

#[cfg(test)]
mod tests {
    use super::{
        annotate_then_count, build_review_timeline, new_draft_log, snapshot,
        summarize_release_steps,
    };

    #[test]
    fn rc_refcell_keeps_shared_edits_visible() {
        let timeline = build_review_timeline();

        assert_eq!(
            timeline,
            vec![
                "draft created".to_string(),
                "review requested".to_string(),
                "comments addressed".to_string(),
            ]
        );
    }

    #[test]
    fn mutable_borrow_scope_can_close_before_reading_again() {
        let draft = new_draft_log("draft created");

        assert_eq!(annotate_then_count(&draft, "review requested"), 2);
        assert_eq!(
            snapshot(&draft),
            vec!["draft created".to_string(), "review requested".to_string()]
        );
    }

    #[test]
    fn arc_can_share_read_only_state_across_threads() {
        let (owners, summary) = summarize_release_steps();

        assert_eq!(owners, 1);
        assert_eq!(summary, "lint -> test -> deploy");
    }
}
