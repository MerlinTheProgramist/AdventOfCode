use std::fs::read_to_string;

const N: usize = 140;

fn main() {
    let mut cols = [0; N];
    let mut rocks = Vec::new();
    let mut y = 1;
    for line in read_to_string("p.in").unwrap().lines() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => cols[x] = y,
                'O' => {
                    rocks.push(cols[x]);
                    cols[x] += 1;
                }
                _ => (),
            }
        }
        y += 1;
    }
    y -= 1;
    let mut ans = 0;
    for rock in rocks {
        ans += y - rock;
        println!("{}", y - rock);
    }
    println!("{}", ans);
}
