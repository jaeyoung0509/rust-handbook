use tokio::time::{sleep, Duration};

// #region borrowed-local-spawn
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let label = String::from("shutdown");

    tokio::spawn(async {
        sleep(Duration::from_millis(1)).await;
        println!("{label}");
    });
}
// #endregion borrowed-local-spawn
