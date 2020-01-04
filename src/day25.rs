use crate::intcode::{ExecuteResult, IntCode};
use std::io::BufRead;

#[derive(Clone)]
struct Droid {
    program: IntCode,
}

impl Droid {
    fn new(program: IntCode) -> Self {
        Self { program }
    }

    fn read_output(&mut self) -> String {
        let mut text = String::new();
        while let Some(output) = self.program.get_output() {
            text.push(output as u8 as char);
        }
        text
    }

    fn submit_line(&mut self, line: &str) {
        for c in line.chars() {
            self.program.add_input(c as u8 as i64);
        }
        self.program.add_input(b'\n' as i64);
    }
}

pub(crate) fn day25() {
    let line = std::fs::read_to_string("data/day25.txt").expect("Failed to open input");
    let intcode: IntCode = line.parse().expect("Could not parse program");

    // The items needed to trick the sensor are: prime number, asterisk, sand, tambourine.
    let mut droid = Droid::new(intcode);
    println!("Go!");
    loop {
        let result = droid.program.execute();
        let output = droid.read_output();
        print!("{}", output);
        if result == ExecuteResult::Done {
            println!("Game over");
            break;
        }
        let stdin = std::io::stdin();
        let line = stdin.lock().lines().next().unwrap().unwrap();
        droid.submit_line(&line);
    }
}
