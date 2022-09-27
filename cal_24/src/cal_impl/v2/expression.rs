use crate::v2::signature::signature;
use crate::v2::tree::Tree;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Expression<'a> {
    numbers: Vec<f64>,
    ops: Vec<&'a str>,
    pub value: f64,
    trans_numbers: Vec<f64>,
    trans_ops: Vec<&'a str>,
    tree: Tree<'a>,
    pub signature: String,
    pub string: String,
}

impl<'a> Display for Expression<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl<'a> Expression<'a> {
    pub fn new(numbers: Vec<f64>, ops: Vec<&str>) -> Expression {
        let mut expr = Expression {
            tree: Tree::empty(),
            numbers,
            ops,
            trans_ops: vec![],
            trans_numbers: vec![],
            value: 0.0,
            signature: "".to_string(),
            string: "".to_string(),
        };
        expr.eval();
        expr.tree = Tree::new(&expr.trans_numbers, &expr.trans_ops);
        expr.signature();
        expr
    }
    fn eval(&mut self) -> f64 {
        let mut number_iter = self.numbers.iter();
        self.value = *number_iter.next().unwrap();
        self.string = self.value.to_string();
        self.trans_numbers.push(self.value);
        for &op in self.ops.iter() {
            let right = *number_iter.next().unwrap();
            self.string = format!("({}{}{})", self.string, op, right);
            self.value = match op {
                "+" => {
                    self.trans_numbers.push(right);
                    self.trans_ops.push("+");
                    self.value + right
                }
                "-" => {
                    self.trans_numbers.push(-right);
                    self.trans_ops.push("+");
                    self.value - right
                }
                "*" => {
                    self.trans_numbers.push(right);
                    self.trans_ops.push("*");
                    self.value * right
                }
                _ => {
                    self.trans_numbers.push(1.0 / right);
                    self.trans_ops.push("*");
                    self.value / right
                }
            }
        }
        self.value
    }

    fn signature(&mut self) {
        self.signature = signature(&self.trans_numbers, &self.trans_ops)
    }
}
