#[cfg(test)]
mod xderef_test {
    use crate::concepts::smart_pointers::xderef::MyBox;

    #[test]
    fn test_pointer() {
        // see more
        // https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/the-stack-and-the-heap.html
        /*
        +-----------+----------+------------+
        +  address  +   name   +   value    +
        +-----------+----------+------------+
          (20^30-1) |          |   ->0
        +-----------+----------+------------+
            ...         ....        ...
        +-----------+----------+------------+
             1      |    y     | ->(20^30-1)
        +-----------+----------+------------+
             0      |    x     |     5
        +-----------+----------+------------+
        */

        // In C Pointer
        // let x = 5;
        // let y = x;
        // assert_eq!(5, x);
        // assert_eq!(5, *y);

        let x = 5; // keep x value in stack
        let y = Box::new(x); // keep y value in heap
        assert_eq!(5, x);
        assert_eq!(5, *y);

        /*
        So if the stack is faster and easier to manage,
        why do we need the heap?
        A big reason is that Stack-allocation alone means you only have 'Last In First Out (LIFO)' semantics for reclaiming storage.
        Heap-allocation is strictly more general, allowing storage to be taken from and returned to the pool in arbitrary order,
        but at a complexity cost.

        Generally, you should prefer stack allocation, and so, Rust stack-allocates by default.
        The LIFO model of the stack is simpler, at a fundamental level.
        This has two big impacts: runtime efficiency and semantic impact.
        */
    }

    #[test]
    fn test_my_box() {
        /*
        if MyBox struct doesn't implement 'Deref' it's will shown error
        "type `MyBox<{integer}>` cannot be dereferences"
        */
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
}
