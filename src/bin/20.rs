advent_of_code::solution!(20);

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
                    /* println!(
                        "Cheat from position {} to {} saves {} moves.",
                        start, end, saved_moves
                    ); */
                    cheat_moves+=1;
                   }
                }
            }
        }
    }
    Some(cheat_moves)
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
