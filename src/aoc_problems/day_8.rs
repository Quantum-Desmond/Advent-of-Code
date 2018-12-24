// use std::collections::HashMap;
// use std::collections::HashSet;

use std::fs::File;
use std::io::prelude::*;

struct Node {
    child_count: u32,
    metadata_count: u32,
    children: Vec<Node>,
    metadata: Vec<u32>
}

impl Node {
    fn new(child_count: u32, metadata_count: u32, children: Vec<Node>, metadata: Vec<u32>) -> Node {
        Node{ child_count, metadata_count, children, metadata }
    }
}

fn create_node(ref mut nodes: &mut Vec<Node>, data_list: &Vec<u32>, mut idx: usize) -> usize {
    nodes.push(Node::new(0, 0, vec![], vec![]));
    let node = nodes.last_mut().unwrap();

    let mut result: usize = 0;

    node.child_count = data_list[idx];
    idx += 1;
    result += 1;

    node.metadata_count = data_list[idx];
    idx += 1;
    result += 1;

    for _n in 0..node.child_count {
        let idx_jump = create_node(&mut node.children, data_list, idx);
        result += idx_jump;
        idx += idx_jump;
    }

    node.metadata = data_list[idx..idx+node.metadata_count as usize].to_vec();
    if cfg!(debug_assertions) {
        assert_eq!(node.metadata.len(), node.metadata_count as usize);
    }

    result += node.metadata_count as usize;

    result
}


fn get_metadata_sum(ref node: &Node) -> u32 {
    let current_metadata_sum: u32 = node.metadata.iter().sum();
    let child_metadata_sum: u32 = node.children.iter().map(|ref child_node| get_metadata_sum(&child_node)).sum();
    current_metadata_sum + child_metadata_sum
}

fn get_root_metadata_sum(ref node: &Node) -> u32 {
    match node.child_count {
        0 => node.metadata.iter().sum(),
        _ => {
            node.metadata.iter().map(|&n: &u32| {
                let child = node.children.get((n-1) as usize);
                match child {
                    Some(child_node) => get_root_metadata_sum(child_node),
                    None => 0
                }
            }).sum()
        }
    }
}


pub fn q1(fname: String) -> u32 {
    let mut f = File::open(fname).expect("File not found");

    let mut f_str = String::new();
    f.read_to_string(&mut f_str).expect("Couldn't find file");

    let data_list: Vec<u32> = f_str.trim().split(' ').map(|s| s.parse().unwrap()).collect();

    if cfg!(debug_assertions) {
        println!("Length of list = {}", data_list.len());
    }

    let mut nodes: Vec<Node> = vec![];

    create_node(&mut nodes, &data_list, 0);

    get_metadata_sum(&nodes[0])
}

pub fn q2(fname: String) -> u32 {
    let mut f = File::open(fname).expect("File not found");

    let mut f_str = String::new();
    f.read_to_string(&mut f_str).expect("Couldn't find file");

    let data_list: Vec<u32> = f_str.trim().split(' ').map(|s| s.parse().unwrap()).collect();

    if cfg!(debug_assertions) {
        println!("Length of list = {}", data_list.len());
    }

    let mut nodes: Vec<Node> = vec![];

    create_node(&mut nodes, &data_list, 0);

    get_root_metadata_sum(&nodes[0])
}
