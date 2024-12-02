use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut list_a: Vec<i32> = vec![];
    let mut list_b: Vec<i32> = vec![];

    for line in handle.lines() {
        split_lists(
            &line.expect("Problem parsing string."),
            &mut list_a,
            &mut list_b
        );
    }

    let list_length = list_a.len();

    assert!(list_length == list_b.len());

    let mut diff: Vec<u32> = Vec::with_capacity(list_length);

    list_a.sort();
    list_b.sort();

    for ii in 0..list_a.len() {
        diff.push(list_a[ii].abs_diff(list_b[ii]));
    }

    let sum: u32 = diff.into_iter().sum();

    println!("{sum}");
}

fn split_lists(line: &str, list_a: &mut Vec<i32>, list_b: &mut Vec<i32>) {
    let inputs: Vec<i32> = line.split_whitespace()
        .map(|x| x.parse().expect("Not an integer."))
        .collect();
    list_a.push(inputs[0]);
    list_b.push(inputs[1]);
}
