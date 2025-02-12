use std::{collections::HashMap, sync::Arc};

advent_of_code::solution!(24);

#[derive(Debug, Clone)]
struct Gate {
    wires: (String, String),
    operator: String,
    output_wire: String,
    output_value: Option<u32>,
}

fn match_operator(operator: &str, val1: u8, val2: u8) -> u8 {
    match operator {
        "AND" => val1 & val2,
        "OR" => val1 | val2,
        "XOR" => val1 ^ val2,
        _ => panic!("ah"),
    }
}

fn solve_gates(input: &str) -> (Vec<Gate>, HashMap<String, u8>) {
    let (wires_input, gates_input) = input.split_once("\n\n").unwrap();
    let mut wires: HashMap<String, u8> = HashMap::new();
    let mut gates: Vec<Gate> = Vec::new();

    wires_input.lines().for_each(|l| {
        let split: Vec<&str> = l.split(":").collect();
        let wire = split[0].to_string();
        let value: u8 = split[1].trim().parse::<u8>().unwrap();
        wires.insert(wire, value);
    });

    // store all gates
    gates_input.lines().for_each(|l| {
        let parts: Vec<&str> = l.split("->").map(|s| s.trim()).collect();
        let input_expr = parts[0];
        let output_wire = parts[1].to_string();

        let (w1, w2, operator) = if input_expr.contains(" AND ") {
            let w: Vec<&str> = input_expr.split(" AND ").collect();
            (w[0].to_string(), w[1].to_string(), "AND".to_string())
        } else if input_expr.contains(" OR ") {
            let w: Vec<&str> = input_expr.split(" OR ").collect();
            (w[0].to_string(), w[1].to_string(), "OR".to_string())
        } else if input_expr.contains(" XOR ") {
            let w: Vec<&str> = input_expr.split(" XOR ").collect();
            (w[0].to_string(), w[1].to_string(), "XOR".to_string())
        } else {
            panic!("Unknown operation: {}", l);
        };

        gates.push(Gate {
            wires: (w1, w2),
            operator,
            output_wire,
            output_value: None,
        });
    });

    let mut progress = true;
    while progress {
        progress = false;

        for gate in gates.iter_mut() {
            if gate.output_value.is_none() {
                let val1 = wires.get(&gate.wires.0).copied();
                let val2 = wires.get(&gate.wires.1).copied();

                if let (Some(v1), Some(v2)) = (val1, val2) {
                    let result = match_operator(&gate.operator, v1, v2) as u32;
                    wires.insert(gate.output_wire.clone(), result as u8);
                    gate.output_value = Some(result);
                    progress = true;
                }
            }
        }
    }

    (gates, wires)
}

pub fn part_one(input: &str) -> Option<u128> {
    let (_, wires) = solve_gates(input);
    let (_, z_binary) = binary_from_wires(&wires, 'z');
    let result = u128::from_str_radix(&z_binary, 2).unwrap_or(0);
    Some(result)
}

fn binary_from_wires(wires: &HashMap<String, u8>, prefix: char) -> (Vec<String>, String) {
    let mut keys: Vec<String> = wires
        .keys()
        .filter(|key| key.starts_with(prefix))
        .cloned()
        .collect();

    keys.sort();
    keys.reverse();

    let binary_str: String = keys
        .iter()
        .map(|key| wires.get(key).unwrap_or(&0).to_string())
        .collect();

    (keys, binary_str)
}

fn add_binary(x: &str, y: &str) -> String {
    let num1 = u128::from_str_radix(x, 2).unwrap_or(0);
    let num2 = u128::from_str_radix(y, 2).unwrap_or(0);
    let sum = num1 + num2;

    format!("{:b}", sum)
}

pub fn part_two(input: &str) -> Option<u128> {
    let (_, wires) = solve_gates(input);

    let (z_keys, z_binary) = binary_from_wires(&wires, 'z');
    let (x_keys, x_binary) = binary_from_wires(&wires, 'x');
    let (y_keys, y_binary) = binary_from_wires(&wires, 'y');

    let xy_sum = add_binary(&x_binary, &y_binary);
    println!("xy");
    println!("Binary: {}", xy_sum);
    // println!("Z Keys: {:?}, Binary: {}", z_keys, z_binary);
    println!("Binary: {}", z_binary);

    let sum_vector: Vec<char> = xy_sum.chars().collect();
    let z_binary_vector: Vec<char> = z_binary.chars().collect();

    println!("xy sum len {}",sum_vector.len());
    println!("z binary len {}",z_binary_vector.len());

    let mut inconsistent: Vec<String> = Vec::new();

    for n in 0..sum_vector.len() -1 {
        let z_value = wires.get(&z_keys[n]).unwrap();
        let z_char = std::char::from_digit(*z_value as u32, 10).unwrap();
        let xy_value = sum_vector[n];
        if z_char != xy_value {
            inconsistent.push(z_keys[n].clone());
            
        }
    }

    println!("{:?}",inconsistent);

    /* println!("X Keys: {:?}, Binary: {}", x_keys, x_binary);
    println!("Y Keys: {:?}, Binary: {}", y_keys, y_binary); */

    /*     println!("Final Wires: {:?}", wires);
    println!("Final Gates: {:?}", gates); // All gates now have output values */



    let result = u128::from_str_radix(&z_binary, 2).unwrap_or(0);
    Some(result)
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
