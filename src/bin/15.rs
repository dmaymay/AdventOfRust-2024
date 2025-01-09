advent_of_code::solution!(15);
// for printing map only
/* use std::fs::File;
use std::io::Write; */

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

    // compute next cell
    let next_r = (pos.0 as isize + dr) as usize;
    let next_c = (pos.1 as isize + dc) as usize;

    // move robot if empty
    if map[next_r][next_c] == '.' {
        map[pos.0][pos.1] = '.';
        pos.0 = next_r;
        pos.1 = next_c;
        map[pos.0][pos.1] = '@';
        return map;
    }
    // if wall do nothing
    else if map[next_r][next_c] == '#' {
        return map;
    }
    // box ahead
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
            // determine if we’re scanning rows (dr != 0) or columns (dc != 0)
            let object = if dr != 0 {
                map[idx][pos.1]
            } else {
                map[pos.0][idx]
            };
            if object == '#' {
                // box hits a wall -> nothing happens
                break;
            } else if object == '.' {
                // found an empty space -> place all boxes
                for n in 0..things_ahead.len() {
                    let new_r = (pos.0 as isize + ((n as isize + 1) * dr) + dr) as usize;
                    let new_c = (pos.1 as isize + ((n as isize + 1) * dc) + dc) as usize;
                    map[new_r][new_c] = 'O';
                }

                // move robot one step
                map[pos.0][pos.1] = '.';
                pos.0 = (pos.0 as isize + dr) as usize;
                pos.1 = (pos.1 as isize + dc) as usize;
                map[pos.0][pos.1] = '@';
                break;
            } else if object == 'O' {
                // add box
                things_ahead.push(object);
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

    let gps_sum = calculate_box_gps(&finished_map);

    Some(gps_sum)
}

fn create_wide_map(original_map: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut wide_map = Vec::new();

    for row in original_map {
        let mut new_row = Vec::new();
        for &ch in row {
            match ch {
                '#' => {
                    // -> "##"
                    new_row.push('#');
                    new_row.push('#');
                }
                'O' => {
                    // -> "[]"
                    new_row.push('[');
                    new_row.push(']');
                }
                '.' => {
                    // -> ".."
                    new_row.push('.');
                    new_row.push('.');
                }
                '@' => {
                    // -> "@."
                    new_row.push('@');
                    new_row.push('.');
                }
                _ => (),
            }
        }
        wide_map.push(new_row);
    }

    wide_map
}

fn movement_wide(
    mut map: Vec<Vec<char>>,
    movements: Vec<char>,
    start_position: (usize, usize),
) -> Vec<Vec<char>> {
    let mut pos: (usize, usize) = start_position;

    for mov in movements {
        match mov {
            '^' => {
                map = update_map_wide(map, &mut pos, -1, 0);
            }
            'v' => {
                map = update_map_wide(map, &mut pos, 1, 0);
            }
            '<' => {
                map = update_map_wide(map, &mut pos, 0, -1);
            }
            '>' => {
                map = update_map_wide(map, &mut pos, 0, 1);
            }
            _ => panic!("wrong input"),
        };
    }

    map
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Box {
    pos: ((usize, usize), (usize, usize)),
}

fn update_map_wide(
    mut map: Vec<Vec<char>>,
    pos: &mut (usize, usize),
    dr: isize,
    dc: isize,
) -> Vec<Vec<char>> {
    let mut box_chain: Vec<Vec<Box>> = Vec::new();

    // compute next cell
    let next_r = (pos.0 as isize + dr) as usize;
    let next_c = (pos.1 as isize + dc) as usize;

    // move robot if empty
    if map[next_r][next_c] == '.' {
        map[pos.0][pos.1] = '.';
        pos.0 = next_r;
        pos.1 = next_c;
        map[pos.0][pos.1] = '@';
        return map;
    }
    // if wall do nothing
    else if map[next_r][next_c] == '#' {
        return map;
    }
    // box ahead
    else {
        if map[next_r][next_c] == '[' {
            box_chain.push(vec![Box {
                pos: ((next_r, next_c), (next_r, next_c + 1)),
            }]);
        } else if map[next_r][next_c] == ']' {
            box_chain.push(vec![Box {
                pos: ((next_r, next_c - 1), (next_r, next_c)),
            }]);
        } else {
            return map;
        }

        // add box levels until any adjacent '#' in current direction or all '.'

        loop {
            // loop through last level we added to the chain:
            let last_level = box_chain.last().unwrap();

            let wall_ahead = last_level.iter().any(|b| {
                let (r1, c1) = b.pos.0;
                let (r2, c2) = b.pos.1;
                let nr1 = (r1 as isize + dr) as usize;
                let nc1 = (c1 as isize + dc) as usize;
                let nr2 = (r2 as isize + dr) as usize;
                let nc2 = (c2 as isize + dc) as usize;
                map[nr1][nc1] == '#' || map[nr2][nc2] == '#'
            });
            if wall_ahead {
                return map;
            }

            let all_empty_ahead = last_level.iter().all(|b| {
                let (r1, c1) = b.pos.0;
                let (r2, c2) = b.pos.1;
                let nr1 = (r1 as isize + dr) as usize;
                let nc1 = (c1 as isize + dc) as usize;
                let nr2 = (r2 as isize + dr) as usize;
                let nc2 = (c2 as isize + dc) as usize;
                map[nr1][nc1] == '.' && map[nr2][nc2] == '.'
            });
            if all_empty_ahead {
                break; // end loop and push boxes
            }

            // if not all '.' or any '#', add more boxes
            let mut next_level = Vec::new();
            for b in last_level {
                let (r1, c1) = b.pos.0;
                let (r2, c2) = b.pos.1;
                let nr1 = (r1 as isize + dr) as usize;
                let nc1 = (c1 as isize + dc) as usize;
                let nr2 = (r2 as isize + dr) as usize;
                let nc2 = (c2 as isize + dc) as usize;

                // check for more boxes
                if map[nr1][nc1] == '[' {
                    next_level.push(Box {
                        pos: ((nr1, nc1), (nr1, nc1 + 1)),
                    });
                } else if map[nr1][nc1] == ']' {
                    next_level.push(Box {
                        pos: ((nr1, nc1 - 1), (nr1, nc1)),
                    });
                }

                if map[nr2][nc2] == '[' {
                    next_level.push(Box {
                        pos: ((nr2, nc2), (nr2, nc2 + 1)),
                    });
                } else if map[nr2][nc2] == ']' {
                    next_level.push(Box {
                        pos: ((nr2, nc2 - 1), (nr2, nc2)),
                    });
                }
            }

            // remove duplicates
            next_level.sort_by_key(|bx| bx.pos);
            next_level.dedup_by_key(|bx| bx.pos);

            if next_level.is_empty() {
                // no new boxes => break
                break;
            }
            if next_level == *last_level {
                // same boxes => break
                break;
            }

            box_chain.push(next_level);
        }

        // move entire chain forward

        // clear old positions
        for level in box_chain.iter() {
            for b in level {
                let ((r1, c1), (r2, c2)) = b.pos;
                map[r1][c1] = '.';
                map[r2][c2] = '.';
            }
        }
        // move the box chain
        for level in box_chain.iter() {
            for b in level {
                let ((r1, c1), (r2, c2)) = b.pos;
                let nr1 = (r1 as isize + dr) as usize;
                let nc1 = (c1 as isize + dc) as usize;
                let nr2 = (r2 as isize + dr) as usize;
                let nc2 = (c2 as isize + dc) as usize;
                // Place '[' and ']'
                map[nr1][nc1] = '[';
                map[nr2][nc2] = ']';
            }
        }

        // move robot
        map[pos.0][pos.1] = '.';
        pos.0 = (pos.0 as isize + dr) as usize;
        pos.1 = (pos.1 as isize + dc) as usize;
        map[pos.0][pos.1] = '@';
    }

    map
}

fn calculate_wide_box_gps(map: &[Vec<char>]) -> u32 {
    let mut sum = 0;
    for r in 0..map.len() {
        let row = &map[r];
        for c in 0..row.len() {
            if row[c] == '[' {
                sum += (r as u32) * 100 + (c as u32);
            }
        }
    }
    sum
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid_input, movement_input) = input.split_once("\n\n").unwrap();
    let movements: Vec<char> = movement_input
        .lines()
        .flat_map(|line| line.chars())
        .collect();

    let original_map: Vec<Vec<char>> = grid_input.lines().map(|l| l.chars().collect()).collect();

    let mut wide_map = create_wide_map(&original_map);

    // print wide map
    /*     {
        let mut file =
            File::create("/Users/dmay/projects/advent-of-rust-2024/data/inputs/15_wide_boy.txt")
                .expect("Could not create output file.");
        for row in &wide_map {
            let line: String = row.iter().collect();
            writeln!(file, "{}", line).unwrap();
        }
    } */

    let wide_rows = wide_map.len();
    let wide_cols = wide_map[0].len();

    let mut starting_position: (usize, usize) = (0, 0);
    'outer: for r in 0..wide_rows {
        for c in 0..wide_cols {
            if wide_map[r][c] == '@' {
                starting_position = (r, c);
                break 'outer;
            }
        }
    }

    wide_map = movement_wide(wide_map, movements, starting_position);

    // print finished map
    /*     {
        let mut file =
            File::create("/Users/dmay/projects/advent-of-rust-2024/data/inputs/15_output_wide.txt")
                .expect("Could not create output file.");
        for row in &wide_map {
            let line: String = row.iter().collect();
            writeln!(file, "{}", line).unwrap();
        }
    } */

    let gps_sum = calculate_wide_box_gps(&wide_map);

    Some(gps_sum)
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
        assert_eq!(result, Some(9021));
    }
}
