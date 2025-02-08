use std::collections::HashMap;

advent_of_code::solution!(24);

#[derive(Debug, Clone)]
struct Gate {
    wires: (String, String),
    operator: String,
    output_wire: String,
}

fn match_operator(operator: &str, val1: u8, val2: u8) -> u8 {
    match operator {
        "AND" => val1 & val2,
        "OR" => val1 | val2,
        "XOR" => val1 ^ val2,
        _ => panic!("ah"),
    }
}

pub fn part_one(input: &str) -> Option<u128> {
    let (wires_input, gates_input) = input.split_once("\n\n").unwrap();
    let mut wires: HashMap<String, u8> = HashMap::new();
    let mut gates: Vec<Gate> = Vec::new();

    wires_input.lines().for_each(|l| {
        let split: Vec<&str> = l.split(":").collect();
        let wire = split[0].to_string();
        let value: u8 = split[1].trim().parse::<u8>().unwrap();
        wires.insert(wire, value);
    });

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
            panic!("unknown operation: {}", l);
        };

        let val1 = wires.get(&w1).copied();
        let val2 = wires.get(&w2).copied();

        match (val1, val2) {
            (Some(v1), Some(v2)) => {
                // both wires are known, compute the result immediately
                let result = match_operator(&operator, v1, v2);
                wires.insert(output_wire, result);
            }
            _ => {
                gates.push(Gate {
                    wires: (w1, w2),
                    operator,
                    output_wire,
                });
            }
        }
    });

    let mut progress = true;
    while progress {
        progress = false;
        let mut remaining_gates = Vec::new();

        for gate in gates.iter() {
            let val1 = wires.get(&gate.wires.0).copied();
            let val2 = wires.get(&gate.wires.1).copied();

            if let (Some(v1), Some(v2)) = (val1, val2) {
                let result = match_operator(&gate.operator, v1, v2);
                wires.insert(gate.output_wire.clone(), result);
                progress = true;
            } else {
                // unresolved gates for next iteration
                remaining_gates.push(gate.clone());
            }
        }
        gates = remaining_gates;
    }

    let mut z_keys: Vec<String> = wires
        .keys()
        .filter(|key| key.starts_with('z'))
        .cloned()
        .collect();
    z_keys.sort();
    z_keys.reverse();

    let binary_str: String = z_keys
        .iter()
        .map(|z| wires.get(z).unwrap_or(&0).to_string())
        .collect();

    let binary_z = u128::from_str_radix(&binary_str, 2).unwrap_or(0);

    Some(binary_z)
}

pub fn part_two(input: &str) -> Option<u32> {
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
}
