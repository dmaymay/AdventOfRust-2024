advent_of_code::solution!(13);

#[derive(Debug)]
struct Machine {
    button_a: (i128, i128),
    button_b: (i128, i128),
    prize: (i128, i128),
}

fn parse_machine_inputs(input: &str, large_prize: bool) -> Vec<Machine> {
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
                    .parse::<i128>()
                    .ok()?,
                lines[0]
                    .split(',')
                    .nth(1)?
                    .split('+')
                    .nth(1)?
                    .trim()
                    .parse::<i128>()
                    .ok()?,
            );

            let button_b = (
                lines[1]
                    .split(',')
                    .nth(0)?
                    .split('+')
                    .nth(1)?
                    .trim()
                    .parse::<i128>()
                    .ok()?,
                lines[1]
                    .split(',')
                    .nth(1)?
                    .split('+')
                    .nth(1)?
                    .trim()
                    .parse::<i128>()
                    .ok()?,
            );

            let mut prize = (
                lines[2]
                    .split(',')
                    .nth(0)?
                    .split('=')
                    .nth(1)?
                    .trim()
                    .parse::<i128>()
                    .ok()?,
                lines[2]
                    .split(',')
                    .nth(1)?
                    .split('=')
                    .nth(1)?
                    .trim()
                    .parse::<i128>()
                    .ok()?,
            );
            if large_prize {
                prize.0 += 10000000000000;
                prize.1 += 10000000000000;
            }

            Some(Machine {
                button_a,
                button_b,
                prize,
            })
        })
        .collect()
}

/* fn find_solutions_iterative(machine: &Machine) -> Vec<(i128, i128)> {
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
} */

// using Cramers rule
fn find_solutions_cramer(machine: &Machine) -> Vec<(i128, i128)> {
    let mut solutions = Vec::new();

    let (ax, ay) = machine.button_a;
    let (bx, by) = machine.button_b;
    let (px, py) = machine.prize;

    // The matrix is something like this:

    // |TA * ax + TB * bx | px
    // |TA * ay + TB * by | py

    // in our case we know ax,ay,bx,by and we are trying to solve for TA,TB
    // determinant of the matrix is ax * bx - ay * by

    let determinant = ax * by - ay * bx;

    if determinant != 0 {
        // determinant for TA,TB
        let det_a = px * by - py * bx;
        let det_b = ax * py - ay * px;

        // check if divisible (whole number answers)
        // TA will be determinant / det_a
        if det_a % determinant == 0 && det_b % determinant == 0 {
            let ta = det_a / determinant;
            let tb = det_b / determinant;
    
            // check that TA and TB are positive
            if ta > 0 && tb > 0 {
                solutions.push((ta, tb));
            }
        }
    }


    solutions
}

// unneccesary because there will only be one solution (or none)
fn count_tokens(solutions: Vec<(i128, i128)>) -> i128 {
    match solutions.len() {
        1 => solutions[0].0 * 3 + solutions[0].1,
        0 => 0,
        _ => solutions.iter().map(|s| s.0 * 3 + s.1).min().unwrap_or(0),
    }
}

pub fn part_one(input: &str) -> Option<i128> {
    let machines = parse_machine_inputs(input, false);
    let mut token_sum = 0;
    for machine in machines {
        let solutions = find_solutions_cramer(&machine);
        token_sum += count_tokens(solutions);
    }

    Some(token_sum)
}

pub fn part_two(input: &str) -> Option<i128> {
    let machines = parse_machine_inputs(input, true);
    let mut token_sum = 0;
    for machine in machines {
        let solutions = find_solutions_cramer(&machine);
        token_sum += count_tokens(solutions);
    }

    Some(token_sum)
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
        assert_eq!(result, Some(875318608908));
    }
}
