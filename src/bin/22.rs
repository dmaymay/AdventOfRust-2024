advent_of_code::solution!(22);

fn secret_sequence(mut number: usize, steps: usize) -> usize {
    for _ in 0..steps {
        number = (number << 6) ^ number;
        number &= 16777215;
        number = (number >> 5) ^ number;
        number = (number << 11) ^ number;
        number &= 16777215;
    }
    number
}

pub fn part_one(input: &str) -> Option<usize> {
    let numbers: Vec<usize> = input.lines().map(|n| n.parse::<usize>().unwrap()).collect();
    let result = numbers.iter().map(|&n| secret_sequence(n, 2000)).sum();
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
