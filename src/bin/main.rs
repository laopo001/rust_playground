use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct A {}
impl A {
    pub fn set<'a>(&'a mut self) -> &'a str {
        return "123";
    }
}
pub struct Mesh {
    pub a: Option<Rc<RefCell<A>>>,
}

fn abc(a: &str) {}
fn test(q: &mut Mesh) {
    let mut s = q.a.as_ref().unwrap().borrow_mut();
    // let mut st= s.borrow_mut();
    let z = s.set();
    abc(z);
    // let s = q.a.as_mut().unwrap().borrow_mut().set();
    println!("{:?}", z);
}

fn main() {
    let mut m = Mesh {
        a: Some(Rc::new(RefCell::new(A {}))),
    };
    test(&mut m);
}
