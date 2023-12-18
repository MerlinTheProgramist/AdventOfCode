use std::fs::read_to_string;

// use of Shoelace Formula for finding the area of enclosed region
// A = abs(sum(x_i(y_{i+1}-y_{i-1})))/2

// use of Picks theorem for finding the accual area of poygon in integer vertex space
// A = area
// b = boundary points
// i = interrior points
// A = i + b/2 - 1
// then:
// i = A - b/2 + 1

// The final area must also include the border area:
// i = A + b/2 + 1 =(|sum(x_i(y_{i+1}-y_{i-1}))| + b)/2 + 1

fn main() {
    let mut verts = Vec::new();
    let mut curr = (0, 0);
    let mut tot_dist = 0;

    for line in read_to_string("p.in").unwrap().lines() {
        let (_,hex) = line.rsplit_once(' ').unwrap();
        let dist = i64::from_str_radix(&hex[2..=6], 16).unwrap();
        let dir = hex.as_bytes()[7] - b'0';

        tot_dist += dist;

        curr = match dir {
            3 => (curr.0 - dist, curr.1), // 'U'
            1 => (curr.0 + dist, curr.1), // 'D'
            2 => (curr.0, curr.1 - dist), // 'L'
            0 => (curr.0, curr.1 + dist), // 'R'
            _ => panic!("undefined symbol"),
        };
        verts.push(curr);
    }

    let mut a = ar_term(verts[verts.len() - 2], verts[verts.len() - 1], verts[0])
        + ar_term(verts[verts.len() - 1], verts[0], verts[1]);
    for i in 0..verts.len() - 2 {
        a += ar_term(verts[i], verts[i + 1], verts[i + 2]);
    }

    let int_area = (a.abs() + tot_dist) / 2 + 1;
    println!("{}", int_area);
}

fn ar_term(v: (i64, i64), u: (i64, i64), e: (i64, i64)) -> i64 {
    u.0 * (e.1 - v.1)
}
