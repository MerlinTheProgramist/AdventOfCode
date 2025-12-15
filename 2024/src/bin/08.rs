use advent_of_code::vec2::Vec2;
use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let mut antennas = HashMap::<u8, Vec<Vec2>>::new();
    let arena_size = Vec2(
        input.lines().count() as isize,
        input.lines().next().unwrap().len() as isize,
    );
    input.lines().enumerate().for_each(|(y, line)| {
        line.bytes().enumerate().for_each(|(x, c)| {
            if c != b'.' {
                antennas
                    .entry(c)
                    .or_insert(vec![])
                    .push(Vec2(y as isize, x as isize));
            }
        })
    });
    let mut count = 0;
    let mut antinodes = vec![vec![false; arena_size.1 as usize]; arena_size.0 as usize];
    for (_, group) in antennas {
        for (&a, &b) in group.iter().tuple_combinations() {
            let diff = b - a; // a to b
            if (a - diff).in_bounds(&arena_size) && antinodes[a - diff] == false {
                antinodes[a - diff] = true;
                count += 1;
            }
            if (b + diff).in_bounds(&arena_size) && antinodes[b + diff] == false {
                antinodes[b + diff] = true;
                count += 1;
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut antennas = HashMap::<u8, Vec<Vec2>>::new();
    let arena_size = Vec2(
        input.lines().count() as isize,
        input.lines().next().unwrap().len() as isize,
    );
    input.lines().enumerate().for_each(|(y, line)| {
        line.bytes().enumerate().for_each(|(x, c)| {
            if c != b'.' {
                antennas
                    .entry(c)
                    .or_insert(vec![])
                    .push(Vec2(y as isize, x as isize));
            }
        })
    });
    let mut count = 0;
    let mut antinodes = vec![vec![false; arena_size.1 as usize]; arena_size.0 as usize];
    for (_, group) in antennas {
        for (&a, &b) in group.iter().tuple_combinations() {
            let diff = b - a; // a to b
            let mut forward = a;
            loop {
                forward += diff;
                if !forward.in_bounds(&arena_size) {
                    break;
                }
                if antinodes[forward] == false {
                    antinodes[forward] = true;
                    count += 1;
                }
            }
            let mut backward = b;
            loop {
                backward -= diff;
                if !backward.in_bounds(&arena_size) {
                    break;
                }
                if antinodes[backward] == false {
                    antinodes[backward] = true;
                    count += 1;
                }
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
/*
......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....#.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.

......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.
*/
