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


// PART TWO

#[derive(Debug, Clone)]
pub struct Gate {
    pub in1: String,
    pub in2: String,
    pub operator: String,
    pub out: String,
}

/// recursively fetches the value on wire `w`, using input xNN/yNN wires
fn evaluate_wire(
    w: &str,
    circuit: &HashMap<String, Gate>,
    inputs: &HashMap<String, u8>,
    memo: &mut HashMap<String, u8>,
) -> u8 {
    // if w is in inputs (an xNN or yNN wire), return that bit
    if let Some(&val) = inputs.get(w) {
        return val;
    }
    if let Some(&val) = memo.get(w) {
        return val;
    }
    // otherwise recursively find
    if let Some(gate) = circuit.get(w) {
        let v1 = evaluate_wire(&gate.in1, circuit, inputs, memo);
        let v2 = evaluate_wire(&gate.in2, circuit, inputs, memo);
        let result = match gate.operator.as_str() {
            "AND" => v1 & v2,
            "OR"  => v1 | v2,
            "XOR" => v1 ^ v2,
            _ => panic!("unknown operator: {}", gate.operator),
        };
        memo.insert(w.to_string(), result);
        return result;
    }
    panic!("Wire {} not found in inputs or circuit", w);
}

/// runs the circuit for all gate-output wires
fn simulate_circuit(
    circuit: &HashMap<String, Gate>,
    inputs: &HashMap<String, u8>,
) -> HashMap<String, u8> {
    let mut outputs = HashMap::new();
    for out_wire in circuit.keys() {
        let mut memo = HashMap::new();
        let val = evaluate_wire(out_wire, circuit, inputs, &mut memo);
        outputs.insert(out_wire.clone(), val);
    }
    outputs
}

/// build a test input vector for the n-th bit
/// plus optional carry in c for bits > 0. Reversed so index 0 is the least significant bit.
fn build_input_vector(n: u8, test_bit: u8, carry: u8) -> Vec<u8> {
    if n == 0 {
        let mut vec = vec![0; 44];
        vec.push(test_bit);
        vec.reverse();
        vec
    } else {
        let n_usize = n as usize;
        let mut vec = vec![0; 44 - n_usize];
        vec.push(test_bit);
        let mut appended = vec![carry];
        appended.extend(vec![0; n_usize - 1]);
        vec.extend(appended);
        vec.reverse();
        vec
    }
}

fn vector_to_wire_map(prefix: &str, bits: Vec<u8>) -> HashMap<String, u8> {
    let mut map = HashMap::new();
    for (i, &b) in bits.iter().enumerate() {
        map.insert(format!("{}{:02}", prefix, i), b);
    }
    map
}

/// validate the n-th bit for all combinations of x,y in {0,1}, plus carry in {0,1} if n>0.
/// return true if z_n always matches (x + y + carry) % 2, else false.
pub fn validate_bit(n: u8, circuit: &HashMap<String, Gate>) -> bool {
    for &x in &[0,1] {
        for &y in &[0,1] {
            for &c in &[0,1] {
                if n == 0 && c == 1 {
                    // No carry in at bit 0
                    continue;
                }
                // Build test vectors
                let xv = build_input_vector(n, x, c);
                let yv = build_input_vector(n, y, c);
                let x_map = vector_to_wire_map("x", xv);
                let y_map = vector_to_wire_map("y", yv);
                let mut inputs = HashMap::new();
                inputs.extend(x_map);
                inputs.extend(y_map);

                let outputs = simulate_circuit(circuit, &inputs);
                let z_wire = format!("z{:02}", n);
                let got = *outputs.get(&z_wire).unwrap_or(&0);
                let expected = (x + y + c) % 2;
                if got != expected {
                    // println!("Bit {} failed for x={},y={},c={} expected={} got={}", n, x, y, c, expected, got);
                    return false;
                }
            }
        }
    }
    true
}

/// build "xNN", "yNN", "zNN" wire names
fn make_wire(prefix: &str, n: i32) -> String {
    format!("{}{:02}", prefix, n)
}

fn find_wire(
    circuit: &HashMap<String, Gate>,
    op: Option<&str>,
    in1: Option<&str>,
    in2: Option<&str>,
) -> Option<Gate> {
    'outer: for gate in circuit.values() {
        // if op is specified, must match
        if let Some(required_op) = op {
            if gate.operator != required_op {
                continue 'outer;
            }
        }
        // if in1 is specified, must be either gate.in1 or gate.in2
        if let Some(required_in1) = in1 {
            if gate.in1 != required_in1 && gate.in2 != required_in1 {
                continue 'outer;
            }
        }
        // if in2 is specified, must also be among gate inputs
        if let Some(required_in2) = in2 {
            if gate.in1 != required_in2 && gate.in2 != required_in2 {
                continue 'outer;
            }
        }
        return Some(gate.clone());
    }
    None
}

/// swap the output wires of two gates in circuit
fn swap_gate_outputs(circuit: &mut HashMap<String, Gate>, wire_a: &str, wire_b: &str) {
    let mut gate_a = circuit.remove(wire_a).expect("Missing gate for wireA");
    let mut gate_b = circuit.remove(wire_b).expect("Missing gate for wireB");

    std::mem::swap(&mut gate_a.out, &mut gate_b.out);

    circuit.insert(gate_a.out.clone(), gate_a);
    circuit.insert(gate_b.out.clone(), gate_b);
}

/// fix the wiring for bit n
/// returns a list of two wire names that it swapped (or empty if no swap)
fn fix_bit_n(n: i32, circuit: &mut HashMap<String, Gate>) -> Vec<String> {
    //println!("Issue with bit n = {}", n);

    // prevand = find_wire(op="AND", in1=x(n-1), in2=y(n-1))
    let prev_and = find_wire(
        circuit,
        Some("AND"),
        Some(&make_wire("x", n-1)),
        Some(&make_wire("y", n-1)),
    );

    // prevxor = find_wire(op="XOR", in1=x(n-1), in2=y(n-1))
    let prev_xor = find_wire(
        circuit,
        Some("XOR"),
        Some(&make_wire("x", n-1)),
        Some(&make_wire("y", n-1)),
    );

    // m2 = find_wire(op="AND", in1=prevxor.out)
    let m2 = if let Some(px) = &prev_xor {
        find_wire(circuit, Some("AND"), Some(&px.out), None)
    } else { None };

    // m1 = find_wire(op="OR", in1=m2.out, in2=prevand.out)
    let m1 = if let (Some(m2_gate), Some(pa)) = (&m2, &prev_and) {
        find_wire(circuit, Some("OR"), Some(&m2_gate.out), Some(&pa.out))
    } else {
        None
    };

    // nxor = find_wire(op="XOR", in1=x(n), in2=y(n))
    let nxor = find_wire(
        circuit,
        Some("XOR"),
        Some(&make_wire("x", n)),
        Some(&make_wire("y", n)),
    );

    // zn = find_wire(op="XOR", in1=nxor.out, in2=m1.out)
    let mut zn = if let (Some(nx), Some(m1v)) = (&nxor, &m1) {
        find_wire(circuit, Some("XOR"), Some(&nx.out), Some(&m1v.out))
    } else {
        None
    };

    let mut to_swap: Vec<String> = Vec::new();

    // if zn is None
    //   let z_gate = circuit[ "zNN" ]
    //   to_swap = set(z_gate.ins) ^ set([nxor.out, m1.out])
    if zn.is_none() {
        let z_wire_name = make_wire("z", n);
        if let Some(z_gate) = circuit.get(&z_wire_name).cloned() {
            if let (Some(nx), Some(m1v)) = (&nxor, &m1) {
                // Current inputs:
                let cur_ins = vec![z_gate.in1.clone(), z_gate.in2.clone()]
                    .into_iter()
                    .collect::<HashSet<_>>();
                let desired_ins = vec![nx.out.clone(), m1v.out.clone()]
                    .into_iter()
                    .collect::<HashSet<_>>();
                let symdiff = cur_ins
                    .symmetric_difference(&desired_ins)
                    .cloned()
                    .collect::<Vec<_>>();
                to_swap = symdiff;
            }
            zn = Some(z_gate);
        }
    }

    // if we do have zn, check if zn.out != "zNN". If so, we swap those two wires:
    if let Some(ref real_zn) = zn {
        let expected_out = make_wire("z", n);
        if real_zn.out != expected_out {
            to_swap = vec![expected_out, real_zn.out.clone()];
        }
    }

    // if to_swap has exactly 2 wires, we perform swap_gate_outputs
    if to_swap.len() == 2 {
        //println!("Swapping wires: {:?} for bit n={}", to_swap, n);
        swap_gate_outputs(circuit, &to_swap[0], &to_swap[1]);
    }

    to_swap
}


pub fn part_two(input: &str) -> Option<String> {

    let (wires_input, gates_input) = input.split_once("\n\n").unwrap();
    let mut xy_wires: HashMap<String, u8> = HashMap::new();
    let mut circuit: HashMap<String, Gate> = HashMap::new();

    // parse the xNN/yNN wire values
    for line in wires_input.lines() {
        let split: Vec<&str> = line.split(':').collect();
        let wire_name = split[0].trim().to_string();
        let value: u8 = split[1].trim().parse().unwrap();
        xy_wires.insert(wire_name, value);
    }

    // parse gates
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

    let mut all_swaps = Vec::new();

    for bit in 0..=44 {
        if validate_bit(bit, &circuit) {
            continue;
        }
        let fix_result = fix_bit_n(bit as i32, &mut circuit);
        all_swaps.extend(fix_result);
    }

    all_swaps.sort();
    all_swaps.dedup();

    let answer = all_swaps.join(",");
    //println!("Swapped wires: {}", answer);

    Some(answer)
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
        assert_eq!(result, Some("".to_string()));
    }
}
