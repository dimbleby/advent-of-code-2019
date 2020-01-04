use crate::intcode::{ExecuteResult, IntCode};
use std::io::BufRead;

#[derive(Clone)]
struct SpringDroid {
    program: IntCode,
}

impl SpringDroid {
    fn new(program: IntCode) -> Self {
        Self { program }
    }

    fn read_output(&mut self) -> Result<i64, String> {
        let mut error = String::new();
        while let Some(output) = self.program.get_output() {
            if output > 255 {
                return Ok(output);
            }
            error.push(output as u8 as char);
        }
        Err(error)
    }

    fn submit_line(&mut self, line: &str) {
        for c in line.chars() {
            self.program.add_input(c as u8 as i64);
        }
        self.program.add_input(b'\n' as i64);
    }
}

pub(crate) fn day21() {
    let line = std::fs::read_to_string("data/day21.txt").expect("Failed to open input");
    let intcode: IntCode = line.parse().expect("Could not parse program");

    // Part one.
    //
    // Jump if: it's safe to land at D, and any of A, B, C is missing.
    //
    // OR A J     - there is ground at A
    // AND B J    - ... and at B
    // AND C J    - ... and at C
    // NOT J J    - there is ground missing at one of A, B, C
    // AND D J    - and it's safe to land at D
    // WALK
    //
    // Part two.
    //
    // Jump if: as above, unless E and H are both empty (for then we'll be stuck unable either to
    // walk or to jump safely).
    //
    // OR A J
    // AND B J
    // AND C J
    // NOT J J
    // AND D J
    // OR E T     - there's ground at E
    // OR H T     - or at H
    // AND T J    - jump as above, if there is ground at E or H
    // RUN
    let mut droid = SpringDroid::new(intcode.clone());
    println!("Go!");
    loop {
        match droid.program.execute() {
            ExecuteResult::Done => {
                let output = droid.read_output();
                match output {
                    Ok(answer) => {
                        println!("Damage is: {}", answer);
                        break;
                    }
                    Err(display) => {
                        print!("{}", display);
                        println!("Try again!");
                        droid = SpringDroid::new(intcode.clone());
                    }
                }
            }
            ExecuteResult::InputNeeded => {}
        }
        let stdin = std::io::stdin();
        let line = stdin.lock().lines().next().unwrap().unwrap();
        droid.submit_line(&line);
    }
}
