use crate::intcode::IntCode;

pub(crate) fn day05() {
    let line = std::fs::read_to_string("data/day05.txt").expect("Failed to open input");
    let intcode: IntCode = line.parse().expect("Could not parse program");

    // Part one.
    println!("Part one:");
    let mut part_one = intcode.clone();
    part_one.add_input(1);
    part_one.execute();
    while let Some(value) = part_one.get_output() {
        println!("{}", value);
    }

    // Part two.
    println!("Part two:");
    let mut part_two = intcode.clone();
    part_two.add_input(5);
    part_two.execute();
    while let Some(value) = part_two.get_output() {
        println!("{}", value);
    }
}
