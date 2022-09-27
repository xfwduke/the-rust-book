#![allow(dead_code, unused)]
mod cal_impl;

use cal_impl::v2;
use std::collections::HashMap;
use std::fmt::{format, Debug};

fn main() {
    let mut expr_hashmap: HashMap<String, Vec<String>> = HashMap::new();
    for numbers in permutations(vec![13.0, 13.0, 1.0, 3.0]) {
        for ops in generate_ops(vec!["+", "-", "*", "/"]) {
            let expr = v2::Expression::new(numbers.clone(), ops);
            if expr.value == 24.0 {
                expr_hashmap.entry(expr.signature).or_insert(vec![]).push(expr.string);
            }
        }
    }
    
    for (k, v) in expr_hashmap {
        println!("{}: {:?}", k, v)
    }
}

fn permutations<T: Clone + Debug>(input: Vec<T>) -> Vec<Vec<T>> {
    if input.len() == 1 {
        return vec![input];
    }

    let mut output: Vec<Vec<T>> = vec![];

    for (idx, value) in input.iter().enumerate() {
        let mut left_input = input.clone();

        left_input.remove(idx);

        for mut perm in permutations(left_input) {
            perm.push(value.clone());
            output.push(perm);
        }
    }
    output
}

fn generate_ops(input: Vec<&str>) -> Vec<Vec<&str>> {
    let mut output = vec![];
    for a in input.iter() {
        for b in input.iter() {
            for c in input.iter() {
                output.push(vec![*a, *b, *c])
            }
        }
    }
    output
}
