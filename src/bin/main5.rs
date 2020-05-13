#[derive(Debug)]
struct A {
  name:i32
}
#[derive(Debug)]
struct B {
  a:A
}
impl B {
  fn new(mut a:A) -> B{
    a.name =123;
    return B{
      a
    };
  }
}
fn run( a:Box<A>){
   dbg!(&a as *const A);
  a.name =123;
  dbg!(a.name);
}

fn main() {
  let  a = Box::new(A{name:0});
   dbg!(&a as *const A);
  run(a);
 
  // let a_p =&a as *const A;
  // let b = B::new(a);
  // unsafe {
  //  dbg!((*a_p).name);
  // }
  // dbg!(b);
}

