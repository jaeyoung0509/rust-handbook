use ownership_playbook::bump_first_two;

// #region split-mutation-main
fn main() {
    let mut scores = vec![11, 13, 17, 19];
    let updated = bump_first_two(&mut scores, 3).expect("the sample has at least two items");

    println!("{scores:?} => {updated:?}");
}
// #endregion split-mutation-main
