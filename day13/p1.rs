use std::fs::read_to_string;

// const N: usize = 140;

fn main() {
    let file = read_to_string("p.in").unwrap();
    let mut ans = 0;
    // let mut columns = [[false; N]; N];
    // let mut similar_col = false;
    let mut rows = Vec::<i32>::new();
    let mut cols = Vec::<i32>::new();

    let mut y = 0;
    for (i, line) in file.lines().enumerate() {
        if line.is_empty() {
            println!("rows: {:?}", rows);
            println!("cols: {:?}", cols);
            if let Some(pos) = find_mirror(&rows) {
                ans += 100 * (pos + 1);
            } else if let Some(pos) = find_mirror(&cols) {
                ans += pos + 1;
            } else {
                println!("line {}", i);
                panic!("no mirror found!");
            }
            println!("ans: {}", ans);
            y = 0;
            rows = Vec::<i32>::new();
            cols = Vec::<i32>::new();
            continue;
        }
        if y == 0 {
            cols = vec![0; line.len()];
        }

        rows.push(0);
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                cols[x] += 1;
                rows[y] += 1;
            }
            cols[x] <<= 1;
            rows[y] <<= 1;
        }
        y += 1;
    }
    println!("{}", ans);
}

fn find_mirror(v: &Vec<i32>) -> Option<usize> {
    for i in 0..v.len() - 1 {
        // -2 -1  i i+1  +2 +3
        if v[i] == v[i + 1] {
            let dist = (v.len() - 1 - (i + 1)).min(i);
            println!("i={} {:?}", i, &v[i - dist..=i + 1 + dist]);
            if (0..=dist).all(|j| {
                println!("{}=={}", &v[i - j], &v[i + 1 + j]);
                v[i - j] == v[i + 1 + j]
            }) {
                return Some(i);
            }
        }
    }
    return None;
}
