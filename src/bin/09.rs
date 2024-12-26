use std::collections::VecDeque;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let chars: Vec<char> = input.chars().collect();
    let mut id: u64 = 0;
    let mut encoding: Vec<String> = Vec::new();

    // Build the initial encoding
    for pair in chars.chunks(2) {
        if pair.len() == 2 {
            for _ in 0..(pair[0].to_digit(10)?) {
                encoding.push(id.to_string());
            }
            for _ in 0..(pair[1].to_digit(10)?) {
                encoding.push(".".to_string());
            }
        } else {
            for _ in 0..(pair[0].to_digit(10)?) {
                encoding.push(id.to_string());
            }
        }
        id += 1;
    }

    // Get free space positions and digit positions
    let mut free_positions = VecDeque::new();
    let mut digit_positions = Vec::new();

    for (i, block) in encoding.iter().enumerate() {
        if block == "." {
            free_positions.push_back(i);
        } else {
            digit_positions.push(i);
        }
    }

    // Compacting the file system
    while let (Some(&gap_idx), Some(&digit_idx)) = (free_positions.front(), digit_positions.last())
    {
        if gap_idx < digit_idx {
            let digit_block = encoding[digit_idx].clone();
            encoding[digit_idx] = ".".to_string();
            encoding[gap_idx] = digit_block;

            free_positions.pop_front();
            digit_positions.pop();
        } else {
            break;
        }
    }

    // Remove trailing periods
    while encoding.last().map(|s| s.as_str()) == Some(".") {
        encoding.pop();
    }

    // Compute the checksum
    let checksum: u64 = encoding
        .iter()
        .enumerate()
        .filter_map(|(pos, ch)| {
            if let Ok(val) = ch.parse::<u64>() {
                Some(pos as u64 * val)
            } else {
                None
            }
        })
        .sum();

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
