use advent::AdventSolver;
use failure::Error;
use std::fs::File;
use std::io::Read;
use std::iter::Iterator;

#[derive(Debug,Default)]
struct Node {
    id: usize,
    children: Vec<usize>,
    metadata: Vec<usize>,
}

impl Node {
    fn new(id: usize) -> Node {
        Node {
            id: id,
            children: Vec::new(),
            metadata: Vec::new(),
        }
    }
}

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        // Load input
        let mut input = String::new();
        File::open("input/day08.txt")?.read_to_string(&mut input)?;
        input = input.trim().to_string();

        let input_values = input.split_whitespace()
                                .map(|s| s.parse::<usize>().unwrap())
                                .collect::<Vec<usize>>();
        let mut nodes = Self::read_nodes(&mut input_values.iter(), 0, 1);
        // Sorting the vec by id allows us to index into it by id.
        nodes.sort_by_key(|n| n.id);

        // Part 1
        println!("Sum of metadata entries: {}",
                 nodes.iter()
                      .map(|n| n.metadata.iter().sum::<usize>())
                      .sum::<usize>());

        // Part 2
        let root_node = &nodes[0];
        println!("Root node value: {}", Self::node_value(root_node, &nodes));
        Ok(())
    }
}

impl Solver {
    fn node_value(node: &Node, nodes: &Vec<Node>) -> usize {
        if node.children.len() == 0 {
            node.metadata.iter().sum()
        } else {
            node.metadata
                .iter()
                .map(|&m| {
                    if m == 0 || m > node.children.len() {
                        0
                    } else {
                        Self::node_value(&nodes[node.children[m-1]], nodes)
                    }
                })
                .sum()
        }
    }

    fn read_nodes<'a, T: Iterator<Item=&'a usize>>(
            input: &mut T, next_id: usize, num_nodes: usize) -> Vec<Node> {
        let mut result = Vec::new();
        let mut next_id = next_id;
        let mut children_to_append: Vec<Node> = Vec::new();
        for _ in 0..num_nodes {
            match input.next() {
                Some(n) => {
                    let mut node = Node::new(next_id);
                    next_id += 1;
                    let num_children = *n;
                    let num_metadata = *(input.next().unwrap());
                    if num_children > 0 {
                        let mut children =
                            Self::read_nodes(input, next_id, num_children);
                        next_id = children.iter()
                                          .max_by_key(|c| c.id)
                                          .unwrap().id + 1;
                        for child in children.iter().take(num_children) {
                            node.children.push(child.id);
                        }
                        children_to_append.append(&mut children);
                    }
                    for _ in 0..num_metadata {
                        node.metadata.push(*(input.next().unwrap()));
                    }
                    result.push(node);
                },
                None => {}
            }
        }
        result.append(&mut children_to_append);
        result
    }
}

