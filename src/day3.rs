use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Segment {
    direction: Direction,
    distance: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct ParseDirectionError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct ParseSegmentError;

impl From<ParseDirectionError> for ParseSegmentError {
    fn from(_err: ParseDirectionError) -> Self {
        Self
    }
}
impl From<ParseIntError> for ParseSegmentError {
    fn from(_err: ParseIntError) -> Self {
        Self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point(i64, i64);

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(ParseDirectionError),
        };
        Ok(direction)
    }
}

impl FromStr for Segment {
    type Err = ParseSegmentError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let step = {
            let direction: Direction = s[..1].parse()?;
            let distance: i64 = s[1..].parse()?;
            Segment {
                direction,
                distance,
            }
        };
        Ok(step)
    }
}

impl Point {
    fn dist_from_origin(&self) -> i64 {
        self.0.abs() + self.1.abs()
    }

    fn step(&self, direction: Direction) -> Point {
        match direction {
            Direction::Up => Point(self.0, self.1 + 1),
            Direction::Down => Point(self.0, self.1 - 1),
            Direction::Left => Point(self.0 - 1, self.1),
            Direction::Right => Point(self.0 + 1, self.1),
        }
    }
}

pub(crate) fn day3() {
    let input = File::open("data/day3.txt").expect("Failed to open input");
    let mut buffered = BufReader::new(input);

    let mut line1 = String::new();
    buffered
        .read_line(&mut line1)
        .expect("Failed to read line1");
    let steps1: Vec<Segment> = line1
        .trim()
        .split(',')
        .map(|word| word.parse::<Segment>().unwrap())
        .collect();

    let mut line2 = String::new();
    buffered
        .read_line(&mut line2)
        .expect("Failed to read line2");
    let steps2: Vec<Segment> = line2
        .trim()
        .split(',')
        .map(|word| word.parse::<Segment>().unwrap())
        .collect();

    let visited1 = visited(&steps1);
    let visited2 = visited(&steps2);

    let set1 = visited1.keys().collect::<HashSet<_>>();
    let set2 = visited2.keys().collect::<HashSet<_>>();
    let intersections = set1.intersection(&set2);
    let closest = intersections
        .clone()
        .map(|c| c.dist_from_origin())
        .min()
        .unwrap();
    println!("Part one answer is: {}", closest);

    let shortest = intersections
        .map(|c| visited1.get(c).unwrap() + visited2.get(c).unwrap())
        .min()
        .unwrap();
    println!("Part two answer is: {}", shortest);
}

// Given a path of `Segment`s, returns a `HashMap` whose keys are the points visited by that path
// and whose values are the number of steps it took to reach the point.
fn visited(path: &[Segment]) -> HashMap<Point, i64> {
    let mut visited = HashMap::new();
    let mut posn = Point(0, 0);
    let mut steps = 0;
    for segment in path {
        for _ii in 1..=segment.distance {
            posn = posn.step(segment.direction);
            steps += 1;
            visited.entry(posn).or_insert(steps);
        }
    }
    visited
}
