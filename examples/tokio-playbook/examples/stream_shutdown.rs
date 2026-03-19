use tokio_playbook::{run_stream_pipeline, run_stream_pipeline_until_shutdown};

// #region stream-shutdown-main
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let pipeline = run_stream_pipeline(vec!["ingest", "transform", "flush"]).await;
    let shutdown =
        run_stream_pipeline_until_shutdown(vec!["ingest", "transform", "flush"], 1).await;

    println!("{pipeline:?} / {shutdown:?}");
}
// #endregion stream-shutdown-main
