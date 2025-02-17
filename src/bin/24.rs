use std::collections::{HashMap, HashSet};

/* use regex::Regex;
 */
advent_of_code::solution!(24);

/*
    Adder structure:

        x01
              \
               \
                >----[ XOR ]---  x01 XOR y01
               /                \
              /                  \
            y01                    >----[ XOR ]--- Sum bit 1 (z01)
                                /
Carry_in (from bit 00) -------/

  --- Carry Generation for Bit 1 ---

    (x01 AND y01) --> (signal A)

    (x01 XOR y01) AND Carry_in (from bit 00) --> (signal B)

    Then: Carry_out(bit 1) = A OR B  (which will be the carry for bit 2)


From input:
sum bit 00:
x00 XOR y00 -> z00

Carry from bit 00: dsr
x00 AND y00 -> dsr

Sum bit 01:
x01 XOR y01 -> nmk
nmk XOR dsr -> z01

Carry from bit 01: skd
x01 AND y01 -> hqh --> signal A
x01 XOR y01 -> nmk
nmk AND dsr -> qrt --> signal B
hqh OR qrt -> skd

Sum bit 02:
x02 XOR y02 -> ssq
skd XOR ssq -> z02

Carry from bit 02: vnw
y02 AND x02 -> jmp --> signal A
x02 XOR y02 -> ssq
ssq AND skd -> nqw --> signal B
jmp OR nqw -> vnw

*/

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

#[derive(Debug, Clone)]
struct Gate {
    in1: String,
    in2: String,
    operator: String,
    out: String,
}

/// Recursively evaluates the value on wire `w`.
fn evaluate_wire(
    w: &str,
    circuit: &HashMap<String, Gate>,
    inputs: &HashMap<String, u8>,
    memo: &mut HashMap<String, u8>,
) -> u8 {
    if let Some(&val) = inputs.get(w) {
        return val;
    }
    if let Some(&val) = memo.get(w) {
        return val;
    }
    if let Some(gate) = circuit.get(w) {
        let v1 = evaluate_wire(&gate.in1, circuit, inputs, memo);
        let v2 = evaluate_wire(&gate.in2, circuit, inputs, memo);
        let result = match gate.operator.as_str() {
            "AND" => v1 & v2,
            "OR" => v1 | v2,
            "XOR" => v1 ^ v2,
            _ => panic!("unknown operator: {}", gate.operator),
        };
        memo.insert(w.to_string(), result);
        return result;
    }
    panic!("Wire {} not found in inputs or circuit", w);
}

/// Simulates the entire circuit given the input values.
fn simulate_circuit(
    circuit: &HashMap<String, Gate>,
    inputs: &HashMap<String, u8>,
) -> HashMap<String, u8> {
    let mut outputs = HashMap::new();
    // Evaluate every wire that is driven by a gate.
    for w in circuit.keys() {
        let mut memo = HashMap::new();
        let value = evaluate_wire(w, circuit, inputs, &mut memo);
        outputs.insert(w.clone(), value);
    }
    outputs
}

/// Builds a fixed-width input vector for a given bit 'n'.
/// For n == 0:
///   vector = [0]*(44) + [test_bit]
/// For n > 0:
///   vector = [0]*(44 - n) + [test_bit] + ([carry] + [0]*(n-1))
/// Then we reverse the vector so that index 0 is the least significant bit.
fn build_input_vector(n: u8, test_bit: u8, carry: u8) -> Vec<u8> {
    if n == 0 {
        let mut vec = vec![0; 44];
        vec.push(test_bit);
        vec.reverse();
        return vec;
    } else {
        let n = n as usize;
        let mut vec = vec![0; 44 - n];
        vec.push(test_bit);
        let mut appended = vec![carry];
        appended.extend(vec![0; n - 1]);
        vec.extend(appended);
        vec.reverse();
        vec
    }
}

/// Converts a vector of bits into a map of the wires
fn vector_to_wire_map(prefix: &str, bits: Vec<u8>) -> HashMap<String, u8> {
    let mut map = HashMap::new();
    for (i, bit) in bits.iter().enumerate() {
        map.insert(format!("{}{:02}", prefix, i), *bit);
    }
    map
}

/// for a bit n, test all combinations of x, y, and carry.
/// returns true if for every combination the computed zn equals (x+y+carry) % 2.
/// Which means the ripple-adder is working correctly

fn validate_bit(n: u8, circuit: &HashMap<String, Gate>) -> bool {
    for &x in &[0, 1] {
        for &y in &[0, 1] {
            for &c in &[0, 1] {
                // For bit 0, we do not inject a carry.
                if n == 0 && c > 0 {
                    continue;
                }
                // Build test vectors for x and y.
                let x_vec = build_input_vector(n, x, c);
                let y_vec = build_input_vector(n, y, c);
                let x_map = vector_to_wire_map("x", x_vec);
                let y_map = vector_to_wire_map("y", y_vec);

                // merge x and y into one input mapping.
                let mut inputs = HashMap::new();
                inputs.extend(x_map);
                inputs.extend(y_map);

                let outputs = simulate_circuit(circuit, &inputs);
                let z_wire = format!("z{:02}", n);
                let z_val = *outputs.get(&z_wire).unwrap_or(&0);

                let expected = (x + y + c) % 2;
                if z_val != expected {
                    println!(
                        "Validation FAILED for bit {}: x={} y={} c={}, expected {} but got {}",
                        n, x, y, c, expected, z_val
                    );
                    return false;
                }
            }
        }
    }
    println!("Bit {} validated correctly.", n);
    true
}

fn get_wires(
    w: &str,
    circuit: &HashMap<String, Gate>,
    memo: &mut HashMap<String, HashSet<String>>,
) -> HashSet<String> {
    if let Some(cached) = memo.get(w) {
        return cached.clone();
    }
    let mut res = HashSet::new();
    res.insert(w.to_string());
    if let Some(gate) = circuit.get(w) {
        for input in [&gate.in1, &gate.in2].iter() {
            if circuit.contains_key(*input) {
                let deps = get_wires(input, circuit, memo);
                res.extend(deps);
            }
        }
    }
    memo.insert(w.to_string(), res.clone());
    res
}

fn relevant_gates(n: u8, circuit: &HashMap<String, Gate>) {
    let mut memo: HashMap<String, HashSet<String>> = HashMap::new();
    let wire_z_n = format!("z{:02}", n);
    let deps_z_n = get_wires(&wire_z_n, circuit, &mut memo);

    let deps_z_n_minus_1 = if n > 0 {
        let wire_z_n_minus_1 = format!("z{:02}", n - 1);
        get_wires(&wire_z_n_minus_1, circuit, &mut memo)
    } else {
        HashSet::new()
    };

    // gates unique to current bit.
    let impact: HashSet<_> = deps_z_n.difference(&deps_z_n_minus_1).cloned().collect();

    println!("Relevant gates for bit {}:", n);
    for wire in impact {
        if let Some(gate) = circuit.get(&wire) {
            println!("{:?}", gate);
        }
    }
}

pub fn part_two(input: &str) -> Option<u128> {
    let (wires_input, gates_input) = input.split_once("\n\n").unwrap();
    let mut xy_wires: HashMap<String, u8> = HashMap::new();
    let mut circuit: HashMap<String, Gate> = HashMap::new();

    // parse initial x,y wire values.
    for line in wires_input.lines() {
        let split: Vec<&str> = line.split(':').collect();
        let wire_name = split[0].trim().to_string();
        let value: u8 = split[1].trim().parse().unwrap();
        xy_wires.insert(wire_name, value);
    }

    // parse and store gates.
    for line in gates_input.lines() {
        let parts: Vec<&str> = line.split("->").map(|s| s.trim()).collect();
        let input_expr = parts[0];
        let output_wire = parts[1].to_string();

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

    for n in 0..=44 {
        if !validate_bit(n, &circuit) {
            println!("Adder validation FAILED for bit {}", n);
            // print out the relevant gates for this failing bit.
            relevant_gates(n, &circuit);
        }
    }
    relevant_gates(8, &circuit);

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
