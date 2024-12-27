advent_of_code::solution!(10);
use std::collections::HashSet;

struct Paths {
    trailhead: (i32, i32),
    valid_peaks: HashSet<(i32, i32)>,
    // alternatively
    // score: u32
}

fn in_bounds(x: i32, y: i32, rows: usize, cols: usize) -> bool {
    x >= 0 && x < rows as i32 && y >= 0 && y < cols as i32
}

fn hike(x: i32, y: i32) -> Vec<(i32, i32)> {
    vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut paths: Vec<Paths> = Vec::new();
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    
    let rows = map.len();
    let cols = map.get(0)?.len();

    // Find trailheads
    for x in 0..rows {
        for y in 0..cols {
            if map[x][y] == '0' {
                paths.push(Paths {
                    trailhead: (x as i32, y as i32),
                    valid_peaks: HashSet::new(),
                    //score: 0,
                });
            }
        }
    }

    // Process each trailhead
    for trail in &mut paths {
        let mut possible_paths: Vec<Vec<(i32, i32)>> = vec![vec![trail.trailhead]];

        while !possible_paths.is_empty() {
            let mut new_possible_paths: Vec<Vec<(i32, i32)>> = Vec::new();

            for path in possible_paths {
                // last position of the current hike
                let (x, y) = path[path.len() - 1];

                // height of current position
                let height: i32 = map[x as usize][y as usize].to_digit(10).unwrap_or(0) as i32;

                // check 4 directions
                let valid_hikes: Vec<(i32, i32)> = hike(x, y)
                    .into_iter()
                    .filter(|&(nx, ny)| {
                        in_bounds(nx, ny, rows, cols)
                            && map[nx as usize][ny as usize].to_digit(10).unwrap_or(0) as i32
                                == height + 1
                    })
                    .collect();

                if valid_hikes.is_empty() {
                    continue;
                }

                for hike in valid_hikes {
                    if map[hike.0 as usize][hike.1 as usize] == '9' {
                        trail.valid_peaks.insert((hike.0, hike.1));
                    } else {
                        new_possible_paths.push(vec![trail.trailhead, (hike.0, hike.1)]);
                    }
                }
            }
            possible_paths = new_possible_paths;
        }
    }
    let total_score: u32 = paths
        .into_iter()
        .map(|trail| trail.valid_peaks.len() as u32)
        .sum();

    Some(total_score)
}

struct Paths2 {
    trailhead: (i32, i32),
    valid_paths: Vec<Vec<(i32, i32)>>,
    // alternatively
    // score: i32
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut paths: Vec<Paths2> = Vec::new();
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = map.len();
    let cols = map.get(0)?.len();

    // Find trailheads
    for x in 0..rows {
        for y in 0..cols {
            if map[x][y] == '0' {
                paths.push(Paths2 {
                    trailhead: (x as i32, y as i32),
                    valid_paths: Vec::new(),
                });
            }
        }
    }

    // Process each trailhead
    for trail in &mut paths {
        let mut possible_paths: Vec<Vec<(i32, i32)>> = vec![vec![trail.trailhead]];

        while !possible_paths.is_empty() {
            let mut new_possible_paths: Vec<Vec<(i32, i32)>> = Vec::new();

            for path in possible_paths {
                // last position of the current hike
                let (x, y) = path[path.len() - 1];

                // height of current position
                let height: i32 = map[x as usize][y as usize].to_digit(10).unwrap_or(0) as i32;

                // check 4 directions
                let valid_hikes: Vec<(i32, i32)> = hike(x, y)
                    .into_iter()
                    .filter(|&(nx, ny)| {
                        in_bounds(nx, ny, rows, cols)
                            && map[nx as usize][ny as usize].to_digit(10).unwrap_or(0) as i32
                                == height + 1
                    })
                    .collect();

                if valid_hikes.is_empty() {
                    continue;
                }

                for hike in valid_hikes {
                    let mut new_path = path.clone();
                    new_path.push(hike);

                    if map[hike.0 as usize][hike.1 as usize] == '9' {
                        trail.valid_paths.push(new_path);
                    } else {
                        new_possible_paths.push(new_path);
                    }
                }
            }

            possible_paths = new_possible_paths;
        }
    }

    let total_paths: u32 = paths
        .iter()
        .map(|trail| trail.valid_paths.len() as u32)
        .sum();

    Some(total_paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
