advent_of_code::solution!(4);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    // Convert input string into Vec<Vec<char>> grid
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let xmas_re = Regex::new(r"XMAS").unwrap();
    let samx_re = Regex::new(r"SAMX").unwrap();

    let mut count = 0;

    let rows = grid.len();
    let cols = grid.get(0)?.len();

    // Horizontal lines
    for row in &grid {
        let line: String = row.iter().collect();
        count += xmas_re.find_iter(&line).count();
        count += samx_re.find_iter(&line).count();
    }

    // Vertical lines
    for col in 0..cols {
        let mut line: String = String::new();
        for row in 0..rows {
            line.push(grid[row][col]);
        }
        count += xmas_re.find_iter(&line).count();
        count += samx_re.find_iter(&line).count();
    }

    // Diagonals (top-left to bottom-right)
    for start in 0..rows {
        let mut line: String = String::new();
        let mut x = start;
        let mut y = 0;
        while x < rows && y < cols {
            line.push(grid[x][y]);
            x += 1;
            y += 1;
        }
        count += xmas_re.find_iter(&line).count();
        count += samx_re.find_iter(&line).count();
    }

    for start in 1..cols {
        let mut line: String = String::new();
        let mut x = 0;
        let mut y = start;
        while x < rows && y < cols {
            line.push(grid[x][y]);
            x += 1;
            y += 1;
        }
        count += xmas_re.find_iter(&line).count();
        count += samx_re.find_iter(&line).count();
    }

    // Diagonals (top-right to bottom-left)
    for start in 0..rows {
        let mut line: String = String::new();
        let mut x = start;
        let mut y = cols - 1;
        while x < rows && y < cols {
            line.push(grid[x][y]);
            x += 1;
            if y == 0 {
                break;
            }
            y -= 1;
        }
        count += xmas_re.find_iter(&line).count();
        count += samx_re.find_iter(&line).count();
    }

    for start in (0..cols - 1).rev() {
        let mut line: String = String::new();
        let mut x = 0;
        let mut y = start;
        while x < rows && y < cols {
            line.push(grid[x][y]);
            x += 1;
            if y == 0 {
                break;
            }
            y -= 1;
        }
        count += xmas_re.find_iter(&line).count();
        count += samx_re.find_iter(&line).count();
    }

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Convert the input string into a Vec<Vec<char>> grid
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid.get(0)?.len();

    let mut count = 0;

    for x in 1..rows - 1 {
        for y in 1..cols - 1 {
            if grid[x][y] == 'A' {
                // Check both diagonals for M and S
                let tl_br = (grid[x - 1][y - 1] == 'M' && grid[x + 1][y + 1] == 'S')
                         || (grid[x - 1][y - 1] == 'S' && grid[x + 1][y + 1] == 'M');

                let tr_bl = (grid[x - 1][y + 1] == 'M' && grid[x + 1][y - 1] == 'S')
                         || (grid[x - 1][y + 1] == 'S' && grid[x + 1][y - 1] == 'M');

                if tl_br && tr_bl {
                    count += 1;
                }
            }
        }
    }
    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
