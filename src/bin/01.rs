advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut distances: u32 = 0;
    let mut first_list: Vec<u32> = Vec::new();
    let mut second_list: Vec<u32> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        first_list.push(parts[0].parse::<u32>().ok()?);
        second_list.push(parts[1].parse::<u32>().ok()?);
    }
    first_list.sort();
    second_list.sort();

    for i in 0..first_list.len() {
        distances += (first_list[i] as i32 - second_list[i] as i32).abs() as u32;
    }

    Some(distances)
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