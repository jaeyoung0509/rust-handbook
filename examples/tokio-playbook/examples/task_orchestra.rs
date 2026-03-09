use tokio_playbook::{collect_streaming_messages, first_completed_label, run_job_fanout};

// #region tokio-main
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let jobs = run_job_fanout(vec!["borrow", "lifetime", "tokio"]).await;
    let messages = collect_streaming_messages(vec!["read", "compile"]).await;
    let winner = first_completed_label().await;

    println!("{jobs:?} / {messages:?} / {winner}");
}
// #endregion tokio-main
