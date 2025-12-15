use itertools::Itertools;

advent_of_code::solution!(7);

fn check_ops(expected: u64, nums: &[u64], result: u64) -> bool {
    if let Some(&next) = nums.first() {
        (result + next <= expected && check_ops(expected, &nums[1..], result + next))
            || (result * next <= expected && check_ops(expected, &nums[1..], result * next))
    } else {
        result == expected
    }
}
fn check_ops_more(target: u64, nums: &[u64], result: u64) -> bool {
    if result > target {
        return false;
    }
    if let Some(&next) = nums.first() {
        check_ops_more(target, &nums[1..], result + next)
            || check_ops_more(target, &nums[1..], result * next)
            || check_ops_more(
                target,
                &nums[1..],
                result * 10u64.pow(next.ilog10() + 1) + next,
            )
    } else {
        result == target
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let line = line.split_once(':').unwrap();
                (
                    line.0.parse::<u64>().unwrap(),
                    line.1
                        .split_whitespace()
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect_vec(),
                )
            })
            .filter(|(test_value, nums)| check_ops(*test_value, &nums[1..], nums[0]))
            .inspect(|(test_value, nums)| println!("{test_value}, {nums:?}"))
            .map(|(res, _)| res)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let line = line.split_once(':').unwrap();
                (
                    line.0.parse::<u64>().unwrap(),
                    line.1
                        .split_whitespace()
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect_vec(),
                )
            })
            .filter(|(test_value, nums)| check_ops_more(*test_value, &nums[1..], nums[0]))
            .inspect(|(test_value, nums)| println!("{test_value}, {nums:?}"))
            .map(|(res, _)| res)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn just_a_test() {
        let result = 15u64;
        let next = 6u64;

        assert_eq!(result * 10u64.pow(next.ilog10() + 1) + next, 156);
    }
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
