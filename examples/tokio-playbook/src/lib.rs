//! Async and Tokio-oriented examples used by the handbook.

use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tokio::time::{Duration, sleep};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JobResult {
    pub id: usize,
    pub message: String,
}

// #region fanout-function
pub async fn run_job_fanout(inputs: Vec<&str>) -> Vec<JobResult> {
    let mut tasks = JoinSet::new();

    for (id, input) in inputs.into_iter().enumerate() {
        let owned = input.to_string();

        tasks.spawn(async move {
            sleep(Duration::from_millis(10 * (id as u64 + 1))).await;
            JobResult {
                id,
                message: owned.to_uppercase(),
            }
        });
    }

    let mut results = Vec::new();
    while let Some(joined) = tasks.join_next().await {
        results.push(joined.expect("example tasks should finish successfully"));
    }

    results.sort_by_key(|job| job.id);
    results
}
// #endregion fanout-function

// #region bounded-fanout
pub async fn run_job_fanout_with_limit(inputs: Vec<&str>, limit: usize) -> Vec<JobResult> {
    let permits = Arc::new(Semaphore::new(limit.max(1)));
    let mut tasks = JoinSet::new();

    for (id, input) in inputs.into_iter().enumerate() {
        let permit_pool = permits.clone();
        let owned = input.to_string();

        tasks.spawn(async move {
            let _permit = permit_pool
                .acquire_owned()
                .await
                .expect("semaphore should stay open");

            sleep(Duration::from_millis(10 * (id as u64 + 1))).await;
            JobResult {
                id,
                message: owned.to_uppercase(),
            }
        });
    }

    let mut results = Vec::new();
    while let Some(joined) = tasks.join_next().await {
        results.push(joined.expect("example tasks should finish successfully"));
    }

    results.sort_by_key(|job| job.id);
    results
}
// #endregion bounded-fanout

// #region channel-function
pub async fn collect_streaming_messages(inputs: Vec<&str>) -> Vec<String> {
    let (tx, mut rx) = mpsc::channel(inputs.len().max(1));
    let mut tasks = JoinSet::new();

    for input in inputs {
        let sender = tx.clone();
        let owned = input.to_string();

        tasks.spawn(async move {
            sender
                .send(format!("done:{owned}"))
                .await
                .expect("receiver should stay alive while tasks are running");
        });
    }

    drop(tx);

    while tasks.join_next().await.is_some() {}

    let mut messages = Vec::new();
    while let Some(message) = rx.recv().await {
        messages.push(message);
    }

    messages.sort();
    messages
}
// #endregion channel-function

// #region select-loop
pub async fn first_completed_label() -> &'static str {
    let fast = sleep(Duration::from_millis(5));
    let slow = sleep(Duration::from_millis(50));
    tokio::pin!(fast);
    tokio::pin!(slow);

    tokio::select! {
        _ = &mut fast => "fast path",
        _ = &mut slow => "slow path",
    }
}
// #endregion select-loop

// #region cancel-slow-work
pub async fn cancel_slow_work(work_ms: u64, timeout_ms: u64) -> &'static str {
    let slow = sleep(Duration::from_millis(work_ms));
    tokio::pin!(slow);

    tokio::select! {
        _ = &mut slow => "completed",
        _ = sleep(Duration::from_millis(timeout_ms)) => "cancelled",
    }
}
// #endregion cancel-slow-work

#[cfg(test)]
mod tests {
    use super::{
        cancel_slow_work, collect_streaming_messages, first_completed_label, run_job_fanout,
        run_job_fanout_with_limit,
    };

    #[tokio::test(flavor = "current_thread")]
    async fn tokio_examples_cover_fanout_channels_and_select() {
        let jobs = run_job_fanout(vec!["borrow", "lifetime"]).await;
        let limited = run_job_fanout_with_limit(vec!["bounded", "queue", "tokio"], 1).await;
        let messages = collect_streaming_messages(vec!["lint", "build"]).await;

        assert_eq!(jobs[0].message, "BORROW");
        assert_eq!(limited[1].message, "QUEUE");
        assert_eq!(messages, vec!["done:build", "done:lint"]);
        assert_eq!(first_completed_label().await, "fast path");
        assert_eq!(cancel_slow_work(1, 10).await, "completed");
        assert_eq!(cancel_slow_work(20, 1).await, "cancelled");
    }
}
