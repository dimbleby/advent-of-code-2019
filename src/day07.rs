use crate::intcode::IntCode;
use itertools::Itertools;

struct Amp {
    intcode: IntCode,
}

impl Amp {
    fn new(intcode: &IntCode, phase: i64) -> Self {
        let mut intcode = intcode.clone();
        intcode.add_input(phase);
        Self { intcode }
    }

    fn run(&mut self, input: i64) -> Option<i64> {
        self.intcode.add_input(input);
        self.intcode.execute();
        self.intcode.get_output()
    }
}

pub(crate) fn day07() {
    let line = std::fs::read_to_string("data/day07.txt").expect("Failed to open input");
    let intcode: IntCode = line.parse().expect("Could not parse program");

    // Part one.
    let mut best_output = 0;
    for phase_sequence in (0..5).permutations(5) {
        let mut amps = phase_sequence
            .iter()
            .map(|&phase| Amp::new(&intcode, phase))
            .collect::<Vec<_>>();
        let output = run_line_once(&mut amps, 0).unwrap();
        if output > best_output {
            best_output = output
        };
    }
    println!("Part one answer is: {}", best_output);

    // Part two.
    let mut best_output = 0;
    for phase_sequence in (5..10).permutations(5) {
        let mut amps = phase_sequence
            .iter()
            .map(|&phase| Amp::new(&intcode, phase))
            .collect::<Vec<_>>();
        let output = run_line_repeatedly(&mut amps);
        if output > best_output {
            best_output = output
        };
    }
    println!("Part two answer is: {}", best_output);
}

fn run_line_once(amps: &mut [Amp], input: i64) -> Option<i64> {
    let mut signal = input;
    for amp in amps {
        signal = amp.run(signal)?;
    }
    Some(signal)
}

fn run_line_repeatedly(amps: &mut [Amp]) -> i64 {
    let mut signal = 0;
    while let Some(output) = run_line_once(amps, signal) {
        signal = output;
    }
    signal
}
