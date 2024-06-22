use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs::read_to_string;

const N: usize = 141;

#[derive(PartialEq, Eq, Copy, Clone)]
struct State {
    curr_loss: u32,
    y: i32,
    x: i32,
    dy: i32,
    dx: i32,
    steps_dir: u8,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.curr_loss.cmp(&self.curr_loss)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut grid = [[u32::MAX; N]; N];
    for (y, line) in read_to_string("p.in").unwrap().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = c.to_digit(10).unwrap();
        }
    }
    let mut seen = HashSet::new();
    let mut heap = BinaryHeap::<State>::new();
    heap.push(State {
        curr_loss: 0,
        y: 0,
        x: 0,
        dy: 0,
        dx: 0,
        steps_dir: 0,
    });

    const DEST: (i32, i32) = (N as i32 - 1, N as i32 - 1);

    while let Some(state) = heap.pop() {
        if DEST == (state.y, state.x) {
            println!("{}", state.curr_loss);
            break;
        }

        if seen.contains(&(state.y, state.x, state.dy, state.dx, state.steps_dir)) {
            continue;
        }
        seen.insert((state.y, state.x, state.dy, state.dx, state.steps_dir));

        if state.steps_dir < 3 && (state.dy != 0 || state.dx != 0) {
            let x = state.x + state.dx;
            let y = state.y + state.dy;
            if x >= 0 && y >= 0 && x < N as i32 && y < N as i32 {
                heap.push(State {
                    curr_loss: state.curr_loss + grid[y as usize][x as usize],
                    x,
                    y,
                    steps_dir: state.steps_dir + 1,
                    ..state
                });
            }
        }

        for (ndy, ndx) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if (ndy, ndx) != (state.dy, state.dx) && (ndy, ndx) != (-state.dy, -state.dx) {
                let x = state.x + ndx;
                let y = state.y + ndy;
                if x >= 0 && y >= 0 && x < N as i32 && y < N as i32 {
                    heap.push(State {
                        curr_loss: state.curr_loss + grid[y as usize][x as usize],
                        x,
                        y,
                        dy: ndy,
                        dx: ndx,
                        steps_dir: 1,
                    });
                }
            }
        }
    }
}
