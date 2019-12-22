use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn day1() {
    let input = File::open("data/day1.txt").expect("Failed to open input");
    let buffered = BufReader::new(input);
    let masses: Vec<u64> = buffered
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    let part_one_fuel: u64 = masses.iter().map(|&m| fuel_needed(m)).sum();
    println!("Part one answer is: {}", part_one_fuel);

    let part_two_fuel: u64 = masses.iter().map(|&m| fuel_really_needed(m)).sum();
    println!("Part two answer is: {}", part_two_fuel);
}

fn fuel_needed(mass: u64) -> u64 {
    let third = mass / 3;
    if third > 2 {
        third - 2
    } else {
        0
    }
}

fn fuel_really_needed(mass: u64) -> u64 {
    let mut total = 0;
    let mut fuel = fuel_needed(mass);
    while fuel > 0 {
        total += fuel;
        fuel = fuel_needed(fuel);
    }
    total
}
