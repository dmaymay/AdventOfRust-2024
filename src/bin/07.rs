advent_of_code::solution!(7);

use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> Option<u64> {
    let mut equations = HashMap::new();

    input.lines().for_each(|l| {
        let parts: Vec<&str> = l.split(":").collect();
        let numbers: Vec<u64> = parts[1]
            .split_whitespace()
            .filter_map(|num| num.parse::<u64>().ok())
            .collect();
        let test_value: u64 = parts[0].trim().parse().unwrap_or(0);
        equations.insert(test_value, numbers);
    });

    let mut valid_sum = 0;

    for (test_value, numbers) in &equations {
        // initialize set in fold opertion with first number
        let possible_results = numbers.iter().skip(1).fold({
            let mut set = HashSet::new();
            set.insert(numbers[0]);
            set
        }, |acc_set, &num| {
            let mut new_set = HashSet::new();
            // add every possible result to set
            for &res in &acc_set {
                let sum = res + num;
                let prod = res * num;
                if sum <= *test_value {
                    new_set.insert(sum);
                }
                if prod <= *test_value {
                    new_set.insert(prod);
                }
            }
            new_set
        });

        // if test_value in set add to sum
        if possible_results.contains(test_value) {
            valid_sum += test_value;
        }
    }

    Some(valid_sum)
}

fn digits_count(n: u64) -> u32 {
    n.to_string().len() as u32
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut equations = HashMap::new();

    input.lines().for_each(|l| {
        let parts: Vec<&str> = l.split(":").collect();
        let numbers: Vec<u64> = parts[1]
            .split_whitespace()
            .filter_map(|num| num.parse::<u64>().ok())
            .collect();
        let test_value: u64 = parts[0].trim().parse().unwrap_or(0);
        equations.insert(test_value, numbers);
    });

    let mut valid_sum = 0;

    for (test_value, numbers) in &equations {
        let possible_results = numbers.iter().skip(1).fold({
            let mut set = HashSet::new();
            set.insert(numbers[0]);
            set
        }, |acc_set, &num| {
            let mut new_set = HashSet::new();
            let d = digits_count(num);
            for &res in &acc_set {
                let sum = res + num;
                let prod = res * num;
                 // concatenate by shifting res left as many digits as num has
                let conc = res * 10u64.pow(d) + num;
            
                if sum <= *test_value {
                    new_set.insert(sum);
                }
                if prod <= *test_value {
                    new_set.insert(prod);
                }
                if conc <= *test_value {
                    new_set.insert(conc);
                }
            }
            new_set
        });

        if possible_results.contains(test_value) {
            valid_sum += test_value;
        }
    }

    Some(valid_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
