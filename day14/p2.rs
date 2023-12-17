use std::fs::read_to_string;

const N: usize = 10; // 100

fn main() {
    let mut board = [Default::default(); N];
    let mut y = 1 as usize;
    for line in read_to_string("p.in").unwrap().lines() {
        board[y] = line.clone();
        y += 1;
    }
    // y -= 1;
    for row in rows {
        println!("{:010b}", row)
    }
}

fn roll_north(board: &[&str; N]) {
    let mut cols = [0u128; N];
    let mut rows = [0u128; N];

    for line in board {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    cols[x] = y as u128;
                }
                'O' => {
                    rows[x as usize] |= 1 << cols[x];

                    cols[x] += 1;
                }
                _ => (),
            }
        }
    }
}
