use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Clone)]
struct Node {
    data: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

#[derive(PartialEq)]
struct NodeS<T> {
    left: Option<T>,
    right: Option<T>,
}

impl<T> NodeS<T> {
    fn new() -> Self {
        NodeS {
            left: None,
            right: None,
        }
    }
}

impl Node {
    pub fn new(data: i32) -> Self {
        Node {
            data: data,
            left: None,
            right: None,
        }
    }
    pub fn is_null(&mut self) {
        self.right = None;
        self.left = None;
    }
}

fn preorder_traversal(root: Option<Box<Node>>, result: &mut Vec<i32>) {
    if let Some(node) = root {
        result.push(node.data);
        preorder_traversal(node.left, result);
        preorder_traversal(node.right, result);
    }
}

fn get_preorder(root: Option<Box<Node>>) -> Option<Vec<i32>> {
    let mut result = Vec::new();
    preorder_traversal(root, &mut result);
    Some(result)
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let root = parse_input_to_tree(&input);
    if let Some(preorder) = get_preorder(root) {
        println!("{:?}", preorder);
    } else {
        println!("Failed to get preorder traversal");
    }
}

fn parse_input_to_tree(input: &str) -> Option<Box<Node>> {
    let values: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    construct_tree(&values)
}

fn construct_tree(values: &[i32]) -> Option<Box<Node>> {
    let mut node_map = HashMap::new();

    for &value in values {
        let mut new_nodes = HashMap::new();
        let current_node = node_map.entry(value).or_insert_with(|| {
            let new_node = Box::new(Node::new(value));
            new_nodes.insert(value, new_node.clone());
            new_node
        });

        if let Some(left_value) = values.get(2 * value as usize - 1) {
            let left_child = new_nodes
                .entry(*left_value)
                .or_insert_with(|| Box::new(Node::new(*left_value)));
            current_node.left = Some(left_child.clone());
        }

        if let Some(right_value) = values.get(2 * value as usize) {
            let right_child = new_nodes
                .entry(*right_value)
                .or_insert_with(|| Box::new(Node::new(*right_value)));
            current_node.right = Some(right_child.clone());
        }

        node_map.extend(new_nodes);
    }

    node_map.get(&values[0]).cloned()
}
