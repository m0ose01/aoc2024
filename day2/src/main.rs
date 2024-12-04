use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin().lock();
    let input = parse_input(stdin);
    println!("{:?}", input);
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
