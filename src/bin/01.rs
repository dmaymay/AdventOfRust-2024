use std::collections::HashMap;

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
    let mut similarity_score: u32 = 0;
    let mut first_list: Vec<u32> = Vec::new();
    let mut second_list: Vec<u32> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        first_list.push(parts[0].parse::<u32>().ok()?);
        second_list.push(parts[1].parse::<u32>().ok()?);
    }
    
    let mut count_map: HashMap<u32, usize> = HashMap::new();
    for num in second_list {
        *count_map.entry(num).or_insert(0) += 1;
    }
    
    for id in first_list {
        similarity_score += id * count_map.get(&id).cloned().unwrap_or(0) as u32;
    }

    Some(similarity_score)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
