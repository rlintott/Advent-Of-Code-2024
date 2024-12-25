use std::collections::HashMap;
use std::io;
use std::fs;

use crate::advent_of_code::Day;

pub struct Day24 { }

impl Day for Day24 {

    fn puzzle_1(input: io::Lines<io::BufReader<fs::File>>) -> String {
        let mut wire_values: HashMap<String, bool> = HashMap::new();
        let mut gates: HashMap<String, (String, Op, String)> = HashMap::new();
        let mut z_wires: Vec<String> = Vec::new();
        let mut is_inputs = true;
        for line in input {
            let string = line.unwrap();
            if string.is_empty() { // inputs and gates separated by an empty line
                is_inputs = false;
                continue;
            }

            if is_inputs {
                let tokens: Vec<&str> = string.split(": ").collect();
                let value = if tokens[1] == "0" { false } else { true };
                wire_values.insert(tokens[0].to_string(), value);
            } else {
                let tokens: Vec<&str> = string.split_ascii_whitespace().collect();            
                let wire_1 = tokens[0].to_string();
                let wire_2 = tokens[2].to_string();
                let op = match tokens[1] {
                    "AND" => Op::AND,
                    "OR" => Op::OR,
                    "XOR" => Op::XOR,
                    _ => panic!() // not gonna happen!
                };
                let output_wire = tokens[4].to_string();
                if output_wire.chars().nth(0).unwrap() == 'z' {
                    z_wires.push(output_wire.to_string());
                }
                gates.insert(output_wire, (wire_1, op, wire_2));
            }
        }

        z_wires.sort();

        let mut result: u64 = 0;
        let mut multiplier: u64 = 1;
        for wire in z_wires {
            get_wire_value(&wire, &gates, &mut wire_values);
            dbg!(&wire);
            dbg!(wire_values[&wire] as u64);
            result += wire_values[&wire] as u64 * multiplier;
            multiplier *= 2;
        }
        dbg!(result);
        // answer too low
        result.to_string()
    }    


    fn puzzle_2(input: io::Lines<io::BufReader<fs::File>>) -> String {
        
        
        "".to_string()
    }
}

enum Op {
    AND,
    OR,
    XOR
}

fn get_wire_value(wire: &String, gates: &HashMap<String, (String, Op, String)>, wire_values: &mut HashMap<String, bool>) {
    if wire_values.contains_key(wire) {
        return;
    }
    let input: &(String, Op, String) = &gates[wire];
    if wire_values.contains_key(&input.0) == false {
        get_wire_value(&input.0, gates, wire_values);
    }
    if wire_values.contains_key(&input.2) == false {
        get_wire_value(&input.2, gates, wire_values);
    }
    let wire_1 = wire_values[&input.0];
    let wire_2 = wire_values[&input.2];
    match input.1 {
        Op::AND => { wire_values.insert(wire.to_string(), wire_1 & wire_2); },
        Op::OR => { wire_values.insert(wire.to_string(), wire_1 | wire_2); },
        Op::XOR => { wire_values.insert(wire.to_string(), wire_1 ^ wire_2); },
    }
}