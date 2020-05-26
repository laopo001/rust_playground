#[derive(Debug)]
struct B {
    name: i32,
}
#[derive(Debug)]
struct A {
    b: Box<B>,
    p_b: Option<std::ptr::NonNull<B>>,
}
fn test() -> A {
    let b = B { name: 1 };
    let mut a = A {
        b: Box::new(b),
        p_b: None,
    };
    a.p_b = std::ptr::NonNull::new(a.b.as_mut() as *mut B);
    dbg!(&a);
    unsafe {
        dbg!(&*a.p_b.unwrap().as_ref());
    }
    return a;
}

fn main() {
    let a = test();
    dbg!(&a);
    unsafe {
        dbg!(&*a.p_b.unwrap().as_ref());
    }
}
