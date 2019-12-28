use crate::intcode::IntCode;

pub(crate) fn day02() {
    let line = std::fs::read_to_string("data/day02.txt").expect("Failed to open input");
    let intcode: IntCode = line.parse().expect("Could not parse program");

    let mut part_one = intcode.clone();
    part_one.write(1, 12);
    part_one.write(2, 2);
    part_one.execute();
    let answer = part_one.read(0);
    println!("Part one answer is: {}", answer);

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut part_two = intcode.clone();
            part_two.write(1, noun);
            part_two.write(2, verb);
            part_two.execute();
            if part_two.read(0) == 19_690_720 {
                let answer = 100 * noun + verb;
                println!("Part two answer is: {}", answer);
                break 'outer;
            }
        }
    }
}
