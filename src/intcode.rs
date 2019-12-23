use std::convert::TryFrom;
use std::io;
use std::io::Write;

#[derive(Debug, Default)]
pub struct IntCode {
    memory: Vec<i64>,
    instruction_pointer: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Mode {
    Position,
    Immediate,
}

impl TryFrom<i64> for Mode {
    type Error = ();

    fn try_from(n: i64) -> Result<Self, Self::Error> {
        let mode = match n {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => return Err(()),
        };
        Ok(mode)
    }
}

impl IntCode {
    pub fn new(memory: Vec<i64>) -> Self {
        Self {
            memory,
            instruction_pointer: 0,
        }
    }

    pub fn read(&self, address: usize) -> i64 {
        self.memory[address]
    }

    pub fn write(&mut self, address: usize, value: i64) {
        self.memory[address] = value
    }

    fn get_opcode(&self) -> i64 {
        self.read(self.instruction_pointer)
    }

    fn fetch(&self, parameter: usize, mode: Mode) -> i64 {
        let value = self.read(self.instruction_pointer + parameter);
        match mode {
            Mode::Immediate => value,
            Mode::Position => self.read(value as usize),
        }
    }

    fn get_parameter(&self, opcode: i64, parameter: usize) -> i64 {
        let mode = (opcode / 10i64.pow(parameter as u32 + 1)) % 10;
        let mode = Mode::try_from(mode).expect("Invalid mode");
        self.fetch(parameter, mode)
    }

    pub fn execute(&mut self) {
        loop {
            let opcode = self.get_opcode();
            match opcode % 100 {
                1 => {
                    // Addition.
                    let x = self.get_parameter(opcode, 1);
                    let y = self.get_parameter(opcode, 2);
                    let dest = self.fetch(3, Mode::Immediate) as usize;
                    self.write(dest, x + y);
                    self.instruction_pointer += 4;
                }
                2 => {
                    // Multiplication.
                    let x = self.get_parameter(opcode, 1);
                    let y = self.get_parameter(opcode, 2);
                    let dest = self.fetch(3, Mode::Immediate) as usize;
                    self.write(dest, x * y);
                    self.instruction_pointer += 4;
                }
                3 => {
                    // Read input and save it to address.
                    print!("Provide input: ");
                    io::stdout().flush().unwrap();
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read input");
                    let value = input.trim().parse::<i64>().expect("Failed to parse input");
                    let dest = self.fetch(1, Mode::Immediate) as usize;
                    self.write(dest, value);
                    self.instruction_pointer += 2;
                }
                4 => {
                    // Output value.
                    let value = self.get_parameter(opcode, 1);
                    println!("{}", value);
                    self.instruction_pointer += 2;
                }
                5 => {
                    // Jump if true.
                    let value = self.get_parameter(opcode, 1);
                    if value != 0 {
                        self.instruction_pointer = self.get_parameter(opcode, 2) as usize;
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                6 => {
                    // Jump if false.
                    let value = self.get_parameter(opcode, 1);
                    if value == 0 {
                        self.instruction_pointer = self.get_parameter(opcode, 2) as usize;
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                7 => {
                    // Less than.
                    let x = self.get_parameter(opcode, 1);
                    let y = self.get_parameter(opcode, 2);
                    let value = if x < y { 1 } else { 0 };
                    let dest = self.fetch(3, Mode::Immediate) as usize;
                    self.write(dest, value);
                    self.instruction_pointer += 4;
                }
                8 => {
                    // Equals.
                    let x = self.get_parameter(opcode, 1);
                    let y = self.get_parameter(opcode, 2);
                    let value = if x == y { 1 } else { 0 };
                    let dest = self.fetch(3, Mode::Immediate) as usize;
                    self.write(dest, value);
                    self.instruction_pointer += 4;
                }
                99 => break,
                _ => panic!("Unexpected opcode: {}!", opcode),
            }
        }
    }
}
