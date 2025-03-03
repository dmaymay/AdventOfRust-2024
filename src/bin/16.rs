advent_of_code::solution!(16);
use std::{collections::{HashMap, HashSet}, u32};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Path {
    direction: Direction,
    score: u32,
    //valid: bool,
    finish: bool,
    positions: Vec<(usize, usize)>,
    
}

fn next_position((r, c): (usize, usize), dir: Direction) -> (usize, usize) {
    match dir {
        Direction::Up => (r - 1, c),
        Direction::Down => (r + 1, c),
        Direction::Left => (r, c - 1),
        Direction::Right => (r, c + 1),
    }
}

fn turn_left(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Left,
        Direction::Left => Direction::Down,
        Direction::Down => Direction::Right,
        Direction::Right => Direction::Up,
    }
}

fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn possible_paths(grid: &Vec<Vec<char>>, path: Path) -> Vec<Path> {
    let mut paths = Vec::new();
    let pos = path.positions[path.positions.len() - 1];
    let dir = path.direction;
    let is_walkable = |ch: char| ch == '.' || ch == 'E';

    // forward
    let (fr, fc) = next_position(pos, dir);
    let forward_ch = grid[fr][fc];
    if is_walkable(forward_ch) {
        if !path.positions.contains(&(fr, fc)) {
            let mut forward_push = path.clone();
            forward_push.score += 1;
            forward_push.positions.push((fr, fc));
            if forward_ch == 'E' {
                forward_push.finish = true;
            }
            paths.push(forward_push);
        }
    }

    // left-turn + forward
    {
        let new_dir = turn_left(dir);
        let (lr, lc) = next_position(pos, new_dir);
        let left_ch = grid[lr][lc];
        if is_walkable(left_ch) {
            if !path.positions.contains(&(lr, lc)) {
                let mut left_path = path.clone();
                left_path.score += 1000;
                left_path.direction = new_dir;
                left_path.score += 1;
                left_path.positions.push((lr, lc));
                if left_ch == 'E' {
                    left_path.finish = true;
                }
                paths.push(left_path);
            }
        }
    }

    // right-turn + forward
    {
        let new_dir = turn_right(dir);
        let (rr, rc) = next_position(pos, new_dir);
        let right_ch = grid[rr][rc];
        if is_walkable(right_ch) {
            if !path.positions.contains(&(rr, rc)) {
                let mut right_path = path.clone();
                right_path.score += 1000;
                right_path.direction = new_dir;
                right_path.score += 1;
                right_path.positions.push((rr, rc));
                if right_ch == 'E' {
                    right_path.finish = true;
                }
                paths.push(right_path);
            }
        }
    }

    paths
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut starting_position: (usize, usize) = (0, 0);
    'outer: for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 'S' {
                starting_position = (r, c);
                break 'outer;
            }
        }
    }

    // check possible paths at pos (r,c)
    let mut valid_paths: Vec<Path> = Vec::new();
    let initial_path = Path {
        //moves: vec![Moves::Start],
        direction: Direction::Right,
        score: 0,
        //valid: true,
        finish: false,
        positions: vec![starting_position],
    };

    valid_paths.extend(possible_paths(&grid, initial_path));
    let mut global_map: HashMap<((usize, usize), Direction), u32> = HashMap::new();


    loop {
        let mut new_paths: Vec<Path> = vec![];

        for path in valid_paths {
            if path.finish {
                new_paths.push(path);
            } else {
                new_paths.extend(possible_paths(&grid, path));
            }
        }

        let (finished_paths, expanding_paths): (Vec<_>, Vec<_>) =
            new_paths.into_iter().partition(|p| p.finish);

        let mut pruned_expanding_paths = vec![];
        for path in expanding_paths {
            let current_pos = *path.positions.last().unwrap();
            let key = (current_pos, path.direction);
            let score = path.score;

            if let Some(&existing_score) = global_map.get(&key) {
                if existing_score <= score {
                    continue;
                } else {
                    pruned_expanding_paths.push(path);
                    // better score for this state -> update
                    global_map.insert(key, score);
                }
            } else {
                pruned_expanding_paths.push(path);
                // first time in this state -> update
                global_map.insert(key, score);
            }
        }

        valid_paths = finished_paths
            .into_iter()
            .chain(pruned_expanding_paths.into_iter())
            .collect();

        if valid_paths.is_empty() || valid_paths.iter().all(|p| p.finish) {
            break;
        }
    }

    if valid_paths.is_empty() {
        None
    } else {
        let min_score = valid_paths.iter().map(|p| p.score).min().unwrap();
        Some(min_score)
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut starting_position: (usize, usize) = (0, 0);
    'outer: for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 'S' {
                starting_position = (r, c);
                break 'outer;
            }
        }
    }

    // check possible paths at pos (r,c)
    let mut valid_paths: Vec<Path> = Vec::new();
    let initial_path = Path {
        direction: Direction::Right,
        score: 0,
        //valid: true,
        finish: false,
        positions: vec![starting_position],
    };

    valid_paths.extend(possible_paths(&grid, initial_path));
    let mut global_map: HashMap<((usize, usize), Direction), u32> = HashMap::new();
    
    loop {
        
        let mut new_paths: Vec<Path> = vec![];

        for path in valid_paths {
            if path.finish {
                new_paths.push(path);
            } else {
                new_paths.extend(possible_paths(&grid, path));
            }
        }

        let (finished_paths, expanding_paths): (Vec<_>, Vec<_>) =
            new_paths.into_iter().partition(|p| p.finish);

        let mut pruned_expanding_paths = vec![];
        for path in expanding_paths {
            let current_pos = *path.positions.last().unwrap();
            let key = (current_pos, path.direction);
            let score = path.score;

            if let Some(&existing_score) = global_map.get(&key) {
                if existing_score < score {
                    continue;
                } else {
                    pruned_expanding_paths.push(path);
                    // better score for this state -> update
                    global_map.insert(key, score);
                }
            } else {
                pruned_expanding_paths.push(path);
                // first time in this state -> update
                global_map.insert(key,  score);
            }
            
        }

        valid_paths = finished_paths
            .into_iter()
            .chain(pruned_expanding_paths.into_iter())
            .collect();

        if valid_paths.is_empty() || valid_paths.iter().all(|p| p.finish) {
            break;
        }
    }

    if valid_paths.is_empty() {
        None
    } else {
        let min_score = valid_paths.iter().map(|p| p.score).min().unwrap();
        let best_paths: Vec<&Path> = valid_paths.iter().filter(|p| p.score == min_score).collect();
        let unique_positions: HashSet<(usize,usize)> = best_paths.iter().flat_map(|p|p.positions.iter()).cloned().collect();
        Some(unique_positions.len() as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
