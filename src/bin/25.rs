advent_of_code::solution!(25);

fn add_vectors(a: &[u8; 5], b: &[u8; 5]) -> [u8; 5] {
    let mut result = [0; 5];
    for i in 0..5 {
        result[i] = a[i] + b[i];
    }
    result
}

fn is_filled(schematic: &str) -> [u8; 5] {
    let mut heights = [0; 5];

    for l in schematic.lines() {
        let mut fill = [0; 5];
        for (i, c) in l.chars().enumerate().take(5) {
            if c == '#' {
                fill[i] = 1;
            }
        }

        for i in 0..5 {
            heights[i] += fill[i];
        }
    }

    // Subtract [1,1,1,1,1] from heights, ensuring non-negative results
    for h in &mut heights {
        *h -= 1;
    }

    heights
}

pub fn part_one(input: &str) -> Option<u32> {
    let split: Vec<&str> = input.split("\n\n").collect();
    let mut keys: Vec<[u8; 5]> = Vec::new();
    let mut locks: Vec<[u8; 5]> = Vec::new();

    for schematic in split {
        let heights = is_filled(schematic);
        if schematic.lines().next().unwrap() == "....." {
            keys.push(heights);
        } else {
            locks.push(heights);
        }
    }

    let mut result = 0;
    for key in &keys {
        for lock in &locks {
            let combined = add_vectors(key, lock);
            if combined.iter().all(|&x| x < 6) {
                result += 1;
            }
        }
    }

    Some(result)
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
