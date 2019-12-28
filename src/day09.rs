use crate::intcode::IntCode;

pub(crate) fn day09() {
    let line = std::fs::read_to_string("data/day09.txt").expect("Failed to open input");
    let intcode: IntCode = line.parse().expect("Could not parse program");

    // Part one.
    let mut part_one = intcode.clone();
    part_one.add_input(1);
    part_one.execute();
    let answer = part_one.get_output().unwrap();
    println!("Part one answer is {}", answer);

    // Part two.
    let mut part_two = intcode.clone();
    part_two.add_input(2);
    part_two.execute();
    let answer = part_two.get_output().unwrap();
    println!("Part one answer is {}", answer);
}
