use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let lists: Vec<Vec<i32>> = parse_input(handle)
        .into_iter()
        .map(|mut v| {v.sort(); v})
        .collect();

    let diff: Vec<u32> = lists[0].iter().zip(&lists[1])
        .map(|(a, b)| i32::abs_diff(*a, *b))
        .collect();

    let sum: u32 = diff.into_iter().sum();
    println!("{sum}");

    let counts: Vec<i32> = lists[0].iter()
        .map(|a| lists[1].iter().filter(|b| a == *b).count() as i32)
        .collect();

    let similarity_score: i32 = lists[0].iter().zip(counts)
        .map(|(a, b)| a * b)
        .sum();

    println!("{similarity_score}");
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
