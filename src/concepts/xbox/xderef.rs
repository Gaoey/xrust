use std::ops::Deref;

#[derive(Debug)]
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub(crate) fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: std::fmt::Debug> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // println!("{:?}", self);
        &self.0
    }
}
