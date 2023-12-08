use std::fs::read_to_string;

fn main() {
    let f = read_to_string("p1.in").unwrap();
    let lines: Vec<_> = f.lines().collect();

    let sequence = lines[0].as_bytes();

    const N: usize = 27 * 27 * 27;
    const TARGET: usize = to_index("AAZ");
    let mut starting_points = Vec::<usize>::new();

    let mut graph: [Option<(usize, usize)>; N] = [None; N];

    for line in lines.iter().skip(2) {
        let from = to_index(&line[..3]);
        let left = to_index(&line[7..10]);
        let right = to_index(&line[12..15]);
        graph[from] = Some((left, right));

        if line.as_bytes()[2] == b'A' {
            starting_points.push(from);
        }
    }

    let mut arrivals = Vec::new();
    for start in starting_points.iter() {
        let mut curr = start.clone();
        let mut count = 0;
        let mut seq = sequence.iter().cycle();
        while curr < TARGET {
            match seq.next().unwrap() {
                b'L' => curr = graph[curr].unwrap().0,
                b'R' => curr = graph[curr].unwrap().1,
                _ => panic!("UNKNOWN CHARACTER"),
            }
            count += 1;
        }
        arrivals.push(count);
    }

    // Least Common Multiple
    let ans = lcm(&arrivals);

    println!("{}", ans);
}

const fn to_index(s: &str) -> usize {
    (s.as_bytes()[0] - b'A') as usize
        + 26 * (s.as_bytes()[1] - b'A') as usize
        + 26 * 26 * (s.as_bytes()[2] - b'A') as usize
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(v: &Vec<usize>) -> usize {
    let mut ans = v[0];
    for n in v.iter().skip(1) {
        ans = (n * ans) / gcd(*n, ans);
    }
    return ans;
}
