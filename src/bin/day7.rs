use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use petgraph::graphmap::GraphMap;
use petgraph::prelude::*;
use petgraph::graph::node_index;
use petgraph::visit::*;

#[macro_use]
extern crate lazy_static;

fn main() {
    day1();
}

struct Tower {
    pub name: String,
    pub weight: i32,
    pub disks_above: Option<Vec<String>>,
}

impl Tower {
    fn from_line(line: String) -> Tower {
        lazy_static! {
        static ref RE: Regex =
            Regex::new("^(?P<name>\\w+) \\((?P<weight>\\d+)\\)(?: -> (?P<next>.*))?$").unwrap();
        }
        let captures = RE.captures(line.as_str()).unwrap();
        Tower {
            name: captures.name("name").unwrap().as_str().to_owned(),
            weight: captures.name("weight").unwrap().as_str().parse().unwrap(),
            disks_above: match captures.name("next") {
                Some(x) => Some(x.as_str().split(", ").map(|x| x.to_owned()).collect()),
                None => None,
            },
        }
    }
}

#[test]
fn simple_disk() {
    let line = "pbga (66)".to_owned();
    let tower = Tower::from_line(line);
    assert_eq!(tower.name, "pbga");
    assert_eq!(tower.weight, 66);
    assert_eq!(tower.disks_above, None);
}

#[test]
fn complex_disk() {
    let line = "fwft (72) -> ktlj, cntj, xhth".to_owned();
    let tower = Tower::from_line(line);
    assert_eq!(tower.name, "fwft");
    assert_eq!(tower.weight, 72);
    assert_eq!(tower.disks_above.unwrap(), vec!["ktlj", "cntj", "xhth"]);
}

fn day1() {
    let reader = BufReader::new(File::open("res/day7_example.in").unwrap());
    let values: Vec<Tower> = reader
        .lines()
        .map(|l| Tower::from_line(l.unwrap()))
        .collect();

    let mut graph = GraphMap::new();

    for tower in &values {
        if tower.disks_above.is_some() {
            for x in tower.disks_above.as_ref().unwrap() {
                graph.add_edge(&tower.name, &x, tower.weight);
            }
        } else {
            graph.add_node(&tower.name);
        }
    }

    let graph: Graph<&String, i32, Directed> = graph.into_graph();

    let tower_bottom = graph.raw_nodes()
        .iter()
        .find(|&node| node.next_edge(Incoming) == EdgeIndex::end())
        .expect("No node without incoming edges!")
        .weight;

    println!("Tower bottom: {}", tower_bottom);


}
