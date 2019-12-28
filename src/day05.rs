use crate::intcode::IntCode;

pub(crate) fn day05() {
    let line = std::fs::read_to_string("data/day05.txt").expect("Failed to open input");
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
