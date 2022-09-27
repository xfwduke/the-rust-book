fn main() {
    let mut s = String::from("a");
    let r = "bar".repeat(3);
    s.push_str(&"".repeat(3));
    println!("{}", to_up(&s));
}

fn to_up(s: &str) -> &str {
    s.to_uppercase().as_str()
}

fn first_worlds(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}
