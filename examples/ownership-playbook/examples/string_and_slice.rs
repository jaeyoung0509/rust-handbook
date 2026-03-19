use ownership_playbook::{describe_score_window, normalize_username};

// #region string-slice-main
fn main() {
    let username = normalize_username("  Alice  ");
    let empty = describe_score_window(&[]);
    let one = describe_score_window(&[12]);
    let many = describe_score_window(&[7, 9, 11]);

    println!("{username} | {empty} | {one} | {many}");
}
// #endregion string-slice-main
