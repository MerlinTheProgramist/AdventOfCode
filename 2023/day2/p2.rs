use std::fs::read_to_string;


fn main() {
    let mut ans = 0;
    for (game_num, line) in read_to_string("p1.in").unwrap().lines().enumerate() {
        ans += game_cubes(line).iter().fold(1, |a,b| a*b);
    }
    println!("{}", ans);
}

fn game_cubes(line: &str) -> [u32;3] {
    let mut cubes = [0,0,0];
    let line = line.split_once(':').unwrap().1;
    for turn in line.split(';') {
        for rec in turn.split(',') {
            let (num, text) = rec.trim().split_once(' ').unwrap();

            let count = num.parse::<u32>().unwrap();

            let mut color = 0;
            for (i, c) in ["red", "green", "blue"].iter().enumerate() {
                if &text == c {
                    color = i;
                }
            }

            cubes[color] = cubes[color].max(count);
        }
    }
    return cubes;
}
