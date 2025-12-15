use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Direction {
    Up = 1,
    Down = 2,
    Left = 4,
    Right = 8,
}
impl Direction {
    pub fn next(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

struct Game {
    obstacles: Vec<Vec<bool>>,
    guard_pos: (usize, usize),
    guard_dir: Direction,
}

impl Game {
    pub fn from_str(s: &str) -> Self {
        let mut obstacles = Vec::new();
        let mut guard = None;
        for (y, line) in s.lines().enumerate() {
            obstacles.push(
                line.bytes()
                    .enumerate()
                    .map(|(x, c)| {
                        match c {
                            b'^' => guard = Some(((y, x), Direction::Up)),
                            b'v' => guard = Some(((y, x), Direction::Down)),
                            b'>' => guard = Some(((y, x), Direction::Right)),
                            b'<' => guard = Some(((y, x), Direction::Left)),
                            _ => (),
                        };
                        c == b'#'
                    })
                    .collect_vec(),
            );
        }
        let (guard_pos, guard_dir) = guard.expect("didn't found the guard in grid");
        Self {
            obstacles,
            guard_pos,
            guard_dir,
        }
    }
    pub fn width(&self) -> usize {
        self.obstacles[0].len()
    }
    pub fn height(&self) -> usize {
        self.obstacles.len()
    }
    pub fn traverse<F: FnMut(((usize, usize), Direction))>(&self, f: F) {
        self.traverse_from(self.guard_pos, self.guard_dir)
            .for_each(f);
        //     let mut guard_pos = self.guard_pos;
        //     let mut guard_dir = self.guard_dir;
        //     loop {
        //         let (Some(pos_y), Some(pos_x)) = (match guard_dir {
        //             Direction::Up => (guard_pos.0.checked_sub(1), Some(guard_pos.1)),
        //             Direction::Down => (
        //                 guard_pos.0.checked_add(1).filter(|y| y < &self.height()),
        //                 Some(guard_pos.1),
        //             ),
        //             Direction::Left => (Some(guard_pos.0), guard_pos.1.checked_sub(1)),
        //             Direction::Right => (
        //                 Some(guard_pos.0),
        //                 guard_pos.1.checked_add(1).filter(|x| x < &self.width()),
        //             ),
        //         }) else {
        //             break;
        //         };

        //         if self.obstacles[pos_y][pos_x] {
        //             guard_dir = guard_dir.next();
        //             continue;
        //         }
        //         f((pos_y, pos_x), guard_dir);

        //         guard_pos = (pos_y, pos_x);
        //     }
    }
    pub fn traverse_from<'a>(
        &'a self,
        guard_pos: (usize, usize),
        guard_dir: Direction,
    ) -> GuardPosIterator<'a> {
        GuardPosIterator {
            game: self,
            guard_pos,
            guard_dir,
        }
    }
}

type GuardState = ((usize, usize), Direction);
struct GuardPosIterator<'a> {
    game: &'a Game,
    guard_pos: (usize, usize),
    guard_dir: Direction,
}

impl<'a> Iterator for GuardPosIterator<'a> {
    type Item = GuardState;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (Some(pos_y), Some(pos_x)) = (match self.guard_dir {
                Direction::Up => (self.guard_pos.0.checked_sub(1), Some(self.guard_pos.1)),
                Direction::Down => (
                    self.guard_pos
                        .0
                        .checked_add(1)
                        .filter(|y| y < &self.game.height()),
                    Some(self.guard_pos.1),
                ),
                Direction::Left => (Some(self.guard_pos.0), self.guard_pos.1.checked_sub(1)),
                Direction::Right => (
                    Some(self.guard_pos.0),
                    self.guard_pos
                        .1
                        .checked_add(1)
                        .filter(|x| x < &self.game.width()),
                ),
            }) else {
                return None;
            };

            if self.game.obstacles[pos_y][pos_x] {
                self.guard_dir = self.guard_dir.next();
                continue;
            }

            self.guard_pos = (pos_y, pos_x);
            return Some((self.guard_pos, self.guard_dir));
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let game = Game::from_str(input);
    let mut been_map = vec![vec![false; game.width()]; game.height()];
    let mut been_count = 0;
    game.traverse(|((pos_y, pos_x), _)| {
        if !been_map[pos_y][pos_x] {
            been_map[pos_y][pos_x] = true;
            been_count += 1;
        }
    });
    Some(been_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let game = Game::from_str(input);

    let mut been_dirs = vec![vec![0; game.width()]; game.height()];
    // let mut cached_unreachable = vec![vec![0; game.width()]; game.height()];

    let mut count = 0;

    game.traverse(|(pos, dir)| {
        let next_dir = dir.next();

        let mut local_been_dirs = been_dirs.clone();
        if game.traverse_from(pos, next_dir).any(|((ty, tx), tdir)| {
            // if cached_unreachable[ty][tx] & (tdir as u8) > 0 {
            //     None
            // } else {
            {
                let out = local_been_dirs[ty][tx] & (tdir as u8) > 0;
                local_been_dirs[ty][tx] |= tdir as u8;
                out
            }
            // }
        }) {
            // cache good
            // for ((ty, tx), tdir) in game.traverse_from(pos, next_dir) {
            //     if (been_dirs[ty][tx] & (tdir as u8)) > 0 {
            //         break;
            //     }
            //     been_dirs[ty][tx] |= tdir as u8;
            // }
            println!("{:?}", pos);
            count += 1;
        } else {
            // cache bad
            // for ((ty, tx), tdir) in game.traverse_from(pos, next_dir) {
            //     if (cached_unreachable[ty][tx] & (tdir as u8)) > 0 {
            //         break;
            //     }
            //     cached_unreachable[ty][tx] |= tdir as u8;
            // }
        }
        been_dirs[pos.0][pos.1] |= dir as u8;
    });
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
