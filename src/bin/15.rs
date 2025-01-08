advent_of_code::solution!(15);
use std::fs::File;
use std::io::Write;

fn movement(
    mut map: Vec<Vec<char>>,
    movements: Vec<char>,
    start_position: (usize, usize),
    rows: usize,
    cols: usize,
) -> Vec<Vec<char>> {
    let mut pos: (usize, usize) = start_position;

    for mov in movements {
        match mov {
            '^' => {
                // Up is (dr, dc) = (-1, 0)
                map = update_map(map, &mut pos, -1, 0, rows, cols);
                
            }
            'v' => {
                // Down (dr, dc) = (1, 0)
                map = update_map(map, &mut pos, 1, 0, rows, cols);
            }
            '<' => {
                // Left (dr, dc) = (0, -1)
                map = update_map(map, &mut pos, 0, -1, rows, cols);
            }
            '>' => {
                // Right (dr, dc) = (0, 1)
                map = update_map(map, &mut pos, 0, 1, rows, cols);
                /* return map; */
            }
            _ => panic!("wrong input"),
        };
    }

    map
}

fn update_map(
    mut map: Vec<Vec<char>>,
    pos: &mut (usize, usize),
    dr: isize,
    dc: isize,
    rows: usize,
    cols: usize,
) -> Vec<Vec<char>> {
    let mut things_ahead: Vec<char> = Vec::new();

    // Compute the cell in front of the robot
    let next_r = (pos.0 as isize + dr) as usize;
    let next_c = (pos.1 as isize + dc) as usize;

    /*     // Safety check for bounds
       if next_r >= rows || next_c >= cols {
           return map;
       }
    */
    // 1) If it’s empty, move the robot
    if map[next_r][next_c] == '.' {
        map[pos.0][pos.1] = '.';
        pos.0 = next_r;
        pos.1 = next_c;
        map[pos.0][pos.1] = '@';
        return map;
    }
    // 2) If it’s a wall, do nothing
    else if map[next_r][next_c] == '#' {
        return map;
    }
    // 3) Otherwise, we have at least one box ('O') in front, so scan
    else {
        let direction_vector: Vec<usize> = if dr < 0 {
            // UP: scan upwards
            (0..pos.0).rev().collect()
        } else if dr > 0 {
            // DOWN: scan down
            ((pos.0 + 1)..rows).collect()
        } else if dc < 0 {
            // LEFT: scan left
            (0..pos.1).rev().collect()
        } else {
            // RIGHT: scan right
            ((pos.1 + 1)..cols).collect()
        };

        for idx in direction_vector {
            // Determine if we’re scanning rows (dr != 0) or columns (dc != 0)
            let object = if dr != 0 {
                map[idx][pos.1]
            } else {
                map[pos.0][idx]
            };
            println!("{:?} ", object);
            if object == '#' {
                // Box chain hits a wall -> stop
                break;
            } else if object == '.' {
                // Found an empty space -> place all boxes
                for n in 0..things_ahead.len() {
                    println!("{} {}", pos.0, pos.1);
                    let new_r = (pos.0 as isize + ((n as isize + 1) * dr) + dr) as usize;
                    let new_c = (pos.1 as isize + ((n as isize + 1) * dc) + dc) as usize;
                    println!("{} {}", new_r, new_c);
                    println!("{:?} ", things_ahead);
                    map[new_r][new_c] = 'O';
                }

                // Move robot one step
                map[pos.0][pos.1] = '.';
                println!("{} {}", pos.0, pos.1);
                pos.0 = (pos.0 as isize + dr) as usize;
                pos.1 = (pos.1 as isize + dc) as usize;
                map[pos.0][pos.1] = '@';

                println!("{} {}", pos.0, pos.1);
                break;
            } else if object == 'O' {
                // Add another box to the chain
                things_ahead.push(object);
                println!("found O {:?} ", things_ahead);
            }
        }
    }

    map
}

fn calculate_box_gps(map: &[Vec<char>]) -> u32 {
    let mut sum = 0;
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] == 'O' {
                sum += (r as u32) * 100 + (c as u32);
            }
        }
    }
    sum
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid_input, movement_input) = input.split_once("\n\n").unwrap();
    let movements: Vec<char> = movement_input
        .lines()
        .flat_map(|line| line.chars())
        .collect();

    let map: Vec<Vec<char>> = grid_input.lines().map(|l| l.chars().collect()).collect();
    let rows = map.len();
    let cols = map.get(0)?.len();
    let mut starting_position: (usize, usize) = (0, 0);

    for x in 0..rows {
        for y in 0..cols {
            if map[x][y] == '@' {
                starting_position = (x, y);
                break;
            }
        }
    }

    let finished_map = movement(map, movements, starting_position, rows, cols);

    {
        let mut file =
            File::create("/Users/dmay/projects/advent-of-rust-2024/data/inputs/15_output.txt")
                .expect("Could not create output file.");

        for row in &finished_map {
            // Convert the row's characters to a string.
            let line: String = row.iter().collect();
            // Write it to the file, followed by a newline.
            writeln!(file, "{}", line).expect("Could not write line to file.");
        }
    }

    let gps_sum = calculate_box_gps(&finished_map);

    Some(gps_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

/*                 {
    let mut file = File::create(
        "/Users/dmay/projects/advent-of-rust-2024/data/inputs/15_output.txt",
    )
    .expect("Could not create output file.");

    for row in &map {
        // Convert the row's characters to a string.
        let line: String = row.iter().collect();
        // Write it to the file, followed by a newline.
        writeln!(file, "{}", line).expect("Could not write line to file.");
    }
} */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

/* fn update_map(
    mut map: Vec<Vec<char>>,
    pos: &mut (usize, usize),
    dr: isize,
    dc: isize,
    rows: usize,
    cols: usize,
) -> Vec<Vec<char>> {
    ****it could be as simple as doing these changes
if map[pos.0 + dr][pos.1 + dc] == '.' {
    map[pos.0][pos.1] = '.';
    pos.0 += dr;
    pos.1 += dc;
    map[pos.0][pos.1] = '@';
    continue;
} else if map[pos.0 + dr][pos.1 + dc] == '#' {
    continue;
} else {
    for x in (0..pos.0).rev() {
        let object = map[x][pos.1];

        if object == '#' {
            break;
        } else if object == '.' {
            println!("things_ahead: {:?}", things_ahead);
            for n in 0..things_ahead.len()-1 {
                map[pos.0 + ( n * dr)][pos.1 + ( n * dc)] = 'O';
            }

            map[pos.0][pos.1] = '.';
            pos.0 += dr;
            pos.1 += dc;
            map[pos.0][pos.1] = '@';
            break;
        } else if object == 'O' {
            things_ahead.push(object);
        }
    }
} */
