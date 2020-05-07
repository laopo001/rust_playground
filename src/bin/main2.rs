use std::cell::RefCell;
use std::rc::Rc;
struct T<'a> {
    name: &'a Vec<i32>,
}
struct A<'a> {
    name: Vec<i32>,
    t: Option<T<'a>>,
}
impl<'a> A<'a> {
    pub fn set_t(&mut self) {
        unsafe {
            self.t = Some(T {
                name: std::mem::transmute(&self.name),
            });
        }
    }
}

struct B {
    pub i: Option<Rc<RefCell<Vec<i32>>>>,
}
impl B {
    pub fn new() -> B {
        return B { i: None };
    }
    pub fn set(&mut self, v: Vec<i32>) {
        self.i = Some(Rc::new(RefCell::new(v)));
    }
}
fn main() {
    let i = vec![32];
    let mut b = B::new();
    b.set(i);
    println!("{:?}", b.i);
    // let mut a = A { name: i, t: None };
    // a.set_t();
    // println!("Hello, {:?}!", a.name);
}
