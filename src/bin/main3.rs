use std::cell::RefCell;
use std::rc::Rc;
fn main() {
    let a = Rc::new(RefCell::new(123));
    let b = a.clone();
    *a.borrow_mut() = 2;
    println!("{:?}", b.borrow());
}
