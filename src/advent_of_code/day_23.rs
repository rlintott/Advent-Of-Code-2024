

use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::io;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools;


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

        let mut starts_with_t: usize = 0;
        for computers in connected_computers {
            if computers.iter().any(|c| c.chars().nth(0).unwrap() == 't') {
                starts_with_t += 1;
            }
        }

        dbg!(starts_with_t);
        starts_with_t.to_string()
    }


    fn puzzle_2(input: io::Lines<io::BufReader<fs::File>>) -> String {
        let mut graph = Graph::new();
        let mut computers: HashMap<u64, String> = HashMap::new();

        for line in input {
            let nodes: Vec<String> =  line.unwrap().split('-').map(|s| s.to_string()).collect();
            let hash_1 = get_hash(&nodes[0]);
            let hash_2 = get_hash(&nodes[1]);
            computers.insert(hash_1, nodes[0].to_string());
            computers.insert(hash_2, nodes[1].to_string());
            let entry_1 = graph.entry(hash_1).or_insert(Vec::new());
            (*entry_1).push(hash_2);
            let entry_2 = graph.entry(hash_2).or_insert(Vec::new());
            (*entry_2).push(hash_1);
        }

        let mut lan_parties: HashSet<Vec<&u64>> = HashSet::new();
        let mut curr_best_size = 2;

        for node in &graph {
            let edges = node.1;
            for i in curr_best_size..edges.len() {
                let combinations = node.1.iter().combinations(i);
                for combination in combinations {
                    let mut is_a_party = true;
                    //dbg!(&combination);
                    for from_node in 0..combination.len() {
                        for to_node in 0..combination.len() {
                            if to_node == from_node {
                                continue;
                            }
                            if graph[combination[from_node]].contains(combination[to_node]) == false {
                                is_a_party = false;
                                break;
                            }
                        }
                        if is_a_party == false {
                            break;
                        }
                    }
                    if is_a_party {
                        // yay, a party!
                        let mut copy = combination.clone();
                        copy.push(node.0);
                        copy.sort();
                        lan_parties.insert(copy);
                        curr_best_size = i + 1;
                    }
                }
            }
        }

        let biggest_party = lan_parties.iter().max_by(|x, y| {x.len().cmp(&y.len())}).unwrap();
        let password = biggest_party.into_iter().map(|h| computers[h].clone()).sorted().join("-");
        dbg!(&password);
        password
    }

}

type Graph<String> = HashMap<String, Vec<String>>;


fn get_hash(string: &String) -> u64 {
    let mut hasher: DefaultHasher = DefaultHasher::new();
    string.hash(&mut hasher);
    hasher.finish()
}