#![allow(unused)]
use std::marker::PhantomData;
pub trait FnBox {
    fn call_box(&mut self);
}

impl<F: FnMut()> FnBox for F {
    fn call_box(&mut self) {
        (*self)()
    }
}

pub type Task<'a> = Box<dyn FnBox + 'a>;

struct B<'a> {
    cb: Option<Task<'a>>,
    // _m: &'a PhantomData<Task<'a>>,
}
impl<'a> B<'a> {
    fn new() -> Self {
        return B {
            cb: None,
            // _m: &PhantomData,
        };
    }
    fn sync(&mut self) {
        self.cb.as_mut().map(|cb| {
            (cb).call_box();
        });
    }
    fn set_cb(&mut self, cb: Task<'a>) {
        self.cb = Some(cb);
    }
}

struct A {
    i: i32,
}
fn main() {
    unsafe {
        let mut a = A { i: 1 };
        let ptr_a = &mut a as *mut A;
        let mut b = B::new();
        b.set_cb(Box::new(|| {
            a.i += 1;
            dbg!(a.i);
        }));
        b.sync();
        b.sync();
        dbg!((*ptr_a).i);
    }
}
