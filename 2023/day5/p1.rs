use std::fs::read_to_string;

fn main() {
    let binding = read_to_string("p1.in").unwrap();
    let mut f = binding.lines();

    let mut seeds = Vec::<i64>::new();
    let seeds_str = f.next().unwrap().split_once(':').unwrap().1;

    for seed in seeds_str.split_whitespace() {
        seeds.push(seed.parse().unwrap());
    }
    println!("seeds {:?}", seeds);

    let mut moved = vec![false; seeds.len()];
    for line in f {
        if line.contains(':') {
            continue;
        }
        if line.is_empty() {
            moved = vec![false; seeds.len()];
            println!(" ");
            continue;
        }

        let split: Vec<_> = line.split_whitespace().collect();
        let (from, to, len) = (
            split[1].parse::<i64>().unwrap(),
            split[0].parse::<i64>().unwrap(),
            split[2].parse::<i64>().unwrap(),
        );
        println!("{} {} {}", from, to, len);
        for (i, seed) in seeds.iter_mut().enumerate() {
            if !moved[i] && *seed >= from && *seed < from + len {
                moved[i] = true;
                *seed = to + *seed - from;
                println!("translate {} -> {}", *seed + from - to, *seed);
            }
        }
    }

    let mut ans = i64::MAX;
    for seed in seeds {
        ans = ans.min(seed);
    }

    println!("{}", ans);
}
