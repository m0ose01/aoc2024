use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let lists: Vec<Vec<i32>> = parse_input(handle)
        .into_iter()
        .map(|mut v| {v.sort(); v})
        .collect();

    let list_length = lists[0].len();

    assert!(list_length == lists[1].len());

    let mut diff: Vec<u32> = Vec::with_capacity(list_length);

    for ii in 0..lists[0].len() {
        diff.push(lists[0][ii].abs_diff(lists[1][ii]));
    }

    let sum: u32 = diff.into_iter().sum();

    println!("{sum}");
}

fn parse_input(input: impl BufRead) -> [Vec<i32>; 2] {
    let t = input.lines()
        .map(|line| parse_line(&line.expect("Problem parsing string.")))
        .unzip();
    [t.0, t.1]
}

fn parse_line(line: &str) -> (i32, i32) {
    let nums: Vec<i32> = line.split_whitespace()
    .map(|c| c.parse::<i32>().expect("Not an integer")).collect();
    (nums[0], nums[1])
}
