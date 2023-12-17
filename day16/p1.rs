use std::fs::read_to_string;
use std::mem::swap;

const N: usize = 10;

fn main() {
    let mut grid = [['\0'; N]; N];

    for (y, line) in read_to_string("p.in").unwrap().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = c;
        }
    }

    let mut visited = [[[false; 2]; N]; N];
    let mut lasers = Vec::<((i32, i32), (i32, i32))>::new();
    let mut pos: (i32, i32) = (0, -1);
    let mut dir: (i32, i32) = (0, 1);

    'main: loop {
        pos.0 += dir.0;
        pos.1 += dir.1;

        // println!("dir:{:?} pos:{:?}", dir, pos);
        // draw_grid(&visited);
        // println!("");

        if pos.0 < 0 || pos.1 < 0 || pos.0 >= N as i32 || pos.1 >= N as i32 {
            if let Some((p, d)) = lasers.pop() {
                pos = p;
                dir = d;
                continue 'main;
            } else {
                break 'main;
            }
        }

        let approach = match grid[pos.0 as usize][pos.1 as usize] {
            '\\' => {
                swap(&mut dir.0, &mut dir.1);
                (dir.0 == -1 || dir.1 == 1) as usize
            }
            '/' => {
                swap(&mut dir.0, &mut dir.1);
                dir.0 *= -1;
                dir.1 *= -1;
                (dir.0 == -1 || dir.1 == -1) as usize
            }
            '|' => {
                if dir.0 == 0 {
                    dir = (-1, 0);
                    lasers.push((pos, (1, 0)));
                    1 as usize
                } else {
                    0 as usize
                }
            }
            '-' => {
                if dir.1 == 0 {
                    dir = (0, -1);
                    lasers.push((pos, (0, 1)));
                    1 as usize
                } else {
                    0 as usize
                }
            }
            _ => (dir.0 == 0) as usize,
        };
        if visited[pos.0 as usize][pos.1 as usize][approach] {
            if let Some((p, d)) = lasers.pop() {
                pos = p;
                dir = d;
                continue 'main;
            } else {
                break 'main;
            }
        }
        visited[pos.0 as usize][pos.1 as usize][approach] = true;
    }

    let visited = visited.iter().fold(0, |acc, &row| {
        acc + row.iter().fold(0, |acc, &x| acc + (x[0] || x[1]) as usize)
    });
    println!("{}", visited);
}

fn draw_grid(grid: &[[[bool; 2]; N]; N]) {
    for row in grid {
        for cell in row {
            if cell[1] || cell[0] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
