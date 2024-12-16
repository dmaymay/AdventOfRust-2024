use std::collections::HashMap;
advent_of_code::solution!(7);

fn get_combinations(num_spaces: usize) -> Vec<Vec<char>> {
    let mut combinations = Vec::new();
    let total_combos = 2u32.pow(num_spaces as u32);

    for n in 0..total_combos {
        let combo = (0..num_spaces)
            .map(|i| if (n >> i) & 1 == 0 { '+' } else { '*' })
            .collect::<Vec<char>>();
        combinations.push(combo);
    }

    combinations
}

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
        let num_spaces = numbers.len() - 1;
        let operator_combinations = get_combinations(num_spaces);
        let mut is_valid = false;

        for operators in operator_combinations {
            let mut result = numbers[0];
            for (i, &operator) in operators.iter().enumerate() {
                match operator {
                    '+' => result += numbers[i + 1],
                    '*' => result *= numbers[i + 1],
                    _ => unreachable!(),
                }
            }

            if result == *test_value {
                is_valid = true;
                break;
            }
        }

        if is_valid {
            valid_sum += test_value;
        }
    }

    Some(valid_sum)
}

fn get_triple_combo(num_spaces: usize) -> Vec<Vec<&'static str>> {
    let choices = ["*", "+", "||"];
    let mut combinations = Vec::new();
    let total_combos = 3usize.pow(num_spaces as u32);

    for n in 0..total_combos {
        let combo = (0..num_spaces)
            .map(|i| {
                let index = (n / 3usize.pow(i as u32)) % 3;
                choices[index]
            })
            .collect::<Vec<&str>>();
        combinations.push(combo)
    }
    combinations
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
        let num_spaces = numbers.len() - 1;
        let operator_combinations = get_triple_combo(num_spaces);
        let mut is_valid = false;

        for operators in operator_combinations {
            let mut result = numbers[0];
            let mut result_as_string = result.to_string();
            for (i, &operator) in operators.iter().enumerate() {
                match operator {
                    "+" => {
                        result += numbers[i + 1];
                        result_as_string = result.to_string();
                    }
                    "*" => {
                        result *= numbers[i + 1];
                        result_as_string = result.to_string();
                    }
                    "||" => {
                        result_as_string.push_str(&numbers[i + 1].to_string());
                        result = result_as_string.parse::<u64>().unwrap_or(0);
                    }
                    _ => unreachable!(),
                }
            }

            if result == *test_value {
                is_valid = true;
                break;
            }
        }

        if is_valid {
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
