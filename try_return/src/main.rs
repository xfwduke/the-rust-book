enum Action {
    Upper,
    Append(usize),
}
fn main() {
    let mut s = String::from("aa");

    println!("{}", foo(&Action::Upper, &mut s));
    println!("{}", foo(&Action::Append(3), &mut s));
}

fn foo<'a>(act: &Action, s: &'a mut String) -> &'a mut str {
    match act {
        Action::Upper => {
            *s = s.to_uppercase();
            s
        }
        Action::Append(u) => {
            s.push_str(&"bar".repeat(*u));
            s
        }
    }
}
