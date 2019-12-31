use crate::intcode::IntCode;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TileContent {
    Scaffold,
    Robot(Direction),
    Empty,
}

impl From<char> for TileContent {
    fn from(x: char) -> Self {
        match x {
            '#' => Self::Scaffold,
            '^' => Self::Robot(Direction::Up),
            'v' => Self::Robot(Direction::Down),
            '<' => Self::Robot(Direction::Left),
            '>' => Self::Robot(Direction::Right),
            _ => Self::Empty,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Position(usize, usize);

impl Position {
    fn step(&self, d: Direction) -> Self {
        match d {
            Direction::Up => Position(self.0, self.1 + 1),
            Direction::Down => Position(self.0, self.1 - 1),
            Direction::Right => Position(self.0 + 1, self.1),
            Direction::Left => Position(self.0 - 1, self.1),
        }
    }

    fn neighbours(&self) -> impl Iterator<Item = Position> + '_ {
        let directions: &[Direction] = &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];
        directions.iter().map(move |&d| self.step(d))
    }
}

#[derive(Clone)]
struct Robot {
    program: IntCode,
}

impl Robot {
    fn new(program: IntCode) -> Self {
        Self { program }
    }

    fn get_text(&mut self) -> (String, Option<i64>) {
        let mut view = String::new();
        let mut extra = None;
        self.program.execute();
        while let Some(output) = self.program.get_output() {
            if output > 255 {
                extra.replace(output);
            };
            view.push(output as u8 as char);
        }
        (view, extra)
    }

    fn submit_text(&mut self, text: &str) {
        for c in text.chars() {
            self.program.add_input(c as u8 as i64);
        }
        self.program.add_input(b'\n' as i64);
        self.program.execute();
    }
}

pub(crate) fn day17() {
    let line = std::fs::read_to_string("data/day17.txt").expect("Failed to open input");
    let intcode: IntCode = line.parse().expect("Could not parse program");
    let mut robot = Robot::new(intcode);

    // Part one.
    let (view, _) = robot.clone().get_text();
    let mut map = HashMap::new();
    for (x, line) in view.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            let position = Position(x, y);
            map.insert(position, TileContent::from(c));
        }
    }
    let Position(xmax, ymax) = map.keys().max().unwrap();
    let mut alignment = 0;
    for x in 1..*xmax {
        for y in 1..*ymax {
            let position = Position(x, y);
            if map[&position] == TileContent::Scaffold
                && position
                    .neighbours()
                    .all(|p| map[&p] == TileContent::Scaffold)
            {
                alignment += x * y;
            }
        }
    }
    println!("Part one answer is: {}", alignment);

    // Part two.
    //
    // Our initial view looks like this.
    //
    // ..............#####............................
    // ..............#...#............................
    // ..............#...#............................
    // ..............#...#............................
    // ..............#...#............................
    // ..............#...#............................
    // ..............#...#.....#############..........
    // ..............#...#.....#...........#..........
    // ..............#.#####...#.#############........
    // ..............#.#.#.#...#.#.........#.#........
    // ..............#.#.#############.....#.#........
    // ..............#.#...#...#.#...#.....#.#........
    // ..............#######...#.#.###########........
    // ................#.......#.#.#.#.....#..........
    // ........#########.......#.#####.....#####......
    // ........#...............#...#...........#......
    // ........#...............#...#...........#......
    // ........#...............#...#...........#......
    // #########.....###########...#...........#......
    // #.............#.............#...........#......
    // #.............#.........#######.........#......
    // #.............#.........#...#.#.........#......
    // #.............#############.#.#.........######^
    // #.......................#.#.#.#................
    // #.......................#####.#................
    // #.........................#...#................
    // #.........................#...#................
    // #.........................#...#................
    // #.........................#...#................
    // #.........................#...#................
    // #######...................#...#................
    // ......#...................#...#................
    // ......#...................#####................
    // ......#........................................
    // ......#........................................
    // ......#........................................
    // ......#........................................
    // ......#........................................
    // ......#####....................................
    // ..........#....................................
    // ..........#....................................
    // ..........#....................................
    // ..........#....................................
    // ..........#....................................
    // ..........#....................................
    // ..........#....................................
    // ..........#############........................
    //
    // By inspection, the path we want is:
    //
    // L6,R8,L4,R8,L12,L12,R10,L4,L12,R10,L4,L12,L6,L4,
    // L4,L12,R10,L4,L12,L6,L4,L4,L12,R10,L4,L12,L6,L4,
    // L4,L6,R8,L4,R8,L12,L6,R8,L4,R8,L12
    //
    // Which we can compress as:
    //
    // A,B,B,C,B,C,B,C,A,A
    //
    // A = L,6,R,8,L,4,R,8,L,12
    // B = L,12,R,10,L,4
    // C = L,12,L,6,L,4,L,4
    robot.program.write(0, 2);
    let (_view, _) = robot.get_text();
    let (_main, _) = robot.get_text();
    robot.submit_text("A,B,B,C,B,C,B,C,A,A");
    let (_function_a, _) = robot.get_text();
    robot.submit_text("L,6,R,8,L,4,R,8,L,12");
    let (_function_b, _) = robot.get_text();
    robot.submit_text("L,12,R,10,L,4");
    let (_function_c, _) = robot.get_text();
    robot.submit_text("L,12,L,6,L,4,L,4");
    let (_video, _) = robot.get_text();
    robot.submit_text("n");
    let (_view, dust) = robot.get_text();
    println!("Part two answer is: {}", dust.unwrap());
}
