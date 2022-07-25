use std::rc::Rc;

/*
see more
https://doc.rust-lang.org/book/ch15-04-rc.html
reference_count (RC) = handle multiple owner
*/
pub enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}
