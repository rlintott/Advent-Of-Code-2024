
use std::io;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::advent_of_code::Day;

pub struct Day23 { }

impl Day for Day23 {

    fn puzzle_1(input: io::Lines<io::BufReader<fs::File>>) -> String {
        let mut graph = Graph::new();
        for line in input {
            let nodes: Vec<String> =  line.unwrap().split('-').map(|s| s.to_string()).collect();
            let entry_1 = graph.entry(nodes[0].to_string()).or_insert(Vec::new());
            (*entry_1).push(nodes[1].to_string());
            let entry_2 = graph.entry(nodes[1].to_string()).or_insert(Vec::new());
            (*entry_2).push(nodes[0].to_string());
        }

        let mut connected_computers: HashSet<Vec<&str>> = HashSet::new();

        for node in &graph {
            let edges = node.1;
            for i in 0..edges.len() {
                for j in i + 1..edges.len() {
                    let i_node_edges = &graph[&edges[i]];
                    if i_node_edges.contains(&edges[j]) {
                        let mut triplet: Vec<&str> = Vec::new();
                        triplet.push(&node.0);
                        triplet.push(&edges[i]);
                        triplet.push(&edges[j]);
                        triplet.sort();            
                        connected_computers.insert(triplet);
                    }
                }
            }
        }

        //dbg!(connected_computers);
        "".to_string()
    }


    fn puzzle_2(input: io::Lines<io::BufReader<fs::File>>) -> String {
        "".to_string()
    }

}

type Graph<String> = HashMap<String, Vec<String>>;
