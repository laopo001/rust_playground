#[derive(Debug, Copy, Clone)]
struct A {
    name: i32,
}
fn test() -> A {
    let a = A { name: 1 };
    println!("{:p}", &a.name);
    return a;
}

fn test2() -> A {
    let a = test();
    println!("{:p}", &a.name);
    return a;
}

fn main() {
    let a = test2();
    println!("{:p}", &a.name);
}
