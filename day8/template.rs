#![allow(unused_mut, unused_variables)]
use std::io::{prelude::*, BufReader};

fn main() {
    use std::env;
    use std::fs::File;

    let mut args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        panic!("Usage: ./bin filename.in");
    } else if args.len() == 1 {
        const DEFAULT_INFILE: &str = "./p1.in";
        println!("Opening default {}", DEFAULT_INFILE);
        args.push(DEFAULT_INFILE.to_string())
    }
    let file = File::open(&args[1])
        .unwrap_or_else(|_| panic!("Error when tryied reading file {}", args[1]));
    let reader = BufReader::new(file);

    solution(reader);
}

fn solution(reader: std::io::BufReader<std::fs::File>) {
    for line in reader.lines() {
        // ...
    }

    let mut ans = 0;
    // ...
    println!("{}", ans);
}
