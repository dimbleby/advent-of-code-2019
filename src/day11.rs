use crate::intcode::IntCode;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Colour {
    Black,
    White,
}

impl From<i64> for Colour {
    fn from(x: i64) -> Self {
        match x {
            0 => Self::Black,
            _ => Self::White,
        }
    }
}

impl From<Colour> for i64 {
    fn from(x: Colour) -> Self {
        match x {
            Colour::Black => 0,
            Colour::White => 1,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Turn {
    Left,
    Right,
}

impl From<i64> for Turn {
    fn from(x: i64) -> Self {
        match x {
            0 => Self::Left,
            _ => Self::Right,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn turn(self, turn: Turn) -> Self {
        match (self, turn) {
            (Direction::Up, Turn::Left) => Self::Left,
            (Direction::Left, Turn::Left) => Self::Down,
            (Direction::Down, Turn::Left) => Self::Right,
            (Direction::Right, Turn::Left) => Self::Up,
            (Direction::Up, Turn::Right) => Self::Right,
            (Direction::Left, Turn::Right) => Self::Up,
            (Direction::Down, Turn::Right) => Self::Left,
            (Direction::Right, Turn::Right) => Self::Down,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl Point {
    fn step(self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self(self.0, self.1 + 1),
            Direction::Left => Self(self.0 - 1, self.1),
            Direction::Down => Self(self.0, self.1 - 1),
            Direction::Right => Self(self.0 + 1, self.1),
        }
    }
}

pub(crate) fn day11() {
    let line = std::fs::read_to_string("data/day11.txt").expect("Failed to open input");
    let mut intcode: IntCode = line.parse().expect("Could not parse program");

    // Part one.
    let mut painted = HashMap::new();
    paint(&mut intcode.clone(), &mut painted);
    let answer = painted.len();
    println!("Part one answer is: {}", answer);

    // Part two.
    let mut painted = HashMap::new();
    painted.insert(Point(0, 0), Colour::White);
    paint(&mut intcode, &mut painted);

    let min_x = painted.keys().map(|p| p.0).min().unwrap();
    let min_y = painted.keys().map(|p| p.1).min().unwrap();
    let max_x = painted.keys().map(|p| p.0).max().unwrap();
    let max_y = painted.keys().map(|p| p.1).max().unwrap();
    let xsize = (1 + max_x - min_x) as usize;
    let mut grid: Vec<Vec<char>> = (min_y..=max_y).map(|_| vec![' '; xsize]).collect();
    for (point, colour) in painted {
        let (x, y) = ((point.0 - min_x) as usize, (max_y - point.1) as usize);
        let c = if colour == Colour::Black { ' ' } else { '#' };
        grid[y][x] = c;
    }
    for line in grid.iter().map(|v| v.iter().collect::<String>()) {
        println!("{}", line);
    }
}

fn paint(program: &mut IntCode, painted: &mut HashMap<Point, Colour>) {
    let mut pos = Point(0, 0);
    let mut direction = Direction::Up;
    loop {
        let paint = painted.get(&pos).unwrap_or(&Colour::Black);
        let input = i64::from(*paint);
        program.add_input(input);
        program.execute();
        match program.get_output() {
            Some(output1) => {
                let colour = Colour::from(output1);
                painted.insert(pos, colour);
                let output2 = program.get_output().unwrap();
                let turn = Turn::from(output2);
                direction = direction.turn(turn);
                pos = pos.step(direction);
            }
            None => break,
        }
    }
}
