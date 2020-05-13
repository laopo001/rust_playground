use std::cell::RefCell;

fn main() {
    let x = RefCell::new(5u8);
    assert_eq!(5, *x.borrow());
    {
        let mut y = x.borrow_mut();
        *y = 10;
        assert_eq!(10, *x.borrow());
        let z = x.borrow();     //编译时会通过，但运行时panic!
    }
}

