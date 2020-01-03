use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum TileContent {
    Wall,
    Corridor,
    Portal(String),
    Space,
    Label(char),
}

impl From<char> for TileContent {
    fn from(x: char) -> Self {
        // We can't recognize portals on first pass.
        match x {
            '#' => Self::Wall,
            '.' => Self::Corridor,
            ' ' => Self::Space,
            c => Self::Label(c),
        }
    }
}

impl TileContent {
    fn is_visitable(&self) -> bool {
        match self {
            Self::Corridor => true,
            Self::Portal(_) => true,
            _ => false,
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
            Direction::Up => Position(self.0, self.1 - 1),
            Direction::Down => Position(self.0, self.1 + 1),
            Direction::Right => Position(self.0 + 1, self.1),
            Direction::Left => Position(self.0 - 1, self.1),
        }
    }

    fn neighbours(self) -> impl Iterator<Item = Position> {
        let directions: &'static [Direction] = &[
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
}

impl Tile {
    fn new(position: Position, distance: usize) -> Self {
        Self { position, distance }
    }
}

struct Maze {
    map: HashMap<Position, TileContent>,
    portals: HashMap<String, Vec<Position>>,
}

impl Maze {
    fn new(map: HashMap<Position, TileContent>, portals: HashMap<String, Vec<Position>>) -> Self {
        Self { map, portals }
    }

    fn get_portal_partner(&self, position: &Position, label: &str) -> Option<Position> {
        self.portals[label].iter().cloned().find(|p| p != position)
    }

    fn content(&self, position: &Position) -> Option<&TileContent> {
        self.map.get(position)
    }

    fn get_portals(&self, label: &str) -> Option<&Vec<Position>> {
        self.portals.get(label)
    }

    // Returns a hashmap with distances to all reachable places, starting from the start.
    fn distances_from(&self, start: Position, allow_jumping: bool) -> HashMap<Position, usize> {
        let start_tile = Tile::new(start, 0);
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back(start_tile);

        while let Some(tile) = queue.pop_front() {
            for new_position in tile.position.neighbours() {
                if distances.contains_key(&new_position) {
                    continue;
                }
                let content = self.content(&new_position);
                if content.map_or(false, |p| p.is_visitable()) {
                    let new_distance = tile.distance + 1;
                    let new_tile = Tile::new(new_position, new_distance);
                    queue.push_back(new_tile);
                    distances.insert(new_position, new_distance);
                }
            }

            if allow_jumping {
                let content = self.content(&tile.position).unwrap();
                if let TileContent::Portal(label) = content {
                    if let Some(partner) = self.get_portal_partner(&tile.position, label) {
                        if distances.contains_key(&partner) {
                            continue;
                        }
                        let partner_distance = tile.distance + 1;
                        let new_tile = Tile::new(partner, partner_distance);
                        queue.push_back(new_tile);
                        distances.insert(partner, partner_distance);
                    }
                }
            }
        }
        distances
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Node {
    label: String,
    outer: bool,
}

impl Node {
    fn new(label: String, outer: bool) -> Self {
        Self { label, outer }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct NodePosition {
    node: Node,
    level: usize,
}

impl NodePosition {
    fn new(node: Node, level: usize) -> Self {
        Self { node, level }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct SearchState {
    position: NodePosition,
    distance: usize,
}

impl SearchState {
    fn new(node: Node, level: usize, distance: usize) -> Self {
        let position = NodePosition::new(node, level);
        Self { position, distance }
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

pub(crate) fn day20() {
    let input = File::open("data/day20.txt").expect("Failed to open input");
    let buffered = BufReader::new(input);
    let mut map = HashMap::new();
    for (y, line) in buffered.lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            let position = Position(x, y);
            let content = TileContent::from(c);
            map.insert(position, content);
        }
    }

    // Mark the portals, and create a table pairing them off.
    mark_portals(&mut map);
    let mut portals: HashMap<String, Vec<Position>> = HashMap::new();
    for (position, content) in &map {
        if let TileContent::Portal(label) = content {
            let positions = portals.entry(label.clone()).or_insert_with(|| vec![]);
            positions.push(*position);
        }
    }

    // Part one.
    let maze = Maze::new(map, portals);
    let start = maze.get_portals("AA").unwrap()[0];
    let distances = maze.distances_from(start, true);
    let end = maze.get_portals("ZZ").unwrap()[0];
    let answer = distances[&end];
    println!("Part one answer is: {}", answer);

    // Part two.
    //
    // Start by constructing a map of the distances between portals.
    let mut nodes: HashSet<Node> = HashSet::new();
    let mut distance_table = HashMap::new();
    let Position(xmax, ymax) = maze.map.keys().max().unwrap();
    for (position, content) in &maze.map {
        if let TileContent::Portal(label1) = content {
            let Position(x1, y1) = position;
            let outer1 = (*x1 < 3) || (*y1 < 3) || (*x1 + 3 > *xmax) || (*y1 + 3 > *ymax);
            let node1 = Node::new(label1.to_owned(), outer1);
            nodes.insert(node1.clone());
            let distances = maze.distances_from(*position, false);
            let mut node_distances = HashMap::new();
            for (label2, positions) in &maze.portals {
                for position2 in positions {
                    let Position(x2, y2) = position2;
                    let outer2 = (*x2 < 3) || (*y2 < 3) || (*x2 + 3 > *xmax) || (*y2 + 3 > *ymax);
                    let node2 = Node::new(label2.to_owned(), outer2);
                    if let Some(distance) = distances.get(position2) {
                        node_distances.insert(node2, *distance);
                    }
                }
            }
            distance_table.insert(node1, node_distances);
        }
    }

    // Now we search for the shortest path from AA on level 0 to ZZ on level 0.
    let start = Node::new("AA".to_owned(), true);
    let end = Node::new("ZZ".to_owned(), true);
    let state = SearchState::new(start.clone(), 0, 0);
    let mut cache = HashMap::new();
    let mut best_distance = None;
    let mut queue = BinaryHeap::new();
    queue.push(state);
    let max_depth = nodes.iter().filter(|n| n.outer).count();
    while let Some(state) = queue.pop() {
        let entry = cache
            .entry(state.position.clone())
            .or_insert(std::usize::MAX);
        if state.distance >= *entry {
            continue;
        };
        *entry = state.distance;

        if (state.position.node == end) && (state.position.level == 0) {
            best_distance.replace(state.distance);
            break;
        }

        let distances = &distance_table[&state.position.node];
        for node in &nodes {
            if (state.position.level == 0) && node.outer && node != &end {
                continue;
            }
            if (state.position.level != 0) && (node == &start || node == &end) {
                continue;
            }
            if (state.position.level == max_depth) && !node.outer {
                continue;
            }
            if let Some(step) = distances.get(node) {
                let new_level = if !node.outer {
                    state.position.level + 1
                } else if node != &end {
                    state.position.level - 1
                } else {
                    state.position.level
                };
                let new_outer = if node != &end {
                    !node.outer
                } else {
                    node.outer
                };
                let new_node = Node::new(node.label.clone(), new_outer);
                let mut new_distance = state.distance + *step;
                if node != &end {
                    new_distance += 1
                };
                let new_state = SearchState::new(new_node, new_level, new_distance);
                queue.push(new_state);
            }
        }
    }
    let best_distance = best_distance.unwrap();
    println!("Part two answer is: {}", best_distance);
}

// Updates the map with the positions of portals.
fn mark_portals(map: &mut HashMap<Position, TileContent>) {
    for position in map.clone().keys() {
        let character = map
            .get(&position)
            .and_then(|content| match content {
                TileContent::Label(c1) => Some(c1),
                _ => None,
            })
            .cloned();

        if let Some(ch) = character {
            let right = position.step(Direction::Right);
            if let Some(TileContent::Label(c2)) = map.get(&right) {
                let label: String = [ch, *c2].iter().collect();
                let right_again = right.step(Direction::Right);
                if let Some(TileContent::Corridor) = map.get(&right_again) {
                    map.insert(right_again, TileContent::Portal(label));
                } else {
                    let left = position.step(Direction::Left);
                    map.insert(left, TileContent::Portal(label));
                }
            }

            let down = position.step(Direction::Down);
            if let Some(TileContent::Label(c2)) = map.get(&down) {
                let label: String = [ch, *c2].iter().collect();
                let down_again = down.step(Direction::Down);
                if let Some(TileContent::Corridor) = map.get(&down_again) {
                    map.insert(down_again, TileContent::Portal(label));
                } else {
                    let up = position.step(Direction::Up);
                    map.insert(up, TileContent::Portal(label));
                }
            }
        }
    }
}
