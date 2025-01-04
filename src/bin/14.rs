advent_of_code::solution!(14);
use regex::Regex;

struct Robot {
    start_position: (i32, i32),
    velocity: (i32, i32),
}

fn movement(robot: Robot) -> (usize, usize) {
    let mut x = robot.start_position.0;
    let mut y = robot.start_position.1;

    for _ in 0..100 {
        x += robot.velocity.0;
        y += robot.velocity.1;

        // check if in horizontal bounds
        // if out of bounds -> x - max_x + velocity.x
        if x < 0 {
            x += 101;
        } else if x > 100 {
            x -= 101;
        }

        // check if in vertical bounds
        // if out of bounds -> y - max_y + velocity.
        if y < 0 {
            y += 103;
        } else if y > 102 {
            y -= 103;
        }
    }
    (x.try_into().unwrap(), y.try_into().unwrap())
}

fn parse_input(input: &str) -> Vec<Robot> {
    let mut robots: Vec<Robot> = Vec::new();
    for line in input.lines() {
        let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
        if let Some(caps) = re.captures(line) {
            let px: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
            let py: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
            let vx: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
            let vy: i32 = caps.get(4).unwrap().as_str().parse().unwrap();

            robots.push(Robot {
                start_position: (px, py),
                velocity: (vx, vy),
            })
        }
    }
    robots
}

fn quadrant_operation(positions: Vec<(usize, usize)>) -> usize {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for pos in positions {
        match pos {
            (x, y) if x < 50 && y < 51 => q1 += 1, // upper-left quadrant
            (x, y) if x > 50 && y < 51 => q2 += 1, // upper-right quadrant
            (x, y) if x < 50 && y > 51 => q3 += 1, // lower-left quadrant
            (x, y) if x > 50 && y > 51 => q4 += 1, // lower-right quadrant
            _ => (),
        }
    }

    q1 * q2 * q3 * q4
}

pub fn part_one(input: &str) -> Option<usize> {
    let robots = parse_input(input);
    let mut final_positions: Vec<(usize, usize)> = Vec::new();

    for robot in robots {
        final_positions.push(movement(robot))
    }

    let quadrant_product = quadrant_operation(final_positions);
    Some(quadrant_product)
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
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
