advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let (register_values, program_input) = input.split_once("\n\n").unwrap();
    let lines: Vec<&str> = register_values.lines().collect();
    let mut ra: u32 = lines[0].split(": ").nth(1).unwrap().parse().unwrap();
    let mut rb: u32 = lines[1].split(": ").nth(1).unwrap().parse().unwrap();
    let mut rc: u32 = lines[2].split(": ").nth(1).unwrap().parse().unwrap();

    let program_data: Vec<u32> = program_input
        .split(": ")
        .nth(1)
        .unwrap()
        .split(',')
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect();

    let mut output = String::new();
    let mut ip: usize = 0;

    while ip < program_data.len() {
        let opcode = program_data[ip];
        let operand = program_data[ip + 1];

        // combo operand
        let combo: u32 = match operand {
            0..=3 => operand,
            4 => ra,
            5 => rb,
            6 => rc,
            _ => operand
        };

        let mut jumped = false;

        match opcode {
            0 => { 
                // adv: divide A by 2^combo
                ra = ra >> combo 
            }
            1 => { 
                // bxl: XOR B with literal operand
                rb = rb ^ operand; 
            }
            2 => { 
                // bst: set B to combo operand modulo 8
                rb = combo & 0b111; 
            }
            3 => { 
                // jnz: jump if A is non-zero
                if ra != 0 {
                    ip = operand as usize;
                    jumped = true;
                }
            }
            4 => { 
                // bxc: XOR B and C
                rb = rb ^ rc; 
            }
            5 => {
                // out: output operand modulo 8
                output.push_str(&(combo & 0b111).to_string());
                output.push(',');
            }
            6 => { 
                // bdv: like adv but store result in B
                rb = ra >> combo
            }
            7 => { 
                // cdv: like adv but store result in C
                rc = ra >> combo
            }
            _ => panic!("unexpected opcode"),
        }

        // if no jump, move to next
        if !jumped {
            ip += 2;
        }
    }

    if output.ends_with(',') {
        output.pop();
    }

    Some(output)
}

pub fn part_two(input: &str) -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
