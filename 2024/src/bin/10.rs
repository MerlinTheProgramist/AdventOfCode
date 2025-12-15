use itertools::Itertools;

use advent_of_code::vec2::Vec2;
advent_of_code::solution!(10);

fn bfs_unique(grid: &Vec<Vec<u8>>, pos: Vec2, cache: &mut Vec<Vec<bool>>) -> u64 {
    if grid[pos.0 as usize][pos.1 as usize] == 9 {
        return 1;
    }
    [Vec2(-1, 0), Vec2(0, -1), Vec2(1, 0), Vec2(0, 1)]
        .iter()
        .map(|&dir| {
            let new_pos = pos + dir;
            if new_pos.in_bounds(&Vec2(grid.len() as isize, grid[0].len() as isize))
                && grid[new_pos] == grid[pos] + 1
                && cache[new_pos] == false
            {
                cache[new_pos] = true;
                bfs_unique(grid, new_pos, cache)
            } else {
                0
            }
        })
        .sum()
}
fn bfs(grid: &Vec<Vec<u8>>, pos: Vec2, cache: &mut Vec<Vec<bool>>) -> u64 {
    if grid[pos.0 as usize][pos.1 as usize] == 9 {
        return 1;
    }
    [Vec2(-1, 0), Vec2(0, -1), Vec2(1, 0), Vec2(0, 1)]
        .iter()
        .map(|&dir| {
            let new_pos = pos + dir;
            if new_pos.in_bounds(&Vec2(grid.len() as isize, grid[0].len() as isize))
                && grid[new_pos] == grid[pos] + 1
            // && cache[new_pos] == false
            {
                // cache[new_pos] = true;
                bfs(grid, new_pos, cache)
            } else {
                0
            }
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect_vec()
        })
        .collect_vec();
    let mut scores_sum = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &height) in row.iter().enumerate() {
            if height == 0 {
                let score = bfs_unique(
                    &grid,
                    Vec2(y as isize, x as isize),
                    &mut vec![vec![false; grid.len()]; grid[0].len()],
                );
                scores_sum += score;
            }
        }
    }
    Some(scores_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect_vec()
        })
        .collect_vec();
    let mut scores_sum = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &height) in row.iter().enumerate() {
            if height == 0 {
                let score = bfs(
                    &grid,
                    Vec2(y as isize, x as isize),
                    &mut vec![vec![false; grid.len()]; grid[0].len()],
                );
                scores_sum += score;
            }
        }
    }
    Some(scores_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
