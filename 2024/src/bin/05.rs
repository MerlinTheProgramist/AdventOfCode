#![feature(hash_set_entry)]
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    str::Lines,
};

advent_of_code::solution!(5);

type Rules = HashMap<usize, HashSet<usize>>;

fn get_rules(lines: &mut Lines) -> Rules {
    let mut strictly_before = HashMap::<usize, HashSet<usize>>::new();
    lines
        .take_while_ref(|line| !line.is_empty())
        .map(|line| {
            line.split_once('|')
                .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                .unwrap()
        })
        .for_each(|(a, b)| {
            strictly_before
                .entry(b)
                .and_modify(|hs| {
                    hs.insert(a);
                })
                .or_insert(HashSet::from([a]));
        });
    lines.next();
    strictly_before
}

fn is_sorted(strictly_before: &Rules, nums: impl Iterator<Item = usize>) -> bool {
    let mut forbidden = HashSet::<usize>::new();
    for n in nums {
        if forbidden.contains(&n) {
            return false;
        }
        if let Some(before) = strictly_before.get(&n) {
            forbidden.extend(before);
        }
    }
    return true;
}

fn can_precedate(rules: &Rules, a: usize, b: usize) -> bool {
    rules.get(&b).is_none_or(|ax| !ax.contains(&a))
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let rules = get_rules(&mut lines);

    let mut sum = 0;
    for line in lines {
        let mut nums = line.split(',').map(|x| x.parse::<usize>().unwrap());
        let count = nums.clone().count();
        if is_sorted(&rules, nums.clone()) {
            sum += nums.nth(count / 2).unwrap();
        }
    }
    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let rules = get_rules(&mut lines);

    Some(
        lines
            .map(|line| {
                line.split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .filter(|nums| !is_sorted(&rules, nums.clone().into_iter()))
            .map(|mut nums| {
                nums.sort_by(|&a, &b| {
                    if can_precedate(&rules, a, b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
                nums
            })
            .map(|list| list[list.len() / 2])
            .sum::<usize>() as u64,
    )
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
