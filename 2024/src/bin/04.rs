#![feature(mixed_integer_ops_unsigned_sub)]
advent_of_code::solution!(4);

fn print_dbg<const A: usize>(grid: &[[u8; A]; A]) {
    for i in 0..A {
        for j in 0..A {
            print!("{}", grid[i][j] as char);
        }
        println!();
    }
    println!();
}

pub fn part_one(input: &str) -> Option<u64> {
    const DIRECTIONS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    const SEARCH: &'static [u8; 4] = b"XMAS";
    let grid: Vec<_> = input.lines().collect();
    let width = grid.first().map(|l| l.len()).unwrap_or(0);
    let height = grid.len();
    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            if grid[y].bytes().nth(x) == Some(SEARCH[0]) {
                'dir: for dir in DIRECTIONS {
                    let mut dir_x = x;
                    let mut dir_y = y;
                    for &c in SEARCH[1..].iter() {
                        if dir_x.checked_add_signed(dir.0).is_some_and(|nx| {
                            dir_x = nx;
                            nx < width
                        }) && dir_y.checked_add_signed(dir.1).is_some_and(|ny| {
                            dir_y = ny;
                            ny < height
                        }) && grid[dir_y].bytes().nth(dir_x) == Some(c)
                        {
                            continue;
                        }
                        continue 'dir;
                    }
                    count += 1;
                }
            }
        }
    }
    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    const SEARCH: &'static [u8; 3] = b"MAS";
    const DIRECTIONS: [((isize, isize), [((isize, isize), (isize, isize)); 2]); 2] = [
        ((-1, -1), [((-2, 0), (1, -1)), ((0, -2), (-1, 1))]),
        ((1, 1), [((2, 0), (-1, 1)), ((0, 2), (1, -1))]),
    ];
    let grid: Vec<_> = input.lines().collect();
    let width = grid.first().map(|l| l.len()).unwrap_or(0);
    let height = grid.len();
    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            if grid[y].bytes().nth(x) == Some(SEARCH[0]) {
                // let mut DBG = [[b'.'; 20]; 20];
                // DBG[y][x] = SEARCH[0];
                'next_dir: for (dir, dirs2) in DIRECTIONS {
                    {
                        let mut dir_x = x;
                        let mut dir_y = y;
                        for &c in SEARCH[1..].iter() {
                            if dir_y.checked_add_signed(dir.0).is_some_and(|ny| {
                                dir_y = ny;
                                ny < height
                            }) && dir_x.checked_add_signed(dir.1).is_some_and(|nx| {
                                dir_x = nx;
                                nx < width
                            }) && grid[dir_y].bytes().nth(dir_x) == Some(c)
                            {
                                // DBG[dir_y][dir_x] = c;
                                // print_dbg(&DBG);

                                continue;
                            }
                            continue 'next_dir;
                        }
                    }
                    'second_word: for (begin, dir) in dirs2 {
                        let (Some(mut dir_y), Some(mut dir_x)) =
                            (y.checked_add_signed(begin.0), x.checked_add_signed(begin.1))
                        else {
                            continue 'second_word;
                        };
                        if grid[dir_y].bytes().nth(dir_x) != Some(SEARCH[0]) {
                            continue 'second_word;
                        }
                        for &c in SEARCH[1..].iter() {
                            if dir_y.checked_add_signed(dir.0).is_some_and(|ny| {
                                dir_y = ny;
                                ny < height
                            }) && dir_x.checked_add_signed(dir.1).is_some_and(|nx| {
                                dir_x = nx;
                                nx < width
                            }) && grid[dir_y].bytes().nth(dir_x) == Some(c)
                            {
                                // DBG[dir_y][dir_x] = c;
                                // print_dbg(&DBG);
                                continue;
                            }
                            continue 'second_word;
                        }
                        count += 1;
                        // print_dbg(&DBG);
                    }
                }
            }
        }
    }
    Some(count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
