use std::collections::VecDeque;
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Clone, Debug, Default)]
pub struct IntCode {
    memory: Vec<i64>,
    instruction_pointer: usize,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    relative_base: i64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<i64> for Mode {
    type Error = ();

    fn try_from(n: i64) -> Result<Self, Self::Error> {
        let mode = match n {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => return Err(()),
        };
        Ok(mode)
    }
}

#[derive(PartialEq, Eq)]
pub enum ExecuteResult {
    Done,
    InputNeeded,
}

impl IntCode {
    pub fn new(program: Vec<i64>) -> Self {
        Self {
            memory: program,
            instruction_pointer: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            relative_base: 0,
        }
    }

    pub fn read(&self, address: usize) -> i64 {
        self.memory.get(address).cloned().unwrap_or(0)
    }

    pub fn write(&mut self, address: usize, value: i64) {
        if address >= self.memory.len() {
            self.memory.resize(address + 1, 0);
        }
        self.memory[address] = value
    }

    pub fn add_input(&mut self, value: i64) {
        self.input.push_back(value)
    }

    pub fn get_output(&mut self) -> Option<i64> {
        self.output.pop_front()
    }

    fn get_opcode(&self) -> i64 {
        self.read(self.instruction_pointer)
    }

    fn get_parameter(&self, opcode: i64, parameter: usize) -> i64 {
        let mode = (opcode / 10i64.pow(parameter as u32 + 1)) % 10;
        let mode = Mode::try_from(mode).expect("Invalid mode");
        let value = self.read(self.instruction_pointer + parameter);
        match mode {
            Mode::Position => self.read(value as usize),
            Mode::Immediate => value,
            Mode::Relative => self.read((self.relative_base + value) as usize),
        }
    }

    fn get_dest(&self, opcode: i64, parameter: usize) -> i64 {
        let mode = (opcode / 10i64.pow(parameter as u32 + 1)) % 10;
        let mode = Mode::try_from(mode).expect("Invalid mode");
        let value = self.read(self.instruction_pointer + parameter);
        match mode {
            Mode::Position => value,
            Mode::Immediate => panic!("Destination parameter in immediate mode"),
            Mode::Relative => value + self.relative_base,
        }
    }

    pub fn execute(&mut self) -> ExecuteResult {
        let mut result = ExecuteResult::Done;
        loop {
            let opcode = self.get_opcode();
            match opcode % 100 {
                1 => {
                    // Addition.
                    let x = self.get_parameter(opcode, 1);
                    let y = self.get_parameter(opcode, 2);
                    let dest = self.get_dest(opcode, 3) as usize;
                    self.write(dest, x + y);
                    self.instruction_pointer += 4;
                }
                2 => {
                    // Multiplication.
                    let x = self.get_parameter(opcode, 1);
                    let y = self.get_parameter(opcode, 2);
                    let dest = self.get_dest(opcode, 3) as usize;
                    self.write(dest, x * y);
                    self.instruction_pointer += 4;
                }
                3 => {
                    // Read input and save it to address.
                    if let Some(value) = self.input.pop_front() {
                        let dest = self.get_dest(opcode, 1) as usize;
                        self.write(dest, value);
                        self.instruction_pointer += 2;
                    } else {
                        result = ExecuteResult::InputNeeded;
                        break;
                    }
                }
                4 => {
                    // Output value.
                    let value = self.get_parameter(opcode, 1);
                    self.output.push_back(value);
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
                    let dest = self.get_dest(opcode, 3) as usize;
                    self.write(dest, value);
                    self.instruction_pointer += 4;
                }
                8 => {
                    // Equals.
                    let x = self.get_parameter(opcode, 1);
                    let y = self.get_parameter(opcode, 2);
                    let value = if x == y { 1 } else { 0 };
                    let dest = self.get_dest(opcode, 3) as usize;
                    self.write(dest, value);
                    self.instruction_pointer += 4;
                }
                9 => {
                    // Relative base offset.
                    let delta = self.get_parameter(opcode, 1);
                    self.relative_base += delta;
                    self.instruction_pointer += 2;
                }
                99 => break,
                _ => panic!("Unexpected opcode: {}!", opcode),
            }
        }
        result
    }
}

impl FromStr for IntCode {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim()
            .split(',')
            .map(|word| word.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()
            .map(|p| IntCode::new(p.to_vec()))
    }
}
