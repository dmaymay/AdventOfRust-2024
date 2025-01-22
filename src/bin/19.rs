advent_of_code::solution!(19);

fn design_possible(
    patterns: &Vec<&str>, 
    design: &str, 
    index: usize, 
    memo: &mut Vec<Option<bool>>
) -> bool {
    // check if we already computed result for this index
    if let Some(cached) = memo[index] {
        return cached;
    }

    // we reached the end of the design
    if index == design.len() {
        memo[index] = Some(true);
        return true;
    }

    // try each pattern at current index
    for &p in patterns {
        if index + p.len() <= design.len() && p == &design[index..index + p.len()] {
            if design_possible(patterns, design, index + p.len(), memo) {
                memo[index] = Some(true);
                return true;
            }
        }
    }

    // no pattern works
    memo[index] = Some(false);
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let (patterns_input, designs_input) = input.split_once("\n\n")?;
    let patterns: Vec<&str> = patterns_input
        .split(", ")
        .filter(|p| !p.is_empty())
        .collect();

    let designs: Vec<&str> = designs_input.lines().collect();

    let mut possible_design_count: u32 = 0;

    for design in designs {
        // memo table
        let mut memo = vec![None; design.len() + 1];
        if design_possible(&patterns, design, 0, &mut memo) {
            possible_design_count += 1;
        }
    }

    Some(possible_design_count)
}

fn count_designs(
    patterns: &Vec<&str>, 
    design: &str, 
    index: usize, 
    memo: &mut Vec<Option<u64>>
) -> u64 {
    // check if we already computed result for this index
    if let Some(cached) = memo[index] {
        return cached;
    }

    // we reached the end of the design
    if index == design.len() {
        memo[index] = Some(1);
        return 1;
    }

    let mut total_ways = 0;

    // try each pattern at current index
    for &p in patterns {
        if index + p.len() <= design.len() && p == &design[index..index + p.len()] {
            total_ways += count_designs(patterns, design, index + p.len(), memo);
            }
        }

    memo[index] = Some(total_ways);
    total_ways
}

pub fn part_two(input: &str) -> Option<u64> {
    let (patterns_input, designs_input) = input.split_once("\n\n")?;
    let patterns: Vec<&str> = patterns_input
        .split(", ")
        .filter(|p| !p.is_empty())
        .collect();

    let designs: Vec<&str> = designs_input.lines().collect();

    let mut total_count = 0;

    for design in designs {
        let mut memo = vec![None; design.len() + 1];
        total_count += count_designs(&patterns, design, 0, &mut memo);
    }

    Some(total_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
