fn main() {
    let mut s1 = String::from("hello");
    s1.push_str(", world");
    println!("{}", s1);

    // let s2 = s1;
    // println!("{}", s1);
    // let s2 = s1.clone();
    takes_ownership(s1);
    println!("after call: {}", s1);
}
fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}
