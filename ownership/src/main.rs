#[derive(Debug)]
struct Foo {
    x : i32,
}

fn take(a : Foo) {
    println!("{:?}",a);
}

// fn borrow(a : &Foo) {
//     println!("{:?}",a);
// }

fn main() {
    let m = Foo { x: 10 };
    take(m);
    // borrow(&m);
}
