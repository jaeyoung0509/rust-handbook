use ownership_playbook::{append_tag, bump_first_two, promote_title, sum_first_two};

// #region borrowed-slice-main
fn main() {
    let mut headline = promote_title("ownership");
    append_tag(&mut headline, "borrowed slices");

    let numbers = vec![10, 20, 30];
    let first_pair = sum_first_two(&numbers).expect("the sample has at least two items");
    let mut scoreboard = vec![3, 5, 8, 13];
    let bumped = bump_first_two(&mut scoreboard, 2).expect("the sample has at least two items");

    println!("{headline} => {first_pair} => {scoreboard:?} => {bumped:?}");
}
// #endregion borrowed-slice-main
