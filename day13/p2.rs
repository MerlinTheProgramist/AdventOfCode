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
            if let Some(pos) = find_smudged_mirror(&rows) {
                ans += 100 * (pos + 1);
            } else if let Some(pos) = find_smudged_mirror(&cols) {
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

fn find_smudged_mirror(v: &Vec<i32>) -> Option<usize> {
    for i in 0..v.len() - 1 {
        // -2 -1  i i+1  +2 +3
        let dist = (v.len() - 1 - (i + 1)).min(i);
        
        let mut smudge_found = false;
        if (0..=dist).all(|j| {
            println!("{}=={} sm={}", &v[i - j], &v[i + 1 + j], smudge_found);
            if !smudge_found && bit_diff(v[i - j], v[i + 1 + j]){
                smudge_found = true;
                true
            }else{
                v[i-j] == v[i+1+j]
            }
        }) && smudge_found
        {
            return Some(i);
        }
        println!("");
    }
    return None;
}

#[inline(always)]
fn bit_diff(a:i32, b:i32)->bool{
    let n = (a-b).abs();
    n!=0 && (n & (n-1))==0
}
