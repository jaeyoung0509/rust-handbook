use std::rc::Rc;

// #region rc-mutation-without-refcell
fn main() {
    let draft = Rc::new(vec![String::from("draft created")]);

    draft.push(String::from("review requested"));
}
// #endregion rc-mutation-without-refcell
