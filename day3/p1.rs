use std::cmp::{max, min};
use std::fs::read_to_string;

const N: usize = 140;

fn main() {
    let mut is_symbol = [[false; N]; N];
    for (y, line) in read_to_string("p1.in").unwrap().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' && !c.is_digit(10) {
                is_symbol[y][x] = true;
            }
        }
    }

    let mut ans = 0;
    for (y, line) in read_to_string("p1.in").unwrap().lines().enumerate() {
        let mut number = 0;
        let mut has_symbol = false;
        for (x, c) in line.chars().enumerate() {
            if let Some(d) = c.to_digit(10) {
                if !has_symbol {
                    // if first digit
                    if number == 0 {
                        has_symbol = x > 0
                            && is_symbol[max(y as i32 - 1, 0) as usize..=min(y + 1, N - 1)]
                                .iter()
                                .any(|row| row[x - 1]);
                    }
                    has_symbol = has_symbol
                        || (y > 0 && is_symbol[y - 1][x])
                        || (y < N - 1 && is_symbol[y + 1][x]);
                }
                number = number * 10 + d;
            } else {
                if number != 0 {
                    if c != '.'
                        || has_symbol
                        || (is_symbol[y][x]
                            || (y > 0 && is_symbol[y - 1][x])
                            || (y < N - 1 && is_symbol[y + 1][x]))
                    {
                        println!("added: {}", number);
                        ans += number;
                    } else {
                        println!("end of {}", number);
                    }
                    number = 0;
                }
                if c == '.' {
                    has_symbol = false;
                }
            }
        }
        if has_symbol {
            ans += number;
        }
    }
    println!("{}", ans);
}
