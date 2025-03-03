use std::collections::HashMap;

advent_of_code::solution!(18);

/* use std::fs::File;
use std::io::Write; */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PositionInfo {
    score: u32,
    relaxed: bool,
    finished: bool,
}

pub fn part_one(input: &str) -> Option<u32> {
    let byte_positions: Vec<(u32, u32)> = input
        .lines()
        .map(|l| {
            let byte_x: u32 = l.split(",").nth(0).unwrap().parse().unwrap();
            let byte_y: u32 = l.split(",").nth(1).unwrap().parse().unwrap();
            (byte_x, byte_y)
        })
        .collect();

    let starting_position: (usize, usize) = (0, 0);

    // position, distance, relaxed, finished
    let mut distance_map: HashMap<(usize, usize), PositionInfo> = HashMap::new();

    distance_map.insert(
        starting_position,
        PositionInfo {
            score: 0,
            relaxed: false,
            finished: false,
        },
    );

    let rows = 71;
    let cols = 71;

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; cols]; rows];

    grid[starting_position.0][starting_position.1] = '0';

    for n in 0..1024 {
        let r = byte_positions[n].0 as usize;
        let c = byte_positions[n].1 as usize;
        grid[r][c] = '#';
    }
    let is_walkable = |ch: char| ch == '.';

    // loop -> advance positions in distance_map, with the minimum score and that are not 'relaxed'
    // check for next possible position, and add to distance_map if possible, if not possible, position is "relaxed"
    // check if advanced position in distance_map, if score is less, replace position in distance_map
    // if replaced or added, add distance number to grid position

    loop {
        let next_pos = distance_map
            .iter()
            .filter(|(_, info)| !info.relaxed)
            .min_by_key(|(_, info)| info.score)
            .map(|(&key, _)| key)
            .unwrap();

        // break if finished
        if next_pos == (70, 70) {
            //println!("Reached destination at {:?}", next_pos);
            break;
        }

        let current = distance_map.get(&next_pos).unwrap();
        let mut directions = Vec::new();

        // up
        if next_pos.0 > 0 {
            directions.push((next_pos.0 - 1, next_pos.1));
        }

        // down
        if next_pos.0 + 1 < rows {
            directions.push((next_pos.0 + 1, next_pos.1));
        }

        // left
        if next_pos.1 > 0 {
            directions.push((next_pos.0, next_pos.1 - 1));
        }

        // right
        if next_pos.1 + 1 < cols {
            directions.push((next_pos.0, next_pos.1 + 1));
        }

        let mut next_positions: Vec<((usize, usize), PositionInfo)> = vec![];

        // evaluate neighbors
        for (r, c) in directions {
            if r < rows && c < cols {
                let next_ch = grid[r][c];
                if is_walkable(next_ch) {
                    let is_finished = r == 70 && c == 70;
                    next_positions.push((
                        (r, c),
                        PositionInfo {
                            score: current.score + 1,
                            relaxed: false,
                            finished: is_finished,
                        },
                    ));
                }
            }
        }
        // if no valid next position relax the current one
        if next_positions.len() == 0 {
            if let Some(current_info) = distance_map.get_mut(&next_pos) {
                current_info.relaxed = true;
            }
        }

        // insert or update distance_map
        for (pos, info) in next_positions {
            distance_map
                .entry(pos)
                .and_modify(|existing| {
                    // if new score is less, update map
                    if info.score < existing.score {
                        existing.score = info.score;
                        existing.relaxed = info.relaxed;
                        existing.finished = info.finished;
                    }
                })
                .or_insert(info);
        }

        // set current_pos to relaxed (all neighbors processed)
        if let Some(current_info) = distance_map.get_mut(&next_pos) {
            current_info.relaxed = true;
        }
    }

    return Some(distance_map.get(&(70, 70)).unwrap().score);
}

fn path_reachable(
    byte_positions: &[(u32, u32)],
    byte_limit: usize,
    rows: usize,
    cols: usize,
    starting_position: (usize, usize)
) -> bool {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; cols]; rows];
    let mut distance_map: HashMap<(usize, usize), PositionInfo> = HashMap::new();

    distance_map.insert(
        starting_position,
        PositionInfo {
            score: 0,
            relaxed: false,
            finished: false,
        },
    );

    grid[starting_position.0][starting_position.1] = '0';

    for n in 0..=byte_limit.min(byte_positions.len()) {
        let r = byte_positions[n].0 as usize;
        let c = byte_positions[n].1 as usize;
        if r < rows && c < cols {
            grid[r][c] = '#';
        }
    }

    let is_walkable = |ch: char| ch == '.';

    loop {
        let next_pos = match distance_map
            .iter()
            .filter(|(_, info)| !info.relaxed)
            .min_by_key(|(_, info)| info.score)
            .map(|(&key, _)| key) {
                Some(pos) => pos,
                None => break,
            };

        if next_pos == (70, 70) {
            return true;
        }

        let current = distance_map.get(&next_pos).unwrap();
        let mut directions = Vec::new();

        // up
        if next_pos.0 > 0 {
            directions.push((next_pos.0 - 1, next_pos.1));
        }
        // down
        if next_pos.0 + 1 < rows {
            directions.push((next_pos.0 + 1, next_pos.1));
        }
        // left
        if next_pos.1 > 0 {
            directions.push((next_pos.0, next_pos.1 - 1));
        }
        // right
        if next_pos.1 + 1 < cols {
            directions.push((next_pos.0, next_pos.1 + 1));
        }

        let mut next_positions: Vec<((usize, usize), PositionInfo)> = vec![];

        // evaluate neighbors
        for (r, c) in directions {
            let next_ch = grid[r][c];
            if is_walkable(next_ch) {
                let is_finished = (r, c) == (rows - 1, cols - 1);
                next_positions.push((
                    (r, c),
                    PositionInfo {
                        score: current.score + 1,
                        relaxed: false,
                        finished: is_finished,
                    },
                ));
            }
        }

        // break the loop if no valid next position
        if next_positions.is_empty() {
            if let Some(current_info) = distance_map.get_mut(&next_pos) {
                current_info.relaxed = true;
            }
        }

        // insert or update distance_map
        for (pos, info) in next_positions {
            distance_map
                .entry(pos)
                .and_modify(|existing| {
                    if info.score < existing.score {
                        existing.score = info.score;
                        existing.relaxed = info.relaxed;
                        existing.finished = info.finished;
                    }
                })
                .or_insert(info);
        }

        // set current_pos to relaxed (all neighbors processed)
        if let Some(current_info) = distance_map.get_mut(&next_pos) {
            current_info.relaxed = true;
        }
    }

    false
}

pub fn part_two(input: &str) -> Option<String> {
    let byte_positions: Vec<(u32, u32)> = input
        .lines()
        .map(|l| {
            let byte_x: u32 = l.split(',').nth(0).unwrap().parse().unwrap();
            let byte_y: u32 = l.split(',').nth(1).unwrap().parse().unwrap();
            (byte_x, byte_y)
        })
        .collect();

    let rows = 71;
    let cols = 71;
    let starting_position = (0usize, 0usize);

    // path reachable up to 1024 bytes
    let mut low = 1024;
    let mut high = byte_positions.len();
    let mut first_blocking_byte = None;

    // Binary Search
    while low < high {
        let mid = low + (high - low) / 2;
        if path_reachable(&byte_positions, mid, rows, cols, starting_position) {
            // path is still reachable until mid, so search above
            low = mid + 1;
        } else {
            // path is blocked with mid bytes, search lower half
            first_blocking_byte = Some(mid);
            high = mid;
        }
    }

    if let Some(index) = first_blocking_byte {
        let (x, y) = byte_positions[index];
        return Some(format!("{},{}", x, y));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(272));
    }

    #[test]
    fn test_part_two() {
        let result: Option<String> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("16,44".to_string()));
    }
}

/* {
    let mut file =
        File::create("/Users/dmay/projects/advent-of-rust-2024/data/inputs/18_1024_bytes.txt")
            .expect("Could not create output file.");
    for row in &grid {
        let line: String = row.iter().collect();
        writeln!(file, "{}", line).unwrap();
    }
} */
