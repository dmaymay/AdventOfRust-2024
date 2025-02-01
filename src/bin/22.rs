advent_of_code::solution!(22);
use std::collections::{HashMap, HashSet, VecDeque};

fn secret_sequence(mut number: usize, steps: usize) -> usize {
    for _ in 0..steps {
        number ^= (number << 6) & 16777215;
        number ^= number >> 5;
        number ^= (number << 11) & 16777215;
        
    }

    number
}

fn sequence_pricing(
    mut number: usize, 
    steps: usize, 
    sequence_sums: &mut HashMap<(i8, i8, i8, i8), u32>
) {
    let mut prices: Vec<u32> = Vec::with_capacity(steps + 1);
    let mut price_changes: VecDeque<i8> = VecDeque::with_capacity(4);
    let mut parsed_sequences: HashSet<(i8, i8, i8, i8)> = HashSet::new();

    for _ in 0..=steps {
        prices.push((number % 10) as u32);
        number ^= (number << 6) & 16777215;
        number ^= number >> 5;
        number ^= (number << 11) & 16777215;
    }

    for i in 1..prices.len() {
        let change = prices[i] as i8 - prices[i - 1] as i8;
        price_changes.push_back(change);

        if price_changes.len() == 4 {
            let key = (
                price_changes[0],
                price_changes[1],
                price_changes[2],
                price_changes[3],
            );

            if parsed_sequences.contains(&key) {
                price_changes.pop_front();
                continue;
            }

            parsed_sequences.insert(key);

            *sequence_sums.entry(key).or_insert(0) += prices[i];
            price_changes.pop_front();
        }
    }
   
}

pub fn part_one(input: &str) -> Option<usize> {
    let numbers: Vec<usize> = input.lines().map(|n| n.parse::<usize>().unwrap()).collect();
    let result = numbers.iter().map(|&n| secret_sequence(n, 2000)).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers: Vec<usize> = input.lines().map(|n| n.parse::<usize>().unwrap()).collect();
    let mut sequence_sums: HashMap<(i8, i8, i8, i8), u32> = HashMap::new();


    for &num in &numbers {
        sequence_pricing(num, 2000, &mut sequence_sums);
    }

    let max_sum = sequence_sums.values().max().copied().unwrap_or(0);
    Some(max_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
