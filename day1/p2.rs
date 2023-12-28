use std::fs::read_to_string;

const DIGITS:[&str;9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main() {
    let mut ans = 0;
    for line in read_to_string("p.in").unwrap().lines() {
        let mut first = None;
        let mut last = None;
        for (i,c) in line.chars().enumerate(){
            let mut dig = None;
            if let Some(d) = c.to_digit(10){
                dig = Some(d);
            }else{
                // check if in digits
                for (j,&num) in DIGITS.iter().enumerate(){
                    if i+num.len() <= line.len() && line[i..i+num.len()] == *num{
                        dig = Some(j as u32 + 1)
                    }
                }
            }
            
            if dig.is_none(){
                continue;
            }
            
            if first.is_none(){
                first = dig;
            }else{
                last = dig;
            }
        }
        let first = first.expect("This map is not valid, it is advised to contant your nearby trebuchet advisor");
        if let Some(last) = last{
            ans += first*10 + last;
            println!("{}", first*10+last);
        }else{
            ans += first*11;
            println!("{}", first*11);
        }
    }
    println!("{}", ans);
}
