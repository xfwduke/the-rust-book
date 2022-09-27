use std::borrow::Borrow;
use std::cell::RefCell;

#[derive(Debug)]
enum Term<'a> {
    Number(f64),
    SubTree(Tree<'a>),
}

#[derive(Debug)]
pub struct Tree<'a> {
    left: Box<Term<'a>>,
    op: &'a str,
    prior: i32,
    right: f64,
}

impl<'a> Tree<'a> {
    pub fn empty() -> Self {
        Tree {
            left: Box::new(Term::Number(0.0)),
            op: "",
            prior: 0,
            right: 0.0,
        }
    }
    pub fn new(numbers: &[f64], ops: &[&'a str]) -> Self {
        let (current_op, left_ops) = ops.split_last().unwrap();
        let (current_right, left_numbers) = numbers.split_last().unwrap();
        let mut prior = 0;
        if *current_op == "*" {
            prior = 1;
        }

        if left_ops.is_empty() {
            let (current_left, _) = left_numbers.split_last().unwrap();
            Tree {
                left: Box::new(Term::Number(*current_left)),
                op: current_op,
                prior,
                right: *current_right,
            }
        } else {
            Tree {
                left: Box::new(Term::SubTree(Tree::new(left_numbers, left_ops))),
                op: current_op,
                prior,
                right: *current_right,
            }
        }
    }
}
