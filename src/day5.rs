use crate::intcode::IntCode;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn day5() {
    let input = File::open("data/day5.txt").expect("Failed to open input");
    let mut buffered = BufReader::new(input);
    let mut line = String::new();
    buffered.read_line(&mut line).expect("Failed to read line");
    let program: Vec<i64> = line
        .trim()
        .split(',')
        .map(|word| word.parse::<i64>().unwrap())
        .collect();

    // Provide user input: 1 for part 1, 5 for part 2.
    let mut intcode = IntCode::new(program);
    intcode.execute();
}
