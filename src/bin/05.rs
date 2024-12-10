advent_of_code::solution!(5);
use std::{collections::HashMap, vec};

#[derive(Debug)]
struct PageRules {
    left: Vec<u32>,
    right: Vec<u32>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, pages) = input.split_once("\n\n").unwrap();

    let page_updates: Vec<Vec<u32>> = pages
        .lines()
        .map(|line| {
            line.split(",")
                .map(|n| n.trim().parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let mut page_rules = HashMap::<u32, PageRules>::new();

    rules.lines().for_each(|line| {
        let parts: Vec<&str> = line.split("|").collect();
        let num1 = parts[0].parse::<u32>().unwrap();
        let num2 = parts[1].parse::<u32>().unwrap();

        page_rules
            .entry(num1)
            .or_insert(PageRules {
                left: vec![],
                right: vec![],
            })
            .right
            .push(num2);

        page_rules
            .entry(num2)
            .or_insert(PageRules {
                left: vec![],
                right: vec![],
            })
            .left
            .push(num1);
    });

    let mut sum = 0;
    let mut valid: bool = true;
    page_updates.into_iter().for_each(|pages| {
        valid = true;
        for i in 0..pages.len() {
            // Check numbers that should be before (left)
            if let Some(rule) = page_rules.get(&pages[i]) {
                if pages[i + 1..].iter().any(|&x| rule.left.contains(&x)) {
                    valid = false;
                    break;
                }
            }
            // Check numbers that should be after (right)
            if let Some(rule) = page_rules.get(&pages[i]) {
                if pages[..i].iter().any(|&x| rule.right.contains(&x)) {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            sum += pages[pages.len() / 2];
        }
    });

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, pages) = input.split_once("\n\n").unwrap();

    let page_updates: Vec<Vec<u32>> = pages
        .lines()
        .map(|line| {
            line.split(",")
                .map(|n| n.trim().parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let mut page_rules = HashMap::<u32, PageRules>::new();

    rules.lines().for_each(|line| {
        let parts: Vec<&str> = line.split("|").collect();
        let num1 = parts[0].parse::<u32>().unwrap();
        let num2 = parts[1].parse::<u32>().unwrap();

        page_rules
            .entry(num1)
            .or_insert(PageRules {
                left: vec![],
                right: vec![],
            })
            .right
            .push(num2);

        page_rules
            .entry(num2)
            .or_insert(PageRules {
                left: vec![],
                right: vec![],
            })
            .left
            .push(num1);
    });


    let mut sum = 0;
    let mut valid: bool = true;
    page_updates.into_iter().for_each(|pages| {
        valid = true;
        for i in 0..pages.len() {
            // Check numbers that should be before (left)
            if let Some(rule) = page_rules.get(&pages[i]) {
                if pages[i + 1..].iter().any(|&x| rule.left.contains(&x)) {
                    valid = false;
                    break;
                }
            }
            // Check numbers that should be after (right)
            if let Some(rule) = page_rules.get(&pages[i]) {
                if pages[..i].iter().any(|&x| rule.right.contains(&x)) {
                    valid = false;
                    break;
                }
            }
        }
        if !valid {
            let mut pages = pages;
            pages.sort_by(|a, b| {
                if page_rules.get(a).unwrap().left.contains(b) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            });
            sum += pages[pages.len() / 2];
        }
    });
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
