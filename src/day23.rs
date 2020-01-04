use crate::intcode::IntCode;

#[derive(Clone)]
struct Computer {
    program: IntCode,
    queue: Vec<i64>,
}

impl Computer {
    fn new(program: IntCode) -> Self {
        Self {
            program,
            queue: vec![],
        }
    }

    fn initialize(&mut self, address: i64) {
        self.program.add_input(address);
        self.program.add_input(-1);
    }

    fn read_message(&mut self) -> Option<(i64, (i64, i64))> {
        match self.program.get_output() {
            None => None,
            Some(address) => {
                let x = self.program.get_output().unwrap();
                let y = self.program.get_output().unwrap();
                Some((address, (x, y)))
            }
        }
    }

    fn queue_packet(&mut self, (x, y): (i64, i64)) {
        self.program.add_input(x);
        self.program.add_input(y);
    }

    fn execute(&mut self) -> Vec<(i64, (i64, i64))> {
        self.program.execute();
        let mut messages = vec![];
        while let Some(message) = self.read_message() {
            messages.push(message);
        }
        messages
    }
}

pub(crate) fn day23() {
    let line = std::fs::read_to_string("data/day23.txt").expect("Failed to open input");
    let intcode: IntCode = line.parse().expect("Could not parse program");

    // Part one.
    let mut computers = (0..50)
        .map(|_| Computer::new(intcode.clone()))
        .collect::<Vec<_>>();

    for (address, computer) in computers.iter_mut().enumerate() {
        computer.initialize(address as i64);
    }
    'outer: loop {
        for idx in 0..50 {
            let computer = &mut computers[idx];
            let results = computer.execute();
            for (address, packet) in &results {
                if *address == 255 {
                    let (_, y) = packet;
                    println!("Part one answer is: {}", y);
                    break 'outer;
                }
                let other = &mut computers[*address as usize];
                other.queue_packet(*packet);
            }
        }
    }

    // Part two.
    let mut computers = (0..50)
        .map(|_| Computer::new(intcode.clone()))
        .collect::<Vec<_>>();

    for (address, computer) in computers.iter_mut().enumerate() {
        computer.initialize(address as i64);
    }
    let mut nat_packet = (0, 0);
    let mut last_nat_y = 0;
    loop {
        let mut activity = false;
        for idx in 0..50 {
            let computer = &mut computers[idx];
            let results = computer.execute();
            for (address, packet) in &results {
                if *address == 255 {
                    nat_packet = *packet;
                } else {
                    activity = true;
                    let other = &mut computers[*address as usize];
                    other.queue_packet(*packet);
                }
            }
        }

        if !activity {
            let (_, y) = nat_packet;
            if y == last_nat_y {
                println!("Part two answer is: {}", y);
                break;
            }
            last_nat_y = y;

            let zero = &mut computers[0];
            zero.queue_packet(nat_packet);
        }
    }
}
