use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin().lock();
    let data = parse_input(stdin);

    let diff: Vec<Vec<i32>> = data.into_iter()
        .map(|report| array_diff(&report))
        .collect();

    let safe_count = diff.into_iter()
        .filter(|diff| is_monotonic(diff))
        .filter(|diff| is_small_diff(diff))
        .count();

    println!("{safe_count}");
}

fn parse_input(input: impl BufRead) -> Vec<Vec<i32>> {
    input.lines()
    .map(|line| parse_report(&line.expect("Problem parsing input.")))
    .collect()
}

fn parse_report(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|x| x.parse().expect("Not an integer"))
        .collect()
}

fn array_diff(report: &[i32]) -> Vec<i32> {
    report.windows(2)
        .map(|x| x[1] - x[0])
        .collect()
}

fn is_monotonic(diff: &[i32]) -> bool {
    diff.iter().all(|x| *x >= 0) || diff.iter().all(|x| *x <= 0)
}

fn is_small_diff(diff: &[i32]) -> bool {
    diff.iter().all(|x| x.abs() <= 3 && x.abs() >= 1)
}
