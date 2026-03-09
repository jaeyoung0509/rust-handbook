//! Ownership-focused examples used by the handbook.

// #region borrowed-slice
pub fn sum_first_two(values: &[i32]) -> Option<i32> {
    values
        .first()
        .zip(values.get(1))
        .map(|(left, right)| *left + *right)
}
// #endregion borrowed-slice

// #region promote-title
pub fn promote_title(title: &str) -> String {
    format!("{title}::deep-dive")
}
// #endregion promote-title

// #region mutable-borrow
pub fn append_tag(buffer: &mut String, tag: &str) {
    if !buffer.is_empty() {
        buffer.push_str(" | ");
    }

    buffer.push_str(tag);
}
// #endregion mutable-borrow

#[cfg(test)]
mod tests {
    use super::{append_tag, promote_title, sum_first_two};

    #[test]
    fn borrowed_slice_reads_without_taking_ownership() {
        assert_eq!(sum_first_two(&[3, 4, 8]), Some(7));
        assert_eq!(sum_first_two(&[3]), None);
    }

    #[test]
    fn mutable_borrow_updates_in_place() {
        let mut title = promote_title("ownership");
        append_tag(&mut title, "slices");
        append_tag(&mut title, "borrow checker");

        assert_eq!(title, "ownership::deep-dive | slices | borrow checker");
    }
}
