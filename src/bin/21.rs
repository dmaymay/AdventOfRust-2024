use std::collections::{HashMap, HashSet};

advent_of_code::solution!(21);

fn keypad_combinations() -> HashMap<(char, char), Vec<String>> {
    let mut combinations: HashMap<(char, char), Vec<String>> = HashMap::new();

    let direction_map: Vec<(char, (usize, usize))> = vec![
        ('^', (0, 1)),
        ('A', (0, 2)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
    ];
    let direction_gap: (usize, usize) = (0, 0);

    let numeric_map: Vec<(char, (usize, usize))> = vec![
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('0', (3, 1)),
        ('A', (3, 2)),
    ];
    let numeric_gap: (usize, usize) = (3, 0);

    combinations_to_string(&direction_map, direction_gap, &mut combinations);
    combinations_to_string(&numeric_map, numeric_gap, &mut combinations);

    // remove duplicate strings from combinations
    for value in combinations.values_mut() {
        let unique_values: HashSet<String> = value.drain(..).collect();
        *value = unique_values.into_iter().collect();
    }

    combinations
}

fn combinations_to_string(
    keypad: &Vec<(char, (usize, usize))>,
    gap: (usize, usize),
    combinations: &mut HashMap<(char, char), Vec<String>>,
) {
    // consecutive moves are always shorter
    // row moves first or column moves first

    for &(start_char, start_pos) in keypad {
        for &(end_char, end_pos) in keypad {
            let r_repeats = start_pos.0.abs_diff(end_pos.0);
            let c_repeats = start_pos.1.abs_diff(end_pos.1);
            let r_str = if start_pos.0 < end_pos.0 {
                "v".repeat(r_repeats)
            } else {
                "^".repeat(r_repeats)
            };
            let c_str = if start_pos.1 < end_pos.1 {
                ">".repeat(c_repeats)
            } else {
                "<".repeat(c_repeats)
            };

            // rows first
            if (end_pos.0, start_pos.1) != gap {
                let mut sequence = r_str.clone();
                sequence.push_str(&c_str);
                sequence.push_str("A");
                combinations
                    .entry((start_char, end_char))
                    .or_default()
                    .push(sequence);
            }

            // columns first
            if (start_pos.0, end_pos.1) != gap {
                let mut sequence = c_str.clone();
                sequence.push_str(&r_str);
                sequence.push_str("A");
                combinations
                    .entry((start_char, end_char))
                    .or_default()
                    .push(sequence);
            }
        }
    }
}

fn recursive_keypad(
    memo: &mut HashMap<(char, char, usize), usize>,
    combinations: &HashMap<(char, char), Vec<String>>,
    code: &str,
    recurse: usize,
) -> usize {
    if recurse == 0 {
        return code.len();
    }

    let mut previous_key = 'A';
    let mut result = 0;

    for current_key in code.chars() {
        let key = (previous_key, current_key, recurse);

        // for every possible combination get the one with the least presses
        result += memo.get(&key).copied().unwrap_or_else(|| {
            let moves_amount = combinations[&(previous_key, current_key)]
                .iter()
                .map(|next| recursive_keypad(memo, combinations, next, recurse - 1))
                .min()
                .unwrap();
            memo.insert(key, moves_amount);
            moves_amount
        });

        previous_key = current_key;
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let combinations = keypad_combinations();
    // Hashmap (start_char, end_char, recursion depth) : len of key string
    let mut memo: HashMap<(char, char, usize), usize> = HashMap::new();

    let recurse = 3;
    let result: u32 = input
        .lines()
        .map(|code| {
            let numeric_chunk = code.replace("A", "").parse::<u32>().unwrap_or(0);
            numeric_chunk as u32 * recursive_keypad(&mut memo, &combinations, code, recurse) as u32
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let combinations = keypad_combinations();
    // Hashmap (start_char, end_char, recursion depth) : len of key string
    let mut memo: HashMap<(char, char, usize), usize> = HashMap::new();

    let recurse = 26;
    let result: u64 = input
        .lines()
        .map(|code| {
            let numeric_chunk = code.replace("A", "").parse::<u64>().unwrap_or(0);
            numeric_chunk as u64 * recursive_keypad(&mut memo, &combinations, code, recurse) as u64
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154115708116294));
    }
}
