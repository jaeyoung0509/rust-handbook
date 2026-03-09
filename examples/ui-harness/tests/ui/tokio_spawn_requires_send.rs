use std::rc::Rc;

// #region non-send-spawn
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let state = Rc::new(String::from("shared state"));

    tokio::spawn(async move {
        println!("{state}");
    });
}
// #endregion non-send-spawn
