use std::cmp::{max, min};
use std::fs::read_to_string;
use std::iter::zip;

const N: usize = 140;

fn main() {
    let mut is_gear = [[false; N]; N];
    let mut gear_count = [[0;N];N];
    let mut ratio = [[0;N];N];

    for (y, line) in read_to_string("p1.in").unwrap().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c=='*' {
                is_gear[y][x] = true;
                ratio[y][x] = 1;
            }
        }
    }

    for (y, line) in read_to_string("p1.in").unwrap().lines().enumerate() {
        let mut number = 0;
        let mut digits = 0;
        for (x, c) in line.chars().enumerate() {
            if let Some(d) = c.to_digit(10) {
                number = number * 10 + d;
                digits += 1;
            } else if number!=0{
                print!("end of {}", number);
                for i in max(0, y as i32 -1) as usize..=min(N-1, y+1){
                    for j in max(0, x as i32 - digits-1)as usize..=x{
                        if is_gear[i][j]{
                            gear_count[i][j] += 1;
                            ratio[i][j] *= number;
                            print!(" (gear {},{})", ratio[i][j], gear_count[i][j]);
                        }
                    }
                }
                println!("");
                digits = 0;
                number = 0;
            }
        }
        
    }
    let mut ans = 0;
    for (r_row, c_row) in zip(ratio, gear_count){
        for (r, c) in zip(r_row, c_row){
            if c == 2{
                ans += r;
            }
        }
    }
    
    println!("{}", ans);
}
