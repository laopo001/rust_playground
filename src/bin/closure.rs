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

pub type Task<'a> = Box<dyn FnBox>;

struct B<'a, T = Task<'a>> {
    cb: Option<T>,
    _m: &'a PhantomData<T>,
}
impl<'a> B<'a> {
    fn new() -> Self {
        return B {
            cb: None,
            _m: &PhantomData,
        };
    }
    fn sync(&mut self) {
        self.cb.as_mut().map(|cb| {
            (cb).call_box();
        });
    }
    fn set_cb(&mut self, cb: Task) {
        self.cb = Some(cb);
    }
}

fn main() {
    let mut b = B::new();
    let mut a = 1;
    b.set_cb(Box::new(|| {
        a += 1;
    }));
    b.sync();
}
