use ownership_playbook::{append_tag, promote_title, sum_first_two};

// #region borrowed-slice-main
fn main() {
    let mut headline = promote_title("ownership");
    append_tag(&mut headline, "borrowed slices");

    let numbers = vec![10, 20, 30];
    let first_pair = sum_first_two(&numbers).expect("the sample has at least two items");

    println!("{headline} => {first_pair}");
}
// #endregion borrowed-slice-main
