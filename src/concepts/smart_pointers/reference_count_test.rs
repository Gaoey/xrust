#[cfg(test)]
mod reference_count_test {
    use crate::concepts::smart_pointers::reference_count::List::{Cons, Nil};
    use std::rc::Rc;

    #[test]
    fn test_my_box() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        assert_eq!(Rc::strong_count(&a), 1);
        let _b = Rc::new(Cons(3, Rc::clone(&a)));
        assert_eq!(Rc::strong_count(&a), 2);

        {
            let _c = Rc::new(Cons(4, Rc::clone(&a)));
            assert_eq!(Rc::strong_count(&a), 3);
        }

        assert_eq!(Rc::strong_count(&a), 2);
    }
}
