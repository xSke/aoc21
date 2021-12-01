use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

pub fn input(day: usize) -> String {
    let filename = format!("inputs/day{}.txt", day);
    let input = fs::read_to_string(filename).expect("could not open file");
    input.trim().to_string()
}

pub fn input_lines(day: usize) -> Vec<String> {
    let input = input(day);
    input
        .split('\n')
        .map(|line| line.trim().to_string())
        .collect()
}

pub fn input_lines_parsed<T>(day: usize) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let lines = input_lines(day);
    lines
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            line.parse()
                .expect(&format!("could not parse line {}", i + 1))
        })
        .collect()
}
