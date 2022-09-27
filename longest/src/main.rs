fn main() {
    let s1 = String::from("long string is long");
    let r;
    {
        let s2 = String::from("abcdaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");

        r = longest(s1.as_str(), s2.as_str());
        println!("{}", r);
    }

    // 字面字符串具有 'static 生命周期
    // 存活于整个程序期间, 所以可以正常运行
    // let r2;
    // let s3 = "aaaaaaaaaaaaa";
    // {
    //     let s4="bbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    //     r2 = longest(s3, s4);
    // }
    // println!("{}", r2);
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        return s1;
    }
    return s2;
}
