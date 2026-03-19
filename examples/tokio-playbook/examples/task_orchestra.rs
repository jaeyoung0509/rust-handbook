use tokio_playbook::{
    cancel_slow_work, collect_streaming_messages, first_completed_label, run_job_fanout,
    run_job_fanout_with_limit,
};

// #region tokio-main
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let jobs = run_job_fanout(vec!["borrow", "lifetime", "tokio"]).await;
    let limited = run_job_fanout_with_limit(vec!["borrow", "lifetime", "tokio"], 1).await;
    let messages = collect_streaming_messages(vec!["read", "compile"]).await;
    let winner = first_completed_label().await;
    let cancelled = cancel_slow_work(20, 1).await;

    println!("{jobs:?} / {limited:?} / {messages:?} / {winner} / {cancelled}");
}
// #endregion tokio-main
