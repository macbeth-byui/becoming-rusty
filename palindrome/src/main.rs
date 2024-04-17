pub trait Palindrome {
    fn is_palindrome(&self) -> bool;
}

impl Palindrome for String {
    fn is_palindrome(&self) -> bool {
        self.chars().eq(self.chars().rev())
    }
}

fn main() {
    let s1 = String::from("racecar");
    println!("Result = {}", s1.is_palindrome());
    let s2 = String::from("racecars");
    println!("Result = {}", s2.is_palindrome());
}
