use crate::intcode::IntCode;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Position(usize, usize);

#[derive(Clone)]
struct Drone {
    program: IntCode,
}

impl Drone {
    fn new(program: IntCode) -> Self {
        Self { program }
    }

    fn probe(&mut self, position: Position) -> i64 {
        self.program.add_input(position.0 as i64);
        self.program.add_input(position.1 as i64);
        self.program.execute();
        self.program.get_output().unwrap()
    }
}

pub(crate) fn day19() {
    let line = std::fs::read_to_string("data/day19.txt").expect("Failed to open input");
    let intcode: IntCode = line.parse().expect("Could not parse program");
    let drone = Drone::new(intcode);

    // Part one.
    let answer: i64 = iproduct!(0..50, 0..50)
        .map(|(x, y)| drone.clone().probe(Position(x, y)))
        .sum();
    println!("Part one answer is: {}", answer);

    // Part two.
    //
    // Slow but straightforward.
    let mut answer = None;
    'outer: for sum in 0..2000 {
        for x in 0..sum {
            let y = sum - x;
            if drone.clone().probe(Position(x, y)) == 0 {
                continue;
            }
            if drone.clone().probe(Position(x + 99, y)) == 0 {
                continue;
            }
            if drone.clone().probe(Position(x, y + 99)) == 0 {
                continue;
            }
            if drone.clone().probe(Position(x + 99, y + 99)) == 0 {
                continue;
            }
            answer.replace(10000 * x + y);
            break 'outer;
        }
    }
    let answer = answer.unwrap();
    println!("Part two answer is: {}", answer);
}
