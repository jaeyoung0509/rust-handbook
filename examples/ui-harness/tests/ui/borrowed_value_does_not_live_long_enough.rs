// #region missing-lifetime
fn longest(left: &str, right: &str) -> &str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}
// #endregion missing-lifetime

fn main() {
    let _ = longest("borrow", "lifetime");
}
