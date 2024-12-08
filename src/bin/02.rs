advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().filter(|line| {
        let levels: Vec<u32> = line.split_whitespace().filter_map(|s| s.parse().ok()).collect();
        is_safe(&levels)
    }).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().filter(|line| {
        let levels: Vec<u32> = line.split_whitespace().filter_map(|s| s.parse().ok()).collect();
        if is_safe(&levels) {
            return true;
        }
        
        (0..levels.len()).any(|i| {
            let mut temp_levels = levels.clone();
            temp_levels.remove(i);
            is_safe(&temp_levels)
        })
    }).count() as u32)
}

fn is_safe(levels: &[u32]) -> bool {
    let is_increasing = levels.windows(2).all(|pair| pair[1] > pair[0]);
    let is_decreasing = levels.windows(2).all(|pair| pair[1] < pair[0]);

    if !(is_increasing || is_decreasing) {
        return false;
    }

    levels.windows(2).all(|pair| {
        let diff = (pair[1] as i32 - pair[0] as i32).abs();
        diff >= 1 && diff <= 3
    })  
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
        assert_eq!(result, Some(4));
    }
}
