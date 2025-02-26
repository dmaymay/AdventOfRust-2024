advent_of_code::solution!(25);
fn add_vectors(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
}

fn is_filled(schematic: &str) -> Vec<u8> {
    let mut heights: Vec<u8> = vec![0, 0, 0, 0, 0];

    schematic.lines().for_each(|l| {
        let characters: Vec<char> = l.chars().collect();
        let fill: Vec<u8> = characters
            .into_iter()
            .map(|c| if c == '#' { 1 } else { 0 })
            .collect();

        heights = add_vectors(&heights, &fill);
    });

    // Subtract [1, 1, 1, 1, 1] from heights
    heights.iter_mut().for_each(|h| *h = h.saturating_sub(1));

    heights
}

pub fn part_one(input: &str) -> Option<u32> {
    let split: Vec<&str> = input.split("\n\n").collect();
    let mut keys: Vec<Vec<u8>> = vec![];
    let mut locks: Vec<Vec<u8>> = vec![];

    for schematic in split {
        let heights = is_filled(schematic);

        let first_line = schematic.lines().next().unwrap();
        if first_line == "....." {
            keys.push(heights)
        } else {
            locks.push(heights)
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
