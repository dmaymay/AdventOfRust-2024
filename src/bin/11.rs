advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let stones: Vec<&str> = input.split_whitespace().collect();
    let mut current_stones: Vec<String> = stones.iter().map(|&s| s.to_string()).collect();

    for _ in 0..25 {
        let mut new_stones: Vec<String> = Vec::new();
        for stone in &current_stones {
            if *stone == "0" {
                new_stones.push("1".to_string());
            } else if stone.len() % 2 == 0 {
                let left: u32 = stone[0..stone.len() / 2].parse().unwrap();
                let right: u32 = stone[stone.len() / 2..].parse().unwrap();
                new_stones.push(left.to_string());
                new_stones.push(right.to_string());
            } else {

                let value: usize = stone.parse::<usize>().unwrap();
                new_stones.push((value * 2024).to_string());
            }
        }
        current_stones = new_stones;
    }

    Some(current_stones.len() as usize)
}

use std::collections::HashMap;

fn count_stones(stone: &str, steps: usize, memo: &mut HashMap<(String, usize), usize>) -> usize {
    if steps == 0 {
        return 1;
    }

    // check if memo has it
    if let Some(&res) = memo.get(&(stone.to_string(), steps)) {
        return res;
    }

    // do one blink
    let result = if stone == "0" {
        count_stones("1", steps - 1, memo)

    } else if stone.len() % 2 == 0 {
        let half = stone.len() / 2;
        let left_val = stone[..half].parse::<u64>().unwrap();
        let right_val = stone[half..].parse::<u64>().unwrap();

        count_stones(&left_val.to_string(), steps - 1, memo)
            + count_stones(&right_val.to_string(), steps - 1, memo)

    } else {
        let value = stone.parse::<u64>().unwrap();
        let multiplied = value * 2024; 

        let new_stone = multiplied.to_string();
        count_stones(&new_stone, steps - 1, memo)
    };

    memo.insert((stone.to_string(), steps), result);
    result
}

pub fn part_two(input: &str) -> Option<usize> {
    let stones: Vec<&str> = input.split_whitespace().collect();
    let mut memo = HashMap::new();
    let mut total = 0;

    for s in stones {
        total += count_stones(s, 75, &mut memo);
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
