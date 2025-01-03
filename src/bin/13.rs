advent_of_code::solution!(13);

#[derive(Debug)]
struct Machine {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}

fn parse_machine_inputs(input: &str) -> Vec<Machine> {
    input
        .split("\n\n") // split input into machine blocks
        .filter_map(|machine| {
            let lines: Vec<&str> = machine.lines().collect();

            let button_a = (
                lines[0]
                    .split(',')
                    .nth(0)?
                    .split('+')
                    .nth(1)?
                    .trim()
                    .parse::<usize>()
                    .ok()?,
                lines[0]
                    .split(',')
                    .nth(1)?
                    .split('+')
                    .nth(1)?
                    .trim()
                    .parse::<usize>()
                    .ok()?,
            );

            let button_b = (
                lines[1]
                    .split(',')
                    .nth(0)?
                    .split('+')
                    .nth(1)?
                    .trim()
                    .parse::<usize>()
                    .ok()?,
                lines[1]
                    .split(',')
                    .nth(1)?
                    .split('+')
                    .nth(1)?
                    .trim()
                    .parse::<usize>()
                    .ok()?,
            );


            let prize = (
                lines[2]
                    .split(',')
                    .nth(0)?
                    .split('=')
                    .nth(1)?
                    .trim()
                    .parse::<usize>()
                    .ok()?,
                lines[2]
                    .split(',')
                    .nth(1)?
                    .split('=')
                    .nth(1)?
                    .trim()
                    .parse::<usize>()
                    .ok()?,
            );

            Some(Machine {
                button_a,
                button_b,
                prize,
            })
        })
        .collect()
}

fn find_solutions_iterative(machine: &Machine) -> Vec<(usize, usize)> {
    let mut solutions = Vec::new();

    let (ax, ay) = machine.button_a;
    let (bx, by) = machine.button_b;
    let (px, py) = machine.prize;

    let max_ta = std::cmp::max(px / ax, py / ay);
    let max_tb = std::cmp::max(px / bx, py / by);

    for ta in 0..=max_ta {
        for tb in 0..=max_tb {
            if ax * ta + bx * tb == px && ay * ta + by * tb == py {
                solutions.push((ta, tb));
            }
        }
    }

    solutions
} 

fn count_tokens(solutions: Vec<(usize, usize)>) -> usize {
    match solutions.len() {
        1 => solutions[0].0 * 3 + solutions[0].1,
        0 => 0,
        _ => solutions
            .iter()
            .map(|s| s.0 * 3 + s.1)
            .min()
            .unwrap_or(0)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let machines = parse_machine_inputs(input);
    let mut token_sum = 0;
    for machine in machines {
        let solutions = find_solutions_iterative(&machine);
        token_sum += count_tokens(solutions);
    }
    
    Some(token_sum)
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
