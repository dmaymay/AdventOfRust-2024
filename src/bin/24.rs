use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(24);

#[derive(Debug, Clone)]
struct Gate1 {
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

fn solve_gates(input: &str) -> (Vec<Gate1>, HashMap<String, u8>) {
    let (wires_input, gates_input) = input.split_once("\n\n").unwrap();
    let mut wires: HashMap<String, u8> = HashMap::new();
    let mut gates: Vec<Gate1> = Vec::new();

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

        gates.push(Gate1 {
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

#[derive(Debug, Clone)]
struct Gate {
    in1: String,
    in2: String,
    operator: String,
    out: String,
}


fn solve_wire(
    w: &str,
    init: &HashMap<&str, Vec<u8>>, // "x" -> [bits], "y" -> [bits]
    circuit: &HashMap<String, Gate>,
) -> u8 {

    let re = Regex::new(r"^(x|y)(\d{2})$").unwrap();

    if let Some(caps) = re.captures(w) {
        let var = caps.get(1).unwrap().as_str();   // "x" or "y"
        let idx = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        return init[var][idx];
    }
    // evaluate recursively
    let gate = &circuit[w];
    let left  = solve_wire(&gate.in1, init, circuit);
    let right = solve_wire(&gate.in2, init, circuit);

    match_operator(&gate.operator, left, right)
}

fn make_wire(var: &str, num: usize) -> String {
    // e.g. make_wire("z", 3) => "z03"
    format!("{}{:02}", var, num)
}

fn validate(n: usize, circuit: &HashMap<String, Gate>) -> bool {
    // We'll do up to 44 bits (or however many bits your puzzle might have).
    // The code below is just your Python approach in Rust style.
    for x in 0..2 {
        for y in 0..2 {
            for c in 0..2 {
                // If n=0, skip carry=1
                if n == 0 && c == 1 {
                    continue;
                }

                // Build init_x, init_y as in your Python code
                // e.g. 44-n zeros, then push x, possibly push c, then push zeros, then reverse
                // This is the same logic as your snippet:
                let size = 44; // or puzzle's bit count
                let mut arr_x = vec![0; size - n];
                arr_x.push(x as u8);
                if n > 0 {
                    arr_x.push(c as u8);
                    for _ in 0..(n - 1) {
                        arr_x.push(0);
                    }
                }
                arr_x.reverse();

                let mut arr_y = vec![0; size - n];
                arr_y.push(y as u8);
                if n > 0 {
                    arr_y.push(c as u8);
                    for _ in 0..(n - 1) {
                        arr_y.push(0);
                    }
                }
                arr_y.reverse();

                let mut init = HashMap::new();
                init.insert("x", arr_x);
                init.insert("y", arr_y);

                let wire_name = make_wire("z", n); // e.g. "z03"
                let z_val = solve_wire(&wire_name, &init, circuit);

                if z_val != ((x + y + c) % 2) as u8 {
                    return false;
                }
            }
        }
    }

    // If all combos passed, bit n is correct
    true
}

pub fn part_two(input: &str) -> Option<u128> {
    let (wires_input, gates_input) = input.split_once("\n\n").unwrap();
    let mut xy_wires: HashMap<String, u8> = HashMap::new();
    let mut circuit: HashMap<String, Gate> = HashMap::new();

    // parsing intial x,y wire values
    for line in wires_input.lines() {
        let split: Vec<&str> = line.split(':').collect();
        let wire_name = split[0].trim().to_string();
        let value: u8 = split[1].trim().parse().unwrap();
        xy_wires.insert(wire_name, value);
    }

    // parsing and storing gate
    for line in gates_input.lines() {
        let parts: Vec<&str> = line.split("->").map(|s| s.trim()).collect();
        let input_expr = parts[0];
        let output_wire = parts[1].to_string();

        // figure out (in1, in2, operator)
        let (in1, in2, operator) = if input_expr.contains(" AND ") {
            let w: Vec<&str> = input_expr.split(" AND ").collect();
            (w[0].to_string(), w[1].to_string(), "AND".to_string())
        } else if input_expr.contains(" OR ") {
            let w: Vec<&str> = input_expr.split(" OR ").collect();
            (w[0].to_string(), w[1].to_string(), "OR".to_string())
        } else if input_expr.contains(" XOR ") {
            let w: Vec<&str> = input_expr.split(" XOR ").collect();
            (w[0].to_string(), w[1].to_string(), "XOR".to_string())
        } else {
            panic!("Unknown operation in '{}'", line);
        };

        circuit.insert(
            output_wire.clone(),
            Gate {
                in1,
                in2,
                operator,
                out: output_wire,
            },
        );
    }

    for i in 0..45 {
        if !validate(i,&circuit) {
            println!("failed at bit {}" ,i);
        }
    }


    None
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

    /* #[test]
    fn test_tiny_circuit() {
        let input = r#"
x00: 1
x01: 1
y00: 0
y01: 1

x00 AND y00 -> z00
x01 OR y01 -> z01
"#;

        let result = run_tiny_circuit(input);
        assert_eq!(result, 2);
    } */
}
