use std::fs::read_to_string;

const N: usize = 140;
type Num = i64;

fn main() {
    let mut empty_rows = [true; N];
    let mut empty_cols = [true; N];
    let mut galaxies = Vec::<(Num, Num)>::new();
    for (y, line) in read_to_string("p1.in").unwrap().lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '#' {
                galaxies.push((y as Num, x as Num));
                empty_rows[y] = false;
                empty_cols[x] = false;
            }
        }
    }

    let mut ans = 0;
    for i in 0..galaxies.len() {
        let (g, others) = galaxies.split_at(i + 1);
        let g = g[i];
        for other in others {
            println!("{:?} <> {:?}", other, g);
            if *other == g {
                continue;
            }
            // normal dist
            ans += (g.0 - other.0).abs() + (g.1 - other.1).abs();
            // additional dist
            ans += empty_rows[g.0.min(other.0) as usize..g.0.max(other.0) as usize]
                .iter()
                .filter(|&b| *b)
                .count() as Num;
            ans += empty_cols[g.1.min(other.1) as usize..g.1.max(other.1) as usize]
                .iter()
                .filter(|&b| *b)
                .count() as Num;
        }
    }

    println!("{}", ans);
}
