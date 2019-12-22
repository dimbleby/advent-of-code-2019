use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn day2() {
    let input = File::open("data/day2.txt").expect("Failed to open input");
    let mut buffered = BufReader::new(input);
    let mut line = String::new();
    buffered.read_line(&mut line).expect("Failed to read line");
    let program: Vec<usize> = line
        .trim()
        .split(',')
        .map(|word| word.parse::<usize>().unwrap())
        .collect();

    let mut part_one = program.clone();
    part_one[1] = 12;
    part_one[2] = 2;
    execute_program(&mut part_one);
    println!("Part one answer is: {}", part_one[0]);

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut part_two = program.clone();
            part_two[1] = noun;
            part_two[2] = verb;
            execute_program(&mut part_two);
            if part_two[0] == 19_690_720 {
                let answer = 100 * noun + verb;
                println!("Part two answer is: {}", answer);
                break 'outer;
            }
        }
    }
}

fn execute_program(program: &mut [usize]) {
    let mut pc = 0;
    loop {
        let opcode = program[pc];
        match opcode {
            1 => {
                let x = program[pc + 1];
                let y = program[pc + 2];
                let dest = program[pc + 3];
                program[dest] = program[x] + program[y];
            }
            2 => {
                let x = program[pc + 1];
                let y = program[pc + 2];
                let dest = program[pc + 3];
                program[dest] = program[x] * program[y];
            }
            99 => break,
            _ => panic!("Unexpected opcode: {}!", opcode),
        }
        pc += 4;
    }
}
