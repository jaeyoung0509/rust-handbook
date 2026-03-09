use tokio_playbook::{collect_streaming_messages, run_job_fanout};

#[tokio::test(flavor = "current_thread")]
async fn async_flow_stays_predictable_after_sorting() {
    let jobs = run_job_fanout(vec!["go", "python", "rust"]).await;
    let messages = collect_streaming_messages(vec!["ci", "pages"]).await;

    assert_eq!(jobs.len(), 3);
    assert_eq!(messages, vec!["done:ci", "done:pages"]);
}
