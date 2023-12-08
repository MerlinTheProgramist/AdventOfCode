use std::fs::read_to_string;

fn main() {
    let mut ans = 0;
    let f = read_to_string("p1.in").unwrap();
    let lines: Vec<_> = f.lines().collect();

    let sequence = lines[0].as_bytes();

    const N: usize = 27 * 27 * 27;
    const START: usize = 0;
    const TARGET: usize = to_index("ZZZ");
    let mut graph: [Option<(usize, usize)>; N] = [None; N];

    for line in lines.iter().skip(2) {
        let from = to_index(&line[..3]);
        let left = to_index(&line[7..10]);
        let right = to_index(&line[12..15]);
        graph[from] = Some((left, right));
    }

    let mut curr = START;
    let mut count = 0;
    let mut seq = sequence.iter().cycle();
    while curr != TARGET {
        match seq.next().unwrap() {
            b'L' => curr = graph[curr].unwrap().0,
            b'R' => curr = graph[curr].unwrap().1,
            _ => panic!("UNKNOWN CHARACTER"),
        }
        count += 1;
    }

    println!("{}", count);
}

const fn to_index(s: &str) -> usize {
    (s.as_bytes()[0] - b'A') as usize
        + 26 * (s.as_bytes()[1] - b'A') as usize
        + 26 * 26 * (s.as_bytes()[2] - b'A') as usize
}
