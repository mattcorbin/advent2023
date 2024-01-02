use std::collections::HashSet;
use std::fs;

use petgraph::algo::connected_components;
use petgraph::prelude::*;

fn part1(input: &str) -> usize {
    let mut graph = UnGraphMap::<&str, usize>::new();
    for line in input.lines() {
        let mut splits = line.split(": ");
        let node = splits.next().unwrap();
        for neighbour in splits.next().unwrap().split(" ") {
            graph.add_edge(node, neighbour, 0);
        }
    }
    // transform the input file and muck with it as an SVG to find the 3 edges
    graph.remove_edge("bqq", "rxt");
    graph.remove_edge("btp", "qxr");
    graph.remove_edge("bgl", "vfx");
    println!("{}", connected_components(&graph));
    let mut sizes = HashSet::new();
    for node in graph.nodes() {
        let mut queue = Vec::new();
        queue.push(node);
        let mut visited = HashSet::new();
        while let Some(n) = queue.pop() {
            visited.insert(n);
            for neighbour in graph.neighbors(n) {
                if !visited.contains(neighbour) {
                    queue.push(neighbour);
                }
            }
        }
        sizes.insert(visited.len());
    }
    sizes.into_iter().reduce(|a, b| a * b).unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    println!("{}", part1(&input));
}
