// #region borrow-conflict
fn main() {
    let mut title = String::from("ownership");
    let borrowed = &title;

    title.push_str(" handbook");
    println!("{borrowed}");
}
// #endregion borrow-conflict
