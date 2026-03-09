// #region use-after-move
fn main() {
    let title = String::from("ownership");
    let moved = title;

    println!("{title} / {moved}");
}
// #endregion use-after-move
