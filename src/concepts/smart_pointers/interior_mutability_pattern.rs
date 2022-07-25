/*
Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data
https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
// Don't recommend - borrow check at run time
Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:
- Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
- Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
- Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
*/

/*
Technique นี้เป็นการ interior mutability ใช้สำหรับการ อยากเปลี่ยนค่าตัวแปรแบบไม่ตรงไปตรงมาเช่น อยากเปลี่ยนตัวแปรทีหลัง แต่ทุกค่าที่ใช้ตัวแปรนี้อยู่จะได้รับค่าทีหลัง เรารู้ได้จากใน run time
library ที่เอา technique นี้ไปใช้คือ Cell, Mutex
*/

use std::cell::RefCell;
use std::rc::Rc;

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[derive(Debug)]
pub enum List<T> {
    Cons(Rc<RefCell<T>>, Rc<List<T>>),
    Nil,
}
