use std::collections::HashSet;
advent_of_code::solution!(6);

fn rotate_clockwise((dr, dc): (i32, i32)) -> (i32, i32) {
    match (dr, dc) {
        (-1, 0) => (0, 1),  // Up -> Right
        (0, 1) => (1, 0),   // Right -> Down
        (1, 0) => (0, -1),  // Down -> Left
        (0, -1) => (-1, 0), // Left -> Up
        _ => (dr, dc),
    }
}

fn guard_movement(
    grid: &[Vec<char>],
    start_pos: (i32, i32),
    start_dir: (i32, i32),
    track_loop: bool,
) -> (Vec<((i32, i32), (i32, i32))>, bool) {
    let rows = grid.len() as i32;
    let cols = if rows > 0 { grid[0].len() as i32 } else { 0 };
    let mut position = start_pos;
    let mut direction = start_dir;
    let mut path = Vec::new();
    path.push((position, direction));
    let mut visited_states = HashSet::new();
    if track_loop {
        visited_states.insert((position.0, position.1, direction.0, direction.1));
    }

    loop {
        let next_r = position.0 + direction.0;
        let next_c = position.1 + direction.1;

        if next_r < 0 || next_r >= rows || next_c < 0 || next_c >= cols {
            return (path, false);
        }

        // Check for obsctacles
        if grid[next_r as usize][next_c as usize] == '#' {
            // Rotate
            direction = rotate_clockwise(direction);
            continue;
        }

        // Move forward
        position = (next_r, next_c);
        path.push((position, direction));

        // Check if loop
        if track_loop {
            let state = (position.0, position.1, direction.0, direction.1);
            if !visited_states.insert(state) {
                return (path, true);
            }
        }

    }
}

/// Find the starting position and direction of the guard
fn find_start(grid: &[Vec<char>]) -> ((i32, i32), (i32, i32)) {
    let rows = grid.len() as i32;
    let cols = if rows > 0 { grid[0].len() as i32 } else { 0 };
    for r in 0..rows {
        for c in 0..cols {
            if grid[r as usize][c as usize] == '^' {
                return ((r, c), (-1, 0))
            }
        }
    }
    ((0, 0), (-1, 0))
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let (start_pos, start_dir) = find_start(&grid);

    // Simulate without loop tracking to get og path
    let (path, _) = guard_movement(&grid, start_pos, start_dir, false);
    let visited: HashSet<(i32, i32)> = path.into_iter().map(|(pos, _)| pos).collect();
    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let (start_pos, start_dir) = find_start(&grid);

    let rows = grid.len() as i32;
    let cols = if rows > 0 {
        grid[0].len() as i32
    } else {
        0
    };

    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if (r, c) == start_pos {
                continue;
            }
            if grid[r as usize][c as usize] == '#' {
                continue;
            }

            // Create a modified grid
            let mut grid_clone = grid.clone();
            grid_clone[r as usize][c as usize] = '#';

            // Run simulation with loop detection, starting from original start and direction
            let (_, loop_detected) = guard_movement(&grid_clone, start_pos, start_dir, true);
            if loop_detected {
                count += 1;
            }
        }
    }

    Some(count)
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
