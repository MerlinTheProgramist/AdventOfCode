#![feature(iter_map_windows)]
advent_of_code::solution!(2);
use std::iter::Peekable;

use itertools::{peek_nth, Itertools};

fn check(incr: bool, a: u32, b: u32) -> bool {
    if incr {
        (a < b) && (b - a >= 1) && (b - a <= 3)
    } else {
        (a > b) && (a - b >= 1) && (a - b <= 3)
    }
}

fn is_safe(nums: impl Iterator<Item = u32>) -> bool {
    let mut nums = nums.peekable();
    let first = nums.next().unwrap();
    let second = *nums.peek().unwrap();
    let incr = first < second;
    check(incr, first, second) && nums.map_windows(|[a, b]| check(incr, *a, *b)).all(|x| x)
}

fn is_really_safe(nums: Vec<u32>) -> bool {
    if is_safe(nums.clone().into_iter()) {
        return true;
    }
    (0..nums.len()).any(|i| {
        is_safe(
            nums.iter()
                .enumerate()
                .filter_map(|(j, &n)| (i != j).then_some(n)),
        )
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter(|l| {
                is_safe(
                    l.split_ascii_whitespace()
                        .map(|x| x.parse::<u32>().unwrap()),
                )
            })
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter(|l| {
                is_really_safe(
                    l.split_ascii_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect(),
                )
            })
            .count() as u64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
