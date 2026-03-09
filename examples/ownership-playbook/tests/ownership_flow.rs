use ownership_playbook::{append_tag, promote_title, sum_first_two};

#[test]
fn ownership_examples_follow_the_book_narrative() {
    let mut title = promote_title("why borrowing matters");
    append_tag(&mut title, "no hidden copies");

    assert_eq!(title, "why borrowing matters::deep-dive | no hidden copies");
    assert_eq!(sum_first_two(&[8, 13, 21]), Some(21));
}
