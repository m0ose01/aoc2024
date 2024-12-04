use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let (mut list_a, mut list_b) = parse_input(handle);
    list_a.sort(); list_b.sort();
    let list_a = list_a; let list_b = list_b;

    let list_length = list_a.len();

    assert!(list_length == list_b.len());

    let mut diff: Vec<u32> = Vec::with_capacity(list_length);

    for ii in 0..list_a.len() {
        diff.push(list_a[ii].abs_diff(list_b[ii]));
    }

    let sum: u32 = diff.into_iter().sum();

    println!("{sum}");
}

fn parse_input(input: impl BufRead) -> (Vec<i32>, Vec<i32>) {
    input.lines()
        .map(|line| parse_line(&line.expect("Problem parsing string.")))
        .unzip()
}

fn parse_line(line: &str) -> (i32, i32) {
    let nums: Vec<i32> = line.split_whitespace()
    .map(|c| c.parse::<i32>().expect("Not an integer")).collect();
    (nums[0], nums[1])
}
