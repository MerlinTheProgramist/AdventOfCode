use std::env;
use std::fs::read_to_string;

const N: usize = 140;

type Connect = [(i8, i8); 2];

struct Pipe {
    pos: (usize, usize),
    from: (i8, i8),
}

const L: Connect = [(-1, 0), (0, 1)];
const F: Connect = [(1, 0), (0, 1)];
const J: Connect = [(-1, 0), (0, -1)];
const _7: Connect = [(0, -1), (1, 0)];
const I: Connect = [(-1, 0), (1, 0)];
const H: Connect = [(0, -1), (0, 1)];
const NONE: Connect = [(0, 0), (0, 0)];

fn main() {
    let args: Vec<_> = env::args().collect();
    let file_name = if args.len() == 2 {
        args[1].as_str()
    } else {
        "p1.in"
    };

    let mut grid = [[&NONE; N]; N];

    let mut start = (0, 0);
    for (y, line) in read_to_string(file_name).unwrap().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (y, x);
            } else {
                grid[y][x] = pipe(c);
            }
        }
    }

    let mut curr: (&Connect, (usize, usize)) = (&L, start);
    let mut arrival_dir = 0;

    for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        // if neighboor pipe allows connection from this
        let pos = (
            (curr.1 .0 as i32 + dir.0 as i32) as usize,
            (curr.1 .1 as i32 + dir.1 as i32) as usize,
        );
        let pipe = grid[pos.0][pos.1];
        let neg_dir = neg_dir(&dir);
        println!("checked {:?}, {:?}", pipe, neg_dir);
        if let Some(arrival) = pipe.iter().position(|&p| p == neg_dir) {
            curr = (pipe, pos);
            arrival_dir = arrival;
            break;
        }
    }
    println!("starting with: {:?}", curr);

    let mut count = 2;
    loop {
        // get the other dir
        let dir = curr.0[(arrival_dir == 0) as usize];
        let pos = (
            (curr.1 .0 as i32 + dir.0 as i32) as usize,
            (curr.1 .1 as i32 + dir.1 as i32) as usize,
        );
        if pos == start {
            break;
        }
        // println!("try: {:?}", pos);
        let pipe = grid[pos.0][pos.1];
        let neg_dir = neg_dir(&dir);
        if let Some(arrival) = pipe.iter().position(|&p| p == neg_dir) {
            curr = (pipe, pos);
            arrival_dir = arrival;
            println!("{:?}", pos);
        }

        count += 1;
    }

    println!("{}", count / 2);
}

#[inline(always)]
fn pipe(c: char) -> &'static Connect {
    match c {
        'L' => &L,
        'F' => &F,
        '|' => &I,
        'J' => &J,
        '7' => &_7,
        '-' => &H,
        '.' => &NONE,
        _ => panic!("UNEXPECTED CHARACTER"),
    }
}
#[inline(always)]
fn neg_dir(dir: &(i8, i8)) -> (i8, i8) {
    (-dir.0, -dir.1)
}
