#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(clippy::all)]
#[allow(unused_mut)]

struct Foo {
    x : i32,
    y : i32
}

fn main2() {
    let s1 = "hello";
    let s2 = s1; // Attempting to copy s1 to s2, but s1 is of type &str which does not implement Copy
    println!("s1: {}, s2: {}", s1, s2); // Error: value borrowed here after move
    let s3 = &s1[2..4];
}

fn main() {
    /*
    If a block needs to use a variable (non native types)
       Option 1 - Clone it
       Option 2 - Pass it to a function or closure via reference
       Option 3 - Use it as a reference if supported (including string slice &str)

    Why??
        1) Allow for reclaiming of memory
        2) Prevent references to memory that doesn't exist
        3) Concurrency Protection
            a) Only one mutable reference at a time.
            b) No borrowing of mutable and immutable reference at the same time
            c) Multiple immutable references allowed

    I know when memory is no longer needed (goes out of scope)
    I know who is in charge of memory

    Don't assign a variable equal to something within a different scope unless:
    1) You passed it by reference to that scope
    2) Or, You want that scope to consume it

    
     */
    let a = String::from("Hello");
    let mut b = String::from("hello");
    let aa = &a[2..5];
    let s = "rust";
    let sss = s;
    println!("{}", s);
    {
        let ss = s;
        let aaa = &s[2..3];
    }

    {
        let k = &a;
        let kk = &a;
        let pp = &a;
        // let z = &b;
        // let zz = &b;
        let mut zzz = &mut b;
        let mut zzzz = &mut b;
        // println!("{} {}",zzz,zzzz);
    }
    println!("{}", b);

    let mut sum = Foo {x : 0, y :0};
    let aaaa = Foo {x:10, y:20};
    for i in 1..5 {
        let a = aaaa.x;
        sum.x += a;
        sum.y += aaaa.y;
    }
    // sum = &aaaa;
    println!("{} {}",sum.x,aaaa.x);
}
