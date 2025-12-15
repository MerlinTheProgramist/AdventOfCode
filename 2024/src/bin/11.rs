use std::{
    collections::{hash_map::Entry, HashMap},
    ops::DerefMut,
    str::FromStr,
};

use itertools::Itertools;
use num::{self, BigInt, Integer, One};

advent_of_code::solution!(11);
fn number_of_digits(n: &BigInt) -> usize {
    n.to_string().len()
}

fn split(n: &BigInt) -> Vec<BigInt> {
    let base = BigInt::from(10).pow((number_of_digits(&n) / 2) as u32);
    vec![n / &base, n % base]
}

fn expand(x: BigInt) -> Vec<BigInt> {
    if x == BigInt::ZERO {
        vec![BigInt::one()]
    } else if number_of_digits(&x).is_even() {
        split(&x)
    } else {
        vec![x * 2024]
    }
}

fn count_descendants(
    num: BigInt,
    iterations: usize,
    cache: &mut HashMap<(BigInt, usize), usize>,
) -> usize {
    if iterations == 0 {
        return 1;
    }
    if let Some(&c) = cache.get(&(num.clone(), iterations)) {
        return c;
    }
    let mut count = 0;
    for x in expand(num.clone()) {
        // println!("{}", &x);
        count += count_descendants(x, iterations - 1, cache);
        cache.insert((num.clone(), iterations), count);
    }
    count
}

// fn blink(
//     iter: impl Iterator<Item = BigInt>,
//     cache: &mut HashMap<BigInt, Vec<BigInt>>,
// ) -> Vec<BigInt> {
//     iter.flat_map(|x| {}).collect_vec()
// }

pub fn part_one(input: &str) -> Option<u64> {
    let mut cache = HashMap::new();
    let nums = input
        .split_whitespace()
        .filter_map(|s| num::BigInt::from_str(s).ok());
    let mut count = 0;
    for num in nums {
        count += count_descendants(num, 25, &mut cache);
    }
    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut cache = HashMap::new();
    let nums = input
        .split_whitespace()
        .filter_map(|s| num::BigInt::from_str(s).ok());
    let mut count = 0;
    for num in nums {
        count += count_descendants(num, 75, &mut cache);
    }
    Some(count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn big_int_split() {
        assert_eq!(
            vec![BigInt::from(10), BigInt::ZERO],
            split(&BigInt::from(1000))
        );
        assert_eq!(
            vec![BigInt::from(542), BigInt::from(454)],
            split(&BigInt::from(542454))
        );
    }
}
