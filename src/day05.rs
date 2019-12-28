use crate::intcode::IntCode;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn day05() {
    let input = File::open("data/day05.txt").expect("Failed to open input");
    let mut buffered = BufReader::new(input);
    let mut line = String::new();
    buffered.read_line(&mut line).expect("Failed to read line");
    let program: Vec<i64> = line
        .trim()
        .split(',')
        .map(|word| word.parse::<i64>().unwrap())
        .collect();

    // Part one.
    println!("Part one:");
    let mut intcode = IntCode::new(program.clone());
    intcode.add_input(1);
    intcode.execute();
    while let Some(value) = intcode.get_output() {
        println!("{}", value);
    }

    // Part two.
    println!("Part two:");
    let mut intcode = IntCode::new(program);
    intcode.add_input(5);
    intcode.execute();
    while let Some(value) = intcode.get_output() {
        println!("{}", value);
    }
}
