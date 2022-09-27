#[derive(Debug)]
pub struct Number {
    value: f64,
    numbers: Vec<f64>,
}

impl Number {
    pub fn append(&mut self, number: f64) {
        self.numbers.push(number);
        self.numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.value *= number;
    }
    pub fn new(number: f64) -> Self {
        Number {
            value: number,
            numbers: vec![number],
        }
    }
    pub fn signature(&self) -> String {
        self.numbers
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("*")
    }
}

pub fn signature(numbers: &[f64], ops: &[&str]) -> String {
    let mut numbers_iter = numbers.iter();
    let mut ops_iter = ops.iter();

    let mut left = Number::new(*numbers_iter.next().unwrap());
    let mut right = Number::new(*numbers_iter.next().unwrap());
    let op = ops_iter.next().unwrap();

    let mut flatten_numbers = vec![];
    let mut flatten_ops = vec![];

    if *op == "+" {
        flatten_ops.push(*op);
        flatten_numbers.push(left);
        flatten_numbers.push(right);
    } else {
        left.append(right.value);
        flatten_numbers.push(left);
    }

    for op in ops_iter {
        let number = numbers_iter.next().unwrap();
        if *op == "+" {
            flatten_ops.push(*op);
            flatten_numbers.push(Number::new(*number));
        } else {
            for n in flatten_numbers.iter_mut() {
                n.append(*number);
            }
        }
    }

    flatten_numbers.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());

    let mut flatten_numbers_iter = flatten_numbers.iter();
    let mut flatten_ops_iter = flatten_ops.iter();
    let mut signature = flatten_numbers_iter.next().unwrap().signature();
    for op in flatten_ops.iter() {
        signature = format!(
            "{}{}{}",
            signature,
            *op,
            flatten_numbers_iter.next().unwrap().signature()
        )
    }
    signature
}
