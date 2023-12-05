use std::fs::read_to_string;

const contents: [u32; 3] = [12, 13, 14];

fn main() {
    let mut ans = 0;
    for (game_num, line) in read_to_string("p1.in").unwrap().lines().enumerate() {
        if is_game_legal(line) {
            ans += game_num + 1;
        }
    }
    println!("{}", ans);
}

fn is_game_legal(line: &str) -> bool {
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

            if contents[color] < count {
                return false;
            }
        }
    }
    return true;
}
