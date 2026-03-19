use tokio_playbook::{
    cancel_slow_work, collect_streaming_messages, run_job_fanout, run_job_fanout_with_limit,
    run_stream_pipeline, run_stream_pipeline_until_shutdown,
};

#[tokio::test(flavor = "current_thread")]
async fn async_flow_stays_predictable_after_sorting() {
    let jobs = run_job_fanout(vec!["go", "python", "rust"]).await;
    let limited = run_job_fanout_with_limit(vec!["bounded", "queue", "rust"], 1).await;
    let messages = collect_streaming_messages(vec!["ci", "pages"]).await;
    let pipeline = run_stream_pipeline(vec!["ship", "drain"]).await;
    let shutdown = run_stream_pipeline_until_shutdown(vec!["ship", "drain", "flush"], 1).await;

    assert_eq!(jobs.len(), 3);
    assert_eq!(limited[0].message, "BOUNDED");
    assert_eq!(messages, vec!["done:ci", "done:pages"]);
    assert_eq!(pipeline, vec!["processed:0:ship", "processed:1:drain"]);
    assert_eq!(shutdown, vec!["processed:0:ship"]);
    assert_eq!(cancel_slow_work(1, 20).await, "completed");
    assert_eq!(cancel_slow_work(20, 1).await, "cancelled");
}
