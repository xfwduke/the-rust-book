fn main() {
    // 不可变变量不能赋值 2 次
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in inner scope is: {}", y);
    }
    println!("The value of y is: {}", y);
}
