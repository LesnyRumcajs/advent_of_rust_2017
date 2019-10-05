use advent_of_rust_2017::helpers::*;
use itertools::Itertools;
use petgraph::prelude::*;
use petgraph::visit::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::day7_helpers::tower::{Tower, Program};

mod day7_helpers;

#[macro_use]
extern crate lazy_static;

fn main() {
    day1_2();
}

fn day1_2() {
    let reader = BufReader::new(File::open("res/day7.in").unwrap());
    let values: Vec<Tower> = reader
        .lines()
        .map(|l| Tower::from_line(l.unwrap()))
        .collect();

    let mut graph = DiGraphMap::new();

    let mut programs = HashMap::new();
    for tower in &values {
        programs
            .entry(&tower.program.name)
            .or_insert(&tower.program);
    }

    for tower in &values {
        graph.add_node(&tower.program);
    }

    for tower in &values {
        if tower.disks_above.is_some() {
            for x in tower.disks_above.as_ref().unwrap() {
                graph.add_edge(&tower.program, &programs[x], 0);
            }
        }
    }

    let graph: Graph<&Program, i32, Directed> = graph.into_graph();
    let tower_bottom = graph
        .raw_nodes()
        .iter()
        .find(|&node| node.next_edge(Incoming) == EdgeIndex::end())
        .expect("No node without incoming edges!")
        .weight;

    println!("Tower bottom: {}", tower_bottom.name);

    let weight = calculate_correct_program_weight(&graph, &tower_bottom);
    println!("Corrected program weight: {}", weight);
}

fn calculate_correct_program_weight(
    graph: &Graph<&Program, i32, Directed, u32>,
    tower_bottom: &Program,
) -> i32 {
    let mut node = graph
        .node_indices()
        .find(|&n| graph.node_weight(n).unwrap() == &tower_bottom)
        .unwrap();

    let mut diff = 0;
    loop {
        let sums = graph
            .neighbors_directed(node, Outgoing)
            .map(|n| {
                (
                    n,
                    Bfs::new(&graph, n)
                        .iter(&graph)
                        .map(|n| graph.node_weight(n).unwrap().weight)
                        .sum::<i32>(),
                )
            })
            .collect_vec();
        let weights = sums.iter().map(|n| n.1).collect_vec();
        let different_element_index = container_helpers::first_different_element(&weights);
        if different_element_index.is_none() {
            return graph.node_weight(node).unwrap().weight - diff;
        }
        let index = different_element_index.unwrap();
        if index != 0 {
            diff = (weights[index] - weights[0]).abs();
        }

        node = sums[index].0;
    }
}
