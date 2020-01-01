use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

lazy_static! {
    static ref ALPHABET_BITS: HashMap<char, u32> = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .enumerate()
        .map(|(i, c)| (c, 1u32 << i))
        .collect();
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum TileContent {
    Wall,
    Empty,
    Me,
    Key(char),
    Door(char),
}

impl From<char> for TileContent {
    fn from(x: char) -> Self {
        match x {
            '#' => Self::Wall,
            '.' => Self::Empty,
            '@' => Self::Me,
            c => {
                if c >= 'a' {
                    Self::Key(c)
                } else {
                    Self::Door(c)
                }
            }
        }
    }
}

impl TileContent {
    fn is_visitable(self) -> bool {
        match self {
            Self::Wall => false,
            _ => true,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct Tile {
    position: Position,
    distance: usize,
    doors: u32,
}

impl Tile {
    fn new(position: Position, distance: usize, doors: u32) -> Self {
        Self {
            position,
            distance,
            doors,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct SearchNode {
    position: Position,
    collected: u32,
}

impl SearchNode {
    fn new(position: Position) -> Self {
        Self {
            position,
            collected: 0,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct SearchState {
    node: SearchNode,
    distance: usize,
}

impl SearchState {
    fn new(node: SearchNode, distance: usize) -> Self {
        Self { node, distance }
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct MultiSearchNode {
    positions: [Position; 4],
    collected: u32,
}

impl MultiSearchNode {
    fn new(positions: [Position; 4]) -> Self {
        Self {
            positions,
            collected: 0,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct MultiState {
    node: MultiSearchNode,
    distance: usize,
}

impl MultiState {
    fn new(node: MultiSearchNode, distance: usize) -> Self {
        Self { node, distance }
    }
}

impl Ord for MultiState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for MultiState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub(crate) fn day18() {
    let input = File::open("data/day18.txt").expect("Failed to open input");
    let buffered = BufReader::new(input);
    let mut map = HashMap::new();
    let mut me = None;
    for (y, line) in buffered.lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            let position = Position(x, y);
            let content = TileContent::from(c);
            if content == TileContent::Me {
                me.replace(position);
            }
            map.insert(position, content);
        }
    }
    let me = me.unwrap();
    let key_positions: HashMap<char, Position> = map
        .iter()
        .filter_map(|(&position, &content)| match content {
            TileContent::Key(c) => Some((c, position)),
            _ => None,
        })
        .collect();
    let all_keys = ALPHABET_BITS.values().fold(0, |acc, bit| acc | bit);

    // Part one: find shortest path to a state where we've collected all keys.
    let part_one = map.clone();
    let node = SearchNode::new(me);
    let mut distance_table = HashMap::new();
    let mut cache = HashMap::new();
    let mut best_distance = None;
    let state = SearchState::new(node, 0);
    let mut queue = BinaryHeap::new();
    queue.push(state);

    while let Some(state) = queue.pop() {
        if state.node.collected == all_keys {
            best_distance.replace(state.distance);
            break;
        }

        let entry = cache.entry(state.node).or_insert(std::usize::MAX);
        if state.distance >= *entry {
            continue;
        };
        *entry = state.distance;

        let distances = distance_table
            .entry(state.node.position)
            .or_insert_with(|| distances_to_keys(&part_one, state.node.position));

        for (key, (step, doors)) in distances {
            let key_bit = ALPHABET_BITS[key];
            if (state.node.collected & key_bit) != 0 {
                continue;
            }
            if (*doors & !state.node.collected) != 0 {
                continue;
            }
            let mut new_state = state;
            new_state.node.collected |= key_bit;
            new_state.node.position = key_positions[key];
            new_state.distance += *step;
            queue.push(new_state);
        }
    }
    let best_distance = best_distance.unwrap();
    println!("Part one answer is: {}", best_distance);

    // Part two is much the same.
    let mut part_two = map;
    part_two.insert(me, TileContent::Wall);
    part_two.insert(me.step(Direction::Up), TileContent::Wall);
    part_two.insert(me.step(Direction::Down), TileContent::Wall);
    part_two.insert(me.step(Direction::Left), TileContent::Wall);
    part_two.insert(me.step(Direction::Right), TileContent::Wall);

    let starts = [
        me.step(Direction::Up).step(Direction::Left),
        me.step(Direction::Up).step(Direction::Right),
        me.step(Direction::Down).step(Direction::Left),
        me.step(Direction::Down).step(Direction::Right),
    ];

    let node = MultiSearchNode::new(starts);
    let mut distance_table = HashMap::new();
    let mut cache = HashMap::new();
    let mut best_distance = None;
    let state = MultiState::new(node, 0);
    let mut queue = BinaryHeap::new();
    queue.push(state);

    while let Some(state) = queue.pop() {
        if state.node.collected == all_keys {
            best_distance.replace(state.distance);
            break;
        }

        let entry = cache.entry(state.node).or_insert(std::usize::MAX);
        if state.distance >= *entry {
            continue;
        };
        *entry = state.distance;

        for (bot, position) in state.node.positions.iter().enumerate() {
            let distances = distance_table
                .entry(*position)
                .or_insert_with(|| distances_to_keys(&part_two, *position));

            for (key, (step, doors)) in distances {
                let key_bit = ALPHABET_BITS[key];
                if (state.node.collected & key_bit) != 0 {
                    continue;
                }
                if (*doors & !state.node.collected) != 0 {
                    continue;
                }
                let mut new_state = state;
                new_state.node.collected |= key_bit;
                new_state.node.positions[bot] = key_positions[key];
                new_state.distance += *step;
                queue.push(new_state);
            }
        }
    }
    let best_distance = best_distance.unwrap();
    println!("Part two answer is: {}", best_distance);
}

// Returns a hash map keyed by key, where the entries give the distance to that key and the doors
// that we must pass through to get there.
fn distances_to_keys(
    map: &HashMap<Position, TileContent>,
    start: Position,
) -> HashMap<char, (usize, u32)> {
    let start_tile = Tile::new(start, 0, 0);
    let mut distances: HashMap<Position, Tile> = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(start_tile);

    while let Some(tile) = queue.pop_front() {
        for new_position in tile.position.neighbours() {
            let content = map.get(&new_position);
            if content.map_or(false, |p| p.is_visitable()) && !distances.contains_key(&new_position)
            {
                let new_distance = tile.distance + 1;
                let mut doors = tile.doors;
                if let Some(TileContent::Door(c)) = content {
                    doors |= ALPHABET_BITS[&c.to_ascii_lowercase()];
                }
                let new_tile = Tile::new(new_position, new_distance, doors);
                queue.push_back(new_tile.clone());
                distances.insert(new_position, new_tile);
            }
        }
    }
    distances
        .into_iter()
        .filter_map(|(p, t)| match map[&p] {
            TileContent::Key(c) => Some((c, (t.distance, t.doors))),
            _ => None,
        })
        .collect()
}
