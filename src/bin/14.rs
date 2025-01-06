advent_of_code::solution!(14);
use flate2::write::ZlibEncoder;
use flate2::Compression;
use regex::Regex;
use std::io::Write;
use std::{thread, time::Duration};

struct Robot {
    start_position: (i32, i32),
    velocity: (i32, i32),
}

fn visualize_robots(robots: &[Robot], width: usize, height: usize, max_seconds: usize) {
    for t in 0..=max_seconds {
        let mut grid = vec![vec!['.'; width]; height];

        for &(x, y) in &positions_at_time(robots, t as i32) {
            grid[y as usize][x as usize] = '#';
        }

        print!("\x1B[2J\x1B[1;1H");

        println!("Time = {}:", t);
        for row in 0..height {
            let row_string: String = grid[row].iter().collect();
            println!("{}", row_string);
        }
        println!();

        // Sleep for 20 ms
        thread::sleep(Duration::from_millis(20));
    }
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

    (q1 * q2 * q3 * q4) as usize
}

fn positions_at_time(robots: &[Robot], t: i32) -> Vec<(usize, usize)> {
    robots
        .iter()
        .map(|r| {
            let mut x = r.start_position.0 + r.velocity.0 * t;
            let mut y = r.start_position.1 + r.velocity.1 * t;

            x = ((x % 101) + 101) % 101;
            y = ((y % 103) + 103) % 103;

            (x as usize, y as usize)
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let robots = parse_input(input);
    let final_positions = positions_at_time(&robots, 100);
    let quadrant_product = quadrant_operation(final_positions);
    Some(quadrant_product)
}

fn render_grid(positions: &[(usize, usize)], width: usize, height: usize) -> String {
    let mut grid = vec![vec!['.'; width]; height];
    for &(x, y) in positions {
        grid[y][x] = '#';
    }

    let mut output = String::new();
    for row in grid {
        let row_str: String = row.iter().collect();
        output.push_str(&row_str);
        output.push('\n');
    }
    output
}

fn compressed_size(s: &str) -> usize {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());
    encoder.write_all(s.as_bytes()).unwrap();
    let compressed_data = encoder.finish().unwrap();
    compressed_data.len()
}

fn find_best_time_for_pattern(robots: &[Robot], max_time: i32) -> i32 {
    let mut best_time = 0;
    let mut best_size = usize::MAX;

    for t in (0..=max_time).rev() {
        let pos = positions_at_time(robots, t);
        let rendered = render_grid(&pos, 101, 103);
        let size = compressed_size(&rendered);

        if size < best_size {
            if best_size != usize::MAX && size * 100 <= best_size * 85 {
                best_size = size;
                best_time = t;
                break;
            } else {
                best_size = size;
                best_time = t;
            }
        }
    }

    //println!("Final best_size: {best_size}");
    best_time
}

pub fn part_two(input: &str) -> Option<u32> {
    let robots = parse_input(input);
    // visualize_robots(&robots, 101, 103, 10000);  
    let max_time = 10_000;
    let best_time = find_best_time_for_pattern(&robots, max_time);

    //println!("tree time = {best_time}");

    //let positions = positions_at_time(&robots, best_time);
    //let rendered = render_grid(&positions, 101, 103);
    //println!("rendering time {best_time}:\n{rendered}");
    Some(best_time as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<usize> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(232253028));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8179));
    }
}
