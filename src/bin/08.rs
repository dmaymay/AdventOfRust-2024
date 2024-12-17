advent_of_code::solution!(8);
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid.get(0)?.len();

    for x in 0..rows {
        for y in 0..cols {
            let antenna = grid[x][y];
            if antenna != '.' {
                for r in 0..rows {
                    for c in 0..cols {
                        if (x, y) != (r, c) && antenna == grid[r][c] {
                            let distance = (x as i32 - r as i32, y as i32 - c as i32);
                            let antinode_one = (x as i32 + 2 * distance.0, y as i32 + 2 * distance.1);
                            let antinode_two = (x as i32 - distance.0, y as i32 - distance.1);

                            if antinode_one.0 >= 0
                                && antinode_one.0 < rows as i32
                                && antinode_one.1 >= 0
                                && antinode_one.1 < cols as i32
                            {
                                antinodes.insert(antinode_one);
                            }

                            if antinode_two.0 >= 0
                                && antinode_two.0 < rows as i32
                                && antinode_two.1 >= 0
                                && antinode_two.1 < cols as i32
                            {
                                antinodes.insert(antinode_two);
                            }
                        }
                    }
                }
            }
        }
    }

    Some(antinodes.len().try_into().unwrap())
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
