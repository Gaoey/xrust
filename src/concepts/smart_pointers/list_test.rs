#[cfg(test)]
mod list_test {
    use crate::concepts::smart_pointers::list::List::{Cons, Nil};

    #[test]
    fn test_list() {
        /*
        box is pointer
        see more: The Stack and the Heap
        https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/the-stack-and-the-heap.html
        */
        let correct_list = Cons(2, Box::new(Cons(3, Box::new(Nil))));
        println!("{:?}", correct_list);
    }

    #[ignore]
    #[test]
    fn test_failed_list() {
        /*
        error shown: "recursive without indirection"
        indirection mean rust compiler doesn't know exactly memory size of FailedList
        direction mean rust compiler know exactly which memory size of particular parameter
        */
        // #[derive(Debug)]
        // pub enum FailedList<T> {
        //     FCons(T, FailedList<T>),
        //     FNil,
        // }
    }
}
