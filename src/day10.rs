use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash)]
struct Direction {
    col: isize,
    row: isize,
}

impl Direction {
    fn new(col: isize, row: isize) -> Self {
        let gcd = gcd(col.abs() as usize, row.abs() as usize);
        let colstep = col / gcd as isize;
        let rowstep = row / gcd as isize;
        Self {
            col: colstep,
            row: rowstep,
        }
    }
}

impl Ord for Direction {
    fn cmp(&self, other: &Self) -> Ordering {
        // Equal things are equal.
        if self == other {
            return Ordering::Equal;
        };

        // We always start at (0, -1).
        if self.col == 0 && self.row < 0 {
            return Ordering::Less;
        };

        if other.col == 0 && other.row < 0 {
            return Ordering::Greater;
        };

        // Any direction on the right hand side of the clock is earlier than any direction on the
        // left hand side of the clock.
        //
        // This also takes care of cases with (0, 1).
        let self_side = self.col.signum();
        let other_side = other.col.signum();
        if self_side != other_side {
            return other_side.cmp(&self_side);
        };

        // So now we can simply compare the (tangents of) the angles formed.
        (self.row * other.col).cmp(&(other.row * self.col))
    }
}

impl PartialOrd for Direction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq)]
enum GridCell {
    Empty,
    Asteroid,
}
struct ParseGridCellError;

impl FromStr for GridCell {
    type Err = ParseGridCellError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match s {
            "." => GridCell::Empty,
            "#" => GridCell::Asteroid,
            _ => return Err(ParseGridCellError),
        };
        Ok(direction)
    }
}

struct Grid {
    asteroids: HashSet<(usize, usize)>,
}

impl Grid {
    fn get_sightlines(&self, posn: (usize, usize)) -> BTreeMap<Direction, Vec<(usize, usize)>> {
        let (col, row) = posn;
        let mut visible = BTreeMap::new();
        for (acol, arow) in &self.asteroids {
            if (*acol, *arow) == (col, row) {
                continue;
            };
            let jumpc = *acol as isize - col as isize;
            let jumpr = *arow as isize - row as isize;
            let direction = Direction::new(jumpc, jumpr);
            visible
                .entry(direction)
                .or_insert_with(|| vec![])
                .push((*acol, *arow));
        }

        visible
    }
}

pub(crate) fn day10() {
    let input = File::open("data/day10.txt").expect("Failed to open input");
    let buffered = BufReader::new(input);
    let cells: Vec<Vec<GridCell>> = buffered
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => GridCell::Empty,
                    '#' => GridCell::Asteroid,
                    _ => panic!("unexpected grid value"),
                })
                .collect()
        })
        .collect();

    // Build a more convenient representation of the grid.
    let mut asteroids = HashSet::new();
    for (row, line) in cells.iter().enumerate() {
        for (col, cell) in line.iter().enumerate() {
            if *cell == GridCell::Asteroid {
                asteroids.insert((col, row));
            }
        }
    }
    let grid = Grid { asteroids };

    // Part one.
    let laser = *grid
        .asteroids
        .iter()
        .max_by_key(|&posn| grid.get_sightlines(*posn).len())
        .unwrap();
    let mut sightlines = grid.get_sightlines(laser);
    println!("Part one answer is: {}", sightlines.len());

    // Our sightlines are automatically arranged in rotational order, also sort the entries by
    // their distance from the laser.
    //
    // Manhattan distance is fine since they're all in the same direction.
    for asts in sightlines.values_mut() {
        asts.sort_by_key(|a| {
            (laser.0 as isize - a.0 as isize).abs() + (laser.1 as isize - a.1 as isize).abs()
        });
        asts.reverse();
    }

    let mut count = 0;
    'outer: loop {
        for asts in sightlines.values_mut() {
            if let Some(a) = asts.pop() {
                count += 1;
                if count == 200 {
                    let answer = 100 * a.0 + a.1;
                    println!("Part two answer is: {}", answer);
                    break 'outer;
                }
            }
        }
    }
}

fn gcd(x: usize, y: usize) -> usize {
    let (mut a, mut b) = if x > y { (x, y) } else { (y, x) };
    loop {
        if b == 0 {
            break;
        };
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}
