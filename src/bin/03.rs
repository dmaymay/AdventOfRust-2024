advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let add_result: u32 = re
        .captures_iter(input)
        .filter_map(|cap| {
            let num1 = cap[1].parse::<u32>().ok()?;
            let num2 = cap[2].parse::<u32>().ok()?;
            Some(num1 * num2)
        })
        .sum();

    Some(add_result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let flattened_input = input.replace('\n', "");

    let ignore_re = Regex::new(r"(?s)don't\(\).*?do\(\)|(?s)don't\(\).*").unwrap();
    let cleaned_input = ignore_re.replace_all(&flattened_input, "");

    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let add_result: u32 = mul_re
        .captures_iter(&cleaned_input)
        .filter_map(|cap| {
            let num1 = cap[1].parse::<u32>().ok()?;
            let num2 = cap[2].parse::<u32>().ok()?;
            Some(num1 * num2)
        })
        .sum();

    Some(add_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
