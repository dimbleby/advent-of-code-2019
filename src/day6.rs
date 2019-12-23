use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

#[derive(Default)]
struct Node {
    parent_id: Option<String>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn set_parent(&mut self, parent_id: &str) {
        self.parent_id.replace(parent_id.into());
    }

    fn add_child(&mut self, child: Rc<RefCell<Node>>) {
        self.children.push(child);
    }
}

pub(crate) fn day6() {
    let input = File::open("data/day6.txt").expect("Failed to open input");
    let buffered = BufReader::new(input);
    let lines = buffered.lines().map(|line| line.unwrap());

    // Build our graph.
    let mut nodes: HashMap<String, Rc<RefCell<Node>>> = HashMap::new();
    for line in lines {
        let centre = &line[..3];
        let orbiter = &line[4..];
        let centre_node = nodes
            .entry(centre.into())
            .or_insert_with(|| Rc::new(RefCell::new(Node::default())))
            .clone();
        let orbiting_node = nodes
            .entry(orbiter.into())
            .or_insert_with(|| Rc::new(RefCell::new(Node::default())));
        centre_node.borrow_mut().add_child(orbiting_node.clone());
        orbiting_node.borrow_mut().set_parent(centre);
    }

    // Find the root.
    let root = nodes.get("COM").expect("Failed to find centre of mass");

    // Solve part one, via depth-first search.
    let mut total = 0;
    let mut stack = vec![(root.clone(), 0)];
    while let Some((node, depth)) = stack.pop() {
        total += depth;
        for child in &node.borrow().children {
            stack.push((child.clone(), depth + 1));
        }
    }
    println!("Part one answer is: {}", total);

    // Solve part two - the shortest path will take us up to our common parent and down again.
    // So find that common parent, and do the maths.
    let mut you_ancestors = get_ancestors(&nodes, "YOU");
    let mut santa_ancestors = get_ancestors(&nodes, "SAN");
    while you_ancestors.pop() == santa_ancestors.pop() {}
    let you_distance = you_ancestors.len() + 1;
    let santa_distance = santa_ancestors.len() + 1;
    let answer = you_distance + santa_distance;
    println!("Part two answer is: {}", answer);
}

fn get_ancestors(nodes: &HashMap<String, Rc<RefCell<Node>>>, id: &str) -> Vec<String> {
    let mut ancestors = vec![];
    let mut node = nodes.get(id).expect("Failed to find node!");
    while let Some(parent_id) = &node.borrow().parent_id {
        ancestors.push(parent_id.clone());
        node = nodes.get(parent_id).expect("Failed to find parent node!");
    }
    ancestors
}
