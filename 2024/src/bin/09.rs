use itertools::{Either, Itertools};

advent_of_code::solution!(9);

#[derive(Copy, Clone)]
struct File {
    pub id: u64,
    pub pos: u64,
    pub blocks: u64,
}
#[derive(Copy, Clone)]
struct Empty {
    pub pos: u64,
    pub blocks: u64,
}

pub fn part_one(input: &str) -> Option<u64> {
    let (files, empty): (Vec<File>, Vec<Empty>) = input
        .trim()
        .chars()
        .map(|c| {
            let num = c.to_digit(10).unwrap() as u64;
            num
        })
        .scan(0u64, |pos, n| {
            let current_pos = *pos;
            *pos += n;
            Some((current_pos, n))
        })
        .filter(|(_, n)| *n != 0)
        .enumerate()
        .partition_map(|(i, (pos, blocks))| {
            if i % 2 == 0 {
                let id = (i / 2) as u64;
                Either::Left(File {
                    id,
                    pos: pos + blocks,
                    blocks,
                })
            } else {
                Either::Right(Empty { pos, blocks })
            }
        });

    let mut check_sum: u64 = 0;

    let mut empty_iter = empty.iter();
    let mut files_iter = files.iter().rev();

    let mut file = *files_iter.next().unwrap();
    let mut empty = *empty_iter.next().unwrap();

    let mut DEBUG = String::from_iter(std::iter::repeat('.').take(30));
    while empty.pos <= file.pos {
        println!("{} * {}", empty.pos, file.id);
        DEBUG.replace_range(
            (empty.pos as usize)..(empty.pos as usize),
            &file.id.to_string(),
        );
        check_sum += file.id * empty.pos;

        file.pos -= 1;
        file.blocks -= 1;
        empty.pos += 1;
        empty.blocks -= 1;

        if empty.blocks == 0 {
            empty = *empty_iter.next().unwrap();
        }
        if file.blocks == 0 {
            file = *files_iter.next().unwrap();
        }
    }

    // add from file which is left
    check_sum += (0..(file.pos))
        .rev()
        .take(file.blocks as usize)
        .map(|p| p * file.id)
        .sum::<u64>();
    println!("LEFT: {} {}", file.pos, file.id);

    // add the rest of unmoved files
    for file in files_iter {
        DEBUG.replace_range(
            ((file.pos - file.blocks) as usize)..(file.pos as usize),
            &std::iter::repeat(file.id.to_string())
                .take(file.blocks as usize)
                .collect::<String>(),
        );
        check_sum += ((file.pos - file.blocks)..file.pos)
            .rev()
            .inspect(|pos| println!("{} * {}", pos, file.id))
            .map(|p| p * file.id)
            .sum::<u64>();
    }

    println!("{DEBUG}");
    Some(check_sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

// 009981118882777333644655556...............
// 0099811188827773336446555566..............
