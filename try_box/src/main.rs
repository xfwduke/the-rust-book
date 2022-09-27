use std::fmt::{Display, Formatter};
use std::ops::Deref;

fn main() {
    let b = MyBox::new(String::from("world"));

    // &b 类型和 sya 的参数类型不匹配
    // 自动调用 b 的 deref 尝试匹配
    say(&b);
}
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        println!("here");
        &self.0
    }
}

impl <T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("drop it");
    }
}

impl<T: Display> Display for MyBox<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn say(s: &str) {
    println!("{}", s);
}
