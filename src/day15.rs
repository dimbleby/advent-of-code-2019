use crate::intcode::IntCode;
use std::collections::{VecDeque, HashMap, HashSet};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl From<Direction> for i64 {
    fn from(d: Direction) -> Self {
        match d {
            Direction::North => 1,
            Direction::South => 2,
            Direction::East => 3,
            Direction::West => 4,
        }
    }
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Position(isize, isize);

impl Position {
    fn step(&self, d: Direction) -> Self {
        match d {
            Direction::North => Position(self.0, self.1 + 1),
            Direction::South => Position(self.0, self.1 - 1),
            Direction::East => Position(self.0 + 1, self.1),
            Direction::West => Position(self.0 - 1, self.1),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TileContent {
    Wall,
    Empty,
    Oxygen,
}

impl From<i64> for TileContent {
    fn from(x: i64) -> Self {
        match x {
            0 => Self::Wall,
            1 => Self::Empty,
            _ => Self::Oxygen,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Tile {
    position: Position,
    distance: usize,
}

impl Tile {
    fn new(position: Position, distance: usize) -> Self {
        Self { position, distance }
    }
}

struct Droid {
    program: IntCode,
}

impl Droid {
    fn new(program: IntCode) -> Self {
        Self { program }
    }

    fn try_step(&mut self, direction: Direction) -> TileContent {
        let input = i64::from(direction);
        self.program.add_input(input);
        self.program.execute();
        let output = self.program.get_output().unwrap();
        TileContent::from(output)
    }

    fn step(&mut self, direction: Direction) {
        let content = self.try_step(direction);
        assert_ne!(content, TileContent::Wall);
    }

    fn peek(&mut self, direction: Direction) -> TileContent {
        let content = self.try_step(direction);
        if content != TileContent::Wall {
            self.step(direction.opposite());
        }
        content
    }
}

pub(crate) fn day15() {
    let line = std::fs::read_to_string("data/day15.txt").expect("Failed to open input");
    let intcode: IntCode = line.parse().expect("Could not parse program");

    // Use the droid to explore the world, depth first.  We will learn all the reachable tiles, as
    // well as the oxygen's position.
    let mut visited = HashSet::new();
    let mut oxygen_position = None;
    let origin = Position(0, 0);
    let mut droid = Droid::new(intcode);
    let mut path: Vec<Direction> = vec![];
    let mut stack = vec![(origin, 0, None)];
    while let Some((position, depth, inwards_direction)) = stack.pop() {
        if !visited.insert(position) {
            continue;
        };

        // Back up, and step to the new place (except at the root).
        if let Some(forward) = inwards_direction {
            let mut excess = path.split_off(depth - 1);
            while let Some(step) = excess.pop() {
                droid.step(step.opposite());
            }
            droid.step(forward);
            path.push(forward);
        }

        // Look for unvisited neighbours.
        for &direction in &[
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            let new_position = position.step(direction);
            let content = droid.peek(direction);
            if content != TileContent::Wall {
                stack.push((new_position, depth + 1, Some(direction)));
                if content == TileContent::Oxygen {
                    oxygen_position.replace(new_position);
                }
            }
        }
    }

    // Part one.
    let oxygen_position = oxygen_position.expect("Failed to find oxygen!");
    let distances = get_distances_from(&visited, origin);
    let answer = distances.get(&oxygen_position).unwrap();
    println!("Part one answer is: {}", answer);

    // Part two.
    let distances = get_distances_from(&visited, oxygen_position);
    let answer = distances.values().max().unwrap();
    println!("Part two answer is: {}", answer);
}

fn get_distances_from(reachable: &HashSet<Position>, start: Position) -> HashMap<Position, usize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(Tile::new(start, 0));
    while let Some(tile) = queue.pop_front() {
        for &direction in &[
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            let new_position = tile.position.step(direction);
            if reachable.contains(&new_position) && !distances.contains_key(&new_position) {
                let new_distance = tile.distance + 1;
                let new_tile = Tile::new(new_position, new_distance);
                queue.push_back(new_tile);
                distances.insert(new_position, new_distance);
            }
        }
    }
    distances
}
