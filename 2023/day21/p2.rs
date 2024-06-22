use std::collections::HashSet;
use std::fs::read_to_string;

const N: usize = 131;
const STEPS: usize = 26501365;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

fn main() {
    let mut grid = [[false; N]; N];
    let mut start = None;
    for (y, line) in read_to_string("p.in").unwrap().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = if c == 'S' {
                start = Some(Point {
                    x: x as isize,
                    y: y as isize,
                });
                true
            } else {
                c == '.'
            };
        }
    }

    let cell_width = N as isize;
    let cell_height = N as isize;
    let half_width = cell_width / 2;
    let target = ((STEPS as isize) - half_width) / cell_width;

    // A modular wall function
    let wall_mod_contains = |p: &Point| {
        let mut p = Point {
            x: p.x % cell_width,
            y: p.y % cell_height,
        };

        if p.x < 0 {
            p.x += cell_width;
        }
        if p.y < 0 {
            p.y += cell_height;
        }

        !grid[p.x as usize][p.y as usize]
    };

    let mut active = HashSet::<Point>::default();
    active.insert(start.expect("No start found in input"));

    let mut points = Vec::new();
    for step in 1..=STEPS {
        let mut next_active = HashSet::default();

        for pos in active {
            for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let neighbor = Point {
                    y: pos.y + dir.0,
                    x: pos.x + dir.1,
                };
                if !wall_mod_contains(&neighbor) {
                    next_active.insert(neighbor);
                }
            }
        }

        active = next_active;

        if ((step as isize) - half_width) % cell_width == 0 {
            let i = ((step as isize) - half_width) / cell_width;
            let p = Point {
                x: i,
                y: active.len() as isize,
            };
            points.push(p);

            println!("{step} {p:?}");

            if points.len() == 3 {
                break;
            }
        }
    }

    // Solve the quadratic equation
    // https://stackoverflow.com/questions/19175037/determine-a-b-c-of-quadratic-equation-using-data-points
    let a = points[0].y / ((points[0].x - points[1].x) * (points[0].x - points[2].x))
        + points[1].y / ((points[1].x - points[0].x) * (points[1].x - points[2].x))
        + points[2].y / ((points[2].x - points[0].x) * (points[2].x - points[1].x));

    let b = -points[0].y * (points[1].x + points[2].x)
        / ((points[0].x - points[1].x) * (points[0].x - points[2].x))
        - points[1].y * (points[0].x + points[2].x)
            / ((points[1].x - points[0].x) * (points[1].x - points[2].x))
        - points[2].y * (points[0].x + points[1].x)
            / ((points[2].x - points[0].x) * (points[2].x - points[1].x));

    let c = points[0].y * points[1].x * points[2].x
        / ((points[0].x - points[1].x) * (points[0].x - points[2].x))
        + points[1].y * points[0].x * points[2].x
            / ((points[1].x - points[0].x) * (points[1].x - points[2].x))
        + points[2].y * points[0].x * points[1].x
            / ((points[2].x - points[0].x) * (points[2].x - points[1].x));

    let target = target as i128;
    let ans = (a as i128) * target * target + (b as i128) * target + (c as i128);
    println!("{ans}");
}
