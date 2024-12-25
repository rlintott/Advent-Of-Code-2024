use std::collections::HashMap;
use std::hash::Hash;
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
        let mut wire_values: HashMap<String, bool> = HashMap::new();
        let mut wire_to_input_gates: HashMap<String, Vec<(Op, String, String)>> = HashMap::new();
        let mut wire_to_output_gates: HashMap<String, (String, Op, String)> = HashMap::new();
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
                wire_to_output_gates.insert(output_wire.to_string(), (wire_1.to_string(), op, wire_2.to_string()));
                let entry_1: &mut Vec<(Op, String, String)> = wire_to_input_gates.entry(wire_1.to_string()) .or_insert(Vec::new());
                (*entry_1).push((op, wire_2.to_string(), output_wire.to_string()));
                let entry_2: &mut Vec<(Op, String, String)> = wire_to_input_gates.entry(wire_2.to_string()) .or_insert(Vec::new());
                (*entry_2).push((op, wire_1.to_string(), output_wire.to_string()));
            }
        }
        
        let intermediate_wires: HashMap<&String, Vec<&String>> = HashMap::new();
        let mut swapped_wires: Vec<(&String, usize)> = Vec::new();

        fn assign_intermediate_wires<'a>(pos: usize, 
            wire_to_input_gates: &'a HashMap<String, Vec<(Op, String, String)>>, 
            wire_to_output_gates: HashMap<String, (String, Op, String)>, 
            intermediate_wires: &'a mut HashMap<&'a String, usize>,
            swapped_wires: &'a mut Vec<(&'a String, usize, usize)>, 
            is_half_adder: bool) {

            let x = format!("x{:02}", pos);

            fn validate_wire<'a>(pos: usize, wire: &'a String, intermediate_wires: &'a mut HashMap<&'a String, usize>, swapped_wires: &'a mut Vec<(&'a String, usize, usize)>, is_carry_output: bool, is_carry_input: bool) -> (&'a mut HashMap<&'a String, usize>, &'a mut Vec<(&'a String, usize, usize)>) {      
                if intermediate_wires.contains_key(wire) == false {
                    intermediate_wires.insert(wire, pos);
                    return (intermediate_wires, swapped_wires);
                }
                if is_carry_input {
                    if intermediate_wires[wire] != pos - 1 {
                        swapped_wires.push((wire, intermediate_wires[wire], pos - 1));
                    }
                } else if is_carry_output {
                    if intermediate_wires[wire] != pos + 1 {
                        swapped_wires.push((wire, intermediate_wires[wire], pos + 1));
                    }
                } else { 
                    swapped_wires.push((wire, intermediate_wires[wire], pos));
                }
                (intermediate_wires, swapped_wires)
            }

            let xor_1: Option<&(Op, String, String)> = wire_to_input_gates[&x].iter().find(|&x| x.0 == Op::XOR);
            let xor_1_output = &xor_1.unwrap().2;
            let (what, huh) = validate_wire(pos, xor_1_output, intermediate_wires, swapped_wires, false, false);

            let and_1: Option<&(Op, String, String)> = wire_to_input_gates[&x].iter().find(|&x| x.0 == Op::AND);
            let and_1_output: &String = &and_1.unwrap().2;
            let (what, huh) = validate_wire(pos, and_1_output, what, huh, false, false);

            if is_half_adder {
                return;
            }

            let xor_2: Option<&(Op, String, String)> = wire_to_input_gates[xor_1_output].iter().find(|&x| x.0 == Op::XOR);
            let z_output: &String = &xor_2.unwrap().2;
            let (what, huh) = validate_wire(pos, z_output, what, huh, false, false);

            let xor_2_carry_input: &String = &xor_2.unwrap().1;
            let (what, huh) = validate_wire(pos, xor_2_carry_input, what, huh, false, true);

            let and_2: Option<&(Op, String, String)> = wire_to_input_gates[xor_1_output].iter().find(|&x| x.0 == Op::AND);
            let and_2_output: &String = &and_2.unwrap().2;
            let (what, huh) = validate_wire(pos, and_2_output, what, huh, false, false);

            let or_2: Option<&(Op, String, String)> = wire_to_input_gates[xor_1_output].iter().find(|&x| x.0 == Op::OR);
            let or_2_carry_output: &String = &or_2.unwrap().2;
            let (what, huh) = validate_wire(pos, or_2_carry_output, what, huh, true, false);

        }

        "".to_string()
    }
}


#[derive(Hash, Clone, Eq, PartialEq, Debug, Copy)]
enum Op {
    AND,
    OR,
    XOR
}



enum AdderGate {
    XOR_0,
    XOR_1,
    AND_0,
    AND_1,
    OR_0
}