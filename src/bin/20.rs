advent_of_code::solution!(20);
use std::collections::{HashMap, VecDeque};

struct Grid {
    space: Vec<bool>,
    start: Option<usize>,
    end: Option<usize>,
    path: Vec<usize>,
    cols: usize,
}

impl Grid {
    fn next_move(&self, current_position: usize) -> Option<usize> {
        // left
        if current_position % self.cols > 0 && self.space[current_position - 1] {
            Some(current_position - 1)
        // right
        } else if current_position % self.cols < self.cols - 1 && self.space[current_position + 1] {
            Some(current_position + 1)
        // up
        } else if current_position >= self.cols && self.space[current_position - self.cols] {
            Some(current_position - self.cols)
        // down
        } else if current_position < self.cols * (self.cols - 1) {
            Some(current_position + self.cols)
        } else {
            None
        }
    }

    fn cheat(&self, current_position: usize) -> [Option<usize>; 4] {
        [
            // left
            if current_position % self.cols >= 2 {
                let intermediate = current_position - 1;
                let target = current_position - 2;
                if !self.space[intermediate] && self.space[target] {
                    Some(target)
                } else {
                    None
                }
            } else {
                None
            },
            // right
            if current_position % self.cols <= self.cols - 3 {
                let intermediate = current_position + 1;
                let target = current_position + 2;
                if !self.space[intermediate] && self.space[target] {
                    Some(target)
                } else {
                    None
                }
            } else {
                None
            },
            // up
            if current_position >= 2 * self.cols {
                let intermediate = current_position - self.cols;
                let target = current_position - 2 * self.cols;
                if !self.space[intermediate] && self.space[target] {
                    Some(target)
                } else {
                    None
                }
            } else {
                None
            },
            // down
            if current_position < self.space.len() - 2 * self.cols {
                let intermediate = current_position + self.cols;
                let target = current_position + 2 * self.cols;
                if !self.space[intermediate] && self.space[target] {
                    Some(target)
                } else {
                    None
                }
            } else {
                None
            },
        ]
    }

    fn long_cheat(&self, cheat_space:Vec<bool>, current_position: usize) -> Vec<(usize, usize)> {
        let mut visited = vec![false; self.space.len()];
        let mut queue = VecDeque::new();
        let mut targets = Vec::new();

        visited[current_position] = true;
        queue.push_back((current_position, 0));

        while let Some((pos, moves)) = queue.pop_front() {
            if moves >= 20 {
                continue;
            }

            let directions = [
                // left
                if pos % self.cols > 0 {
                    Some(pos - 1)
                } else {
                    None
                },
                // right
                if pos % self.cols < self.cols - 1 {
                    Some(pos + 1)
                } else {
                    None
                },
                // up
                if pos >= self.cols {
                    Some(pos - self.cols)
                } else {
                    None
                },
                // down
                if pos < self.space.len() - self.cols {
                    Some(pos + self.cols)
                } else {
                    None
                },
            ];

            for &next_pos_opt in directions.iter() {
                if let Some(next_pos) = next_pos_opt {
                    if !visited[next_pos] {
                        visited[next_pos] = true;
                        let next_moves = moves + 1;

                        if cheat_space[next_pos] {
                            if next_moves > 1 {
                                targets.push((next_pos, next_moves));
                            }
                        }
                        // add next position to queue
                        queue.push_back((next_pos, next_moves));
                    }
                }
            }
        }

        targets
    }

    fn position_to_index_map(&self) -> HashMap<usize, usize> {
        let mut map = HashMap::new();
        for (idx, &pos) in self.path.iter().enumerate() {
            map.insert(pos, idx);
        }
        map
    }
}

fn create_space_map_with_positions(input: &str, rows: usize, cols: usize) -> Grid {
    let mut space = vec![false; rows * cols];
    let mut start = None;
    let mut end = None;

    for (row_index, line) in input.lines().enumerate() {
        for (col_index, ch) in line.chars().enumerate() {
            let index = row_index * cols + col_index;
            match ch {
                '.' | 'S' | 'E' => {
                    space[index] = true;
                    if ch == 'S' {
                        start = Some(index);
                    }
                    if ch == 'E' {
                        end = Some(index);
                    }
                }
                _ => {}
            }
        }
    }
    Grid {
        space,
        start,
        end,
        path: vec![],
        cols,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    // traverse path normally, get number of moves and store positions
    // for every position check if if two consecutive steps in any direction
    // are a 'wall' and a non traversed empty space.
    // get number of skipped positions

    let rows = input.lines().collect::<Vec<&str>>().len();
    let cols = input.lines().next().map(|line| line.len()).unwrap_or(0);
    let mut grid = create_space_map_with_positions(input, rows, cols);
    let mut current_position = grid.start;
    let end_position = grid.end;

    grid.path.push(current_position?);

    // store starting position and end position of cheats
    let mut possible_cheats: Vec<(usize, usize)> = vec![];

    loop {
        let cheat_targets = grid.cheat(current_position?);
        for &cheat_pos in &cheat_targets {
            if let Some(target) = cheat_pos {
                possible_cheats.push((current_position?, target));
            }
        }
        current_position = grid.next_move(current_position?);
        grid.path.push(current_position?);
        grid.space[current_position?] = false;

        if current_position == end_position {
            break;
        }
    }

    let mut cheat_moves: u32 = 0;
    // for every cheat move check beginning move in grid.path and target move in grid.path, target index - beginning index is saved moves
    for (start, end) in &possible_cheats {
        if let Some(start_idx) = grid.path.iter().position(|&x| x == *start) {
            if let Some(end_idx) = grid.path.iter().position(|&x| x == *end) {
                if end_idx > start_idx {
                    let normal_moves = (end_idx - start_idx) as u32;
                    let saved_moves = normal_moves - 2;
                    if saved_moves >= 100 {
                        cheat_moves += 1;
                    }
                }
            }
        }
    }
    Some(cheat_moves)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rows = input.lines().collect::<Vec<&str>>().len();
    let cols = input.lines().next().map(|line| line.len()).unwrap_or(0);
    let mut grid = create_space_map_with_positions(input, rows, cols);
    let mut current_position = grid.start;
    let end_position = grid.end;
    let cheat_space = grid.space.clone();
    grid.path.push(current_position?);
    grid.space[current_position?] = false;
    
    loop {
        current_position = grid.next_move(current_position?);
        grid.path.push(current_position?);
        grid.space[current_position?] = false;

        if current_position == end_position {
            break;
        }
    }

    let position_map = grid.position_to_index_map();
    let mut cheat_moves = 0;

    for (start_idx, &start_pos) in grid.path.iter().enumerate() {
        let cheat_targets = grid.long_cheat(cheat_space.clone(), start_pos);
        for (target, moves) in cheat_targets {
            if let Some(&target_idx) = position_map.get(&target) {
                if target_idx > start_idx {
                    let saved_moves = (target_idx - start_idx) as i64 - (moves as i64);
                    if saved_moves >= 100 {
                        cheat_moves += 1;
                    }
                }
            }
        }
    }
    Some(cheat_moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
