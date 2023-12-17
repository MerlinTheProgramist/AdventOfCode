use std::fs::read_to_string;

const N: usize = 256;

fn hash(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .fold(0, |acc, &c| ((acc + c as usize) * 17) % N)
}

fn main() {
    let mut ans = 0;
    for s in read_to_string("p.in").unwrap().trim().split(',') {
        ans += hash(&s);
    }
    println!("{}", ans);
}
