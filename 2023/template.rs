use std::fs::read_to_string;

fn main() {
    for (y, line) in read_to_string("p.in").unwrap().lines().enumerate() {
        // ...
    }
    let mut ans = 0;
    println!("{}", ans);
}
