use std::collections;
use std::collections::HashMap;
use std::hash::Hash;
use std::io;
use std::fs;
use std::fmt;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

use crate::advent_of_code::Day;
use crate::advent_of_code::UDLRIterator;
use crate::advent_of_code::Direction;

pub struct Day21 { }

impl Day for Day21 {

    fn puzzle_1(input: io::Lines<io::BufReader<fs::File>>) -> String {

        use std::time::Instant;
        let now = Instant::now();    
        let (graph, edges) = generate_graph(1);
        let mut complexity: u64 = 0;
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

        for line in input {
            let code: String = line.unwrap();
            let mut prev_char = 'A';
            let mut shortest_sequence: u64 = 0;
            for char in code.chars() {
                shortest_sequence += dijkstra(&graph, &edges, (prev_char, Button::ACTIVATE), (char, Button::ACTIVATE)).unwrap();
                prev_char = char;
            }
            
            let mut numeric_code: u64 = 0; 
            let mut multiplier: u64 = 1;
            for char in code.chars().rev() {
                if char != 'A' {
                    numeric_code += char.to_digit(10).unwrap() as u64 * multiplier;
                    multiplier *= 10;
                }
            }
            complexity += numeric_code * shortest_sequence;
        }

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        dbg!(complexity);
        complexity.to_string()
    }


    fn puzzle_2(input: io::Lines<io::BufReader<fs::File>>) -> String {

        use std::time::Instant;
        let now = Instant::now();    
        let (graph, edges) = generate_graph(24);
        let mut complexity: u64 = 0;

        for line in input {
            let code: String = line.unwrap();
            let mut prev_char = 'A';
            let mut shortest_sequence: u64 = 0;
            for char in code.chars() {
                shortest_sequence += dijkstra(&graph, &edges, (prev_char, Button::ACTIVATE), (char, Button::ACTIVATE)).unwrap();
                prev_char = char;
            }
            
            let mut numeric_code: u64 = 0; 
            let mut multiplier: u64 = 1;
            for char in code.chars().rev() {
                if char != 'A' {
                    numeric_code += char.to_digit(10).unwrap() as u64 * multiplier;
                    multiplier *= 10;
                }
            }
            complexity += numeric_code * shortest_sequence;
        }
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    
        dbg!(complexity);
        complexity.to_string()    }
}

#[derive(Hash, Clone, Eq, PartialEq, Debug, Copy)]
enum Button {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    ACTIVATE
}

// TODO: why doesnt this work with dbg! ? 
impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Button::UP => write!(f, "^"),
            Button::DOWN => write!(f, "v"),
            Button::LEFT => write!(f, "<"),
            Button::RIGHT => write!(f, ">"),
            Button::ACTIVATE => write!(f, "A"),
            _ => panic!()
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u64,
    node: (char, Button),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


type Graph = HashMap<Node, Vec<Node>>;
type Edges = HashMap<(Node, Node), u64>;
type Node = (char, Button);

fn generate_graph(depth: u32) -> (Graph, Edges) {

    let keypad: Vec<Vec<char>> = Vec::from([
        Vec::from(['💀','0','A']),
        Vec::from(['1','2','3']),
        Vec::from(['4','5','6']),
        Vec::from(['7','8','9']),
    ]);

    let instructions: HashMap<(Button, Button), Vec<Button>> = HashMap::from([
        ((Button::LEFT, Button::UP), Vec::from([Button::RIGHT, Button::UP, Button::ACTIVATE])),
        ((Button::LEFT, Button::RIGHT), Vec::from([Button::RIGHT, Button::RIGHT, Button::ACTIVATE])),
        ((Button::LEFT, Button::DOWN), Vec::from([Button::RIGHT, Button::ACTIVATE])),
        ((Button::LEFT, Button::ACTIVATE), Vec::from([Button::RIGHT, Button::RIGHT, Button::UP, Button::ACTIVATE])),
        ((Button::LEFT, Button::LEFT), Vec::from([Button::ACTIVATE])),

        ((Button::RIGHT, Button::UP), Vec::from([Button::LEFT, Button::UP, Button::ACTIVATE])),
        ((Button::RIGHT, Button::LEFT), Vec::from([Button::LEFT, Button::LEFT, Button::ACTIVATE])),
        ((Button::RIGHT, Button::DOWN), Vec::from([Button::LEFT, Button::ACTIVATE])),
        ((Button::RIGHT, Button::ACTIVATE), Vec::from([Button::UP, Button::ACTIVATE])),
        ((Button::RIGHT, Button::RIGHT), Vec::from([Button::ACTIVATE])),

        ((Button::UP, Button::RIGHT), Vec::from([Button::DOWN, Button::RIGHT, Button::ACTIVATE])),
        ((Button::UP, Button::LEFT), Vec::from([Button::DOWN, Button::LEFT, Button::ACTIVATE])),
        ((Button::UP, Button::DOWN), Vec::from([Button::DOWN, Button::ACTIVATE])),
        ((Button::UP, Button::ACTIVATE), Vec::from([Button::RIGHT, Button::ACTIVATE])),
        ((Button::UP, Button::UP), Vec::from([Button::ACTIVATE])),

        ((Button::DOWN, Button::RIGHT), Vec::from([Button::RIGHT, Button::ACTIVATE])),
        ((Button::DOWN, Button::LEFT), Vec::from([Button::LEFT, Button::ACTIVATE])),
        ((Button::DOWN, Button::UP), Vec::from([Button::UP, Button::ACTIVATE])),
        ((Button::DOWN, Button::ACTIVATE), Vec::from([Button::UP, Button::RIGHT, Button::ACTIVATE])),
        ((Button::DOWN, Button::DOWN), Vec::from([Button::ACTIVATE])),

        ((Button::ACTIVATE, Button::RIGHT), Vec::from([Button::DOWN, Button::ACTIVATE])),
        ((Button::ACTIVATE, Button::LEFT), Vec::from([Button::DOWN, Button::LEFT, Button::LEFT, Button::ACTIVATE])),
        ((Button::ACTIVATE, Button::UP), Vec::from([Button::LEFT, Button::ACTIVATE])),
        ((Button::ACTIVATE, Button::DOWN), Vec::from([Button::LEFT, Button::DOWN, Button::ACTIVATE])),
        ((Button::ACTIVATE, Button::ACTIVATE), Vec::from([Button::ACTIVATE])),
    ]);


    fn calculate_cost(from_state: Button, to_state: Button, depth: u32, instructions: &HashMap<(Button, Button), Vec<Button>>, memo: &mut HashMap<(Button, Button, u32), u64>) -> u64 {
        if depth == 0 {
            return (&instructions[&(from_state, to_state)]).len() as u64;
        } else {
            let mut new_instr: u64 = 0;
            if memo.contains_key(&(from_state, to_state, depth)) {
                return memo[&(from_state, to_state, depth)];
            }

            let steps = &instructions[&(from_state, to_state)];
            if steps.len() == 1 {
                new_instr += 1;
            } else {
                // example, if from < to >, then steps are  > > a, so need, from next keypad: (a, >), (>, >), (>, a), ie: v A A ^ A
                let mut step_iter = steps.iter().peekable();
                let mut prev_robot_position = Button::ACTIVATE;
                while let Some(step) = step_iter.next() {
                    new_instr += calculate_cost(prev_robot_position.clone(), step.clone(), depth - 1, instructions, memo);
                    prev_robot_position = step.clone();
                }   
            }
            memo.entry((from_state, to_state, depth))
                .or_insert(new_instr);
            //dbg!(depth);
            new_instr
        }
    }

    let mut graph: Graph = Graph::new();
    let mut edges: Edges = Edges::new();
    let mut memo: HashMap<(Button, Button, u32), u64> = HashMap::new();

    // generate the graph
    for y in 0..keypad.len() {
        for x in 0..keypad[y].len() {
            if keypad[y][x] == '💀' {
                continue;
            }
            let mut new_nodes: Vec<(char, Button)> = Vec::new();

            // insert ('key', ACTIVATE), node, that represents actually pressing the key
            let activate_node =  (keypad[y][x], Button::ACTIVATE); 
            graph.insert(activate_node.clone(), Vec::new());
            new_nodes.push(activate_node.clone());

            for pos_data in (UDLRIterator { center: (x as i32, y as i32), 
                                                                dist_from_center: 1, 
                                                                index: 0, 
                                                                x_bound: keypad[y].len() as i32, 
                                                                y_bound: keypad.len() as i32 }) {
                let pos = pos_data.0;
                if keypad[pos.1 as usize][pos.0 as usize] == '💀' {
                    continue;
                }
                                                                        
                let keypad_state = match pos_data.1 {
                    Direction::UP => Button::DOWN,
                    Direction::DOWN => Button::UP,
                    Direction::LEFT => Button::RIGHT,
                    Direction::RIGHT => Button::LEFT
                };
                let node = (keypad[y][x], keypad_state);
                graph.insert(node.clone(), Vec::new());
                new_nodes.push(node);
            }

            //let mut activation_node_dont
            for node in new_nodes {

                for pos_data in (UDLRIterator { center: (x as i32, y as i32), 
                    dist_from_center: 1, 
                    index: 0, 
                    x_bound: keypad[y].len() as i32, 
                    y_bound: keypad.len() as i32 }) {

                    let pos: (i32, i32) = pos_data.0;
                    if keypad[pos.1 as usize][pos.0 as usize] == '💀' {
                        continue;
                    }
    
                    let keypad_dest = match pos_data.1 {
                        Direction::UP => Button::UP,
                        Direction::DOWN => Button::DOWN,
                        Direction::LEFT => Button::LEFT,
                        Direction::RIGHT => Button::RIGHT
                    };

                    let end_node = (keypad[pos.1 as usize][pos.0 as usize], keypad_dest.clone());

                    graph.entry(node.clone())
                        .and_modify(|conn_nodes| {
                            conn_nodes.push(end_node.clone());
                        });
                    let edge_cost: u64 = calculate_cost(node.clone().1, keypad_dest.clone(), depth, &instructions, &mut memo);
                    edges.insert((node.clone(), end_node.clone()), edge_cost);

                }

                // node to activate node edge represents pressing the button once you get there
                graph.entry(node.clone())
                .and_modify(|conn_nodes| {
                    conn_nodes.push(activate_node.clone());
                });

                let edge_cost: u64 = calculate_cost(node.clone().1, activate_node.clone().1, depth, &instructions, &mut memo);
                edges.insert((node.clone(), activate_node.clone()), edge_cost);
                                
            }
        }
    }

    (graph, edges)
}


fn dijkstra(graph: &Graph, 
            edges: &Edges,
            souce: (char, Button), 
            goal: (char, Button)) -> Option<u64> {

    let mut heap: BinaryHeap<State> = collections::BinaryHeap::new();
    let mut dist: HashMap<(char, Button), u64> = HashMap::new();

    for node in graph.keys() {
        if *node != souce {
            heap.push(State { cost: u64::max_value(), node: node.clone() });
            dist.insert(node.clone(), u64::max_value());
        }
    }

    dist.insert(souce.clone(), 0);
    heap.push(State { cost: 0, node: souce });

    while let Some(State { cost, node }) = heap.pop() {
        if node == goal {
            return Some(cost);
        }

        if cost > dist[&node] {
            continue;
        }

        let neighbors = &graph[&node];
        for neighbor in neighbors {
            let alt = dist[&node] + edges[&(node, neighbor.clone())];

            if alt < dist[&neighbor] {
                heap.push(State { cost: alt, node: neighbor.clone() });
                dist.insert(*neighbor, alt);
            }
        }
    }
    None
}
