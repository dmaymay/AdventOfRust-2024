advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_reports = 0;

    for line in input.lines() {
        let levels: Vec<u32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        let is_increasing = levels.windows(2).all(|pair| pair[1] > pair[0]);
        let is_decreasing = levels.windows(2).all(|pair| pair[1] < pair[0]);

        if !(is_increasing || is_decreasing) {
            continue;
        }

        let diffs_within_range = levels.windows(2).all(|pair| {
            let diff = (pair[1] as i32 - pair[0] as i32).abs();
            diff >= 1 && diff <= 3
        });

        if diffs_within_range {
            safe_reports += 1;
        }
    }

    Some(safe_reports)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
