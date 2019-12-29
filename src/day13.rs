use crate::intcode::IntCode;
use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl From<i64> for Tile {
    fn from(x: i64) -> Self {
        match x {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::HorizontalPaddle,
            _ => Self::Ball,
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Empty => " ",
            Self::Wall => "|",
            Self::Block => "#",
            Self::HorizontalPaddle => "_",
            Self::Ball => "*",
        };
        write!(fmt, "{}", s)
    }
}

struct Arcade {
    program: IntCode,
    screen: Vec<Vec<Tile>>,
    score: i64,
    paddle: (usize, usize),
    ball: (usize, usize),
}

impl Arcade {
    fn new(program: IntCode) -> Self {
        // Dimensions determined by examining start screen.
        let screen = vec![vec![Tile::Empty; 42]; 24];
        let mut arcade = Self {
            program,
            screen,
            score: 0,
            paddle: (100, 100),
            ball: (100, 100),
        };

        // Set free play, and write the starting screen.
        arcade.program.write(0, 2);
        arcade.step();
        arcade
    }

    fn game_over(&self) -> bool {
        !self
            .screen
            .iter()
            .any(|row| row.iter().any(|&t| t == Tile::Block))
    }

    fn tilt_joystick(&mut self, tilt: i64) {
        self.program.add_input(tilt);
    }

    fn step(&mut self) {
        self.program.execute();
        self.update_state();
    }

    fn update_state(&mut self) {
        while let Some(x) = self.program.get_output() {
            let y = self.program.get_output().unwrap();
            let value = self.program.get_output().unwrap();
            if x == -1 && y == 0 {
                self.score = value;
            } else {
                let tile = Tile::from(value);
                let x = x as usize;
                let y = y as usize;
                self.screen[y][x] = tile;
                match tile {
                    Tile::HorizontalPaddle => self.paddle = (x, y),
                    Tile::Ball => self.ball = (x, y),
                    _ => {}
                }
            }
        }
    }
}

impl fmt::Display for Arcade {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let output = self
            .screen
            .iter()
            .map(|row| row.iter().map(|t| format!("{}", t)).join(""))
            .join("\n");
        writeln!(fmt, "{}", output)?;
        write!(fmt, "Score: {}", self.score)
    }
}

pub(crate) fn day13() {
    let line = std::fs::read_to_string("data/day13.txt").expect("Failed to open input");
    let intcode: IntCode = line.parse().expect("Could not parse program");

    // Part one.
    let mut part_one = intcode.clone();
    part_one.execute();
    let mut counter = 0;
    while let Some(_output1) = part_one.get_output() {
        let _output2 = part_one.get_output().unwrap();
        let tile = part_one.get_output().unwrap();
        if Tile::from(tile) == Tile::Block {
            counter += 1;
        }
    }
    println!("Part one answer is: {}", counter);

    // Part two.
    let mut arcade = Arcade::new(intcode);
    while !arcade.game_over() {
        let input = match arcade.ball.0.cmp(&arcade.paddle.0) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };
        arcade.tilt_joystick(input);
        arcade.step();
    }
    println!("Part two answer is: {}", arcade.score);
}
