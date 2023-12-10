use std::env;
use std::fs::read_to_string;
use std::fmt;

const N: usize = 140;

type Connect = [(i8, i8); 2];

struct Pipe {
    pos: (usize, usize),
    from: (i8, i8),
}

#[derive(Copy, Clone, PartialEq)]
enum Shape{
    Border,
    Point, 
    Other,
    Consumed,
}
impl fmt::Debug for Shape{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Self::Border => write!(f, "+"),
            Self::Point => write!(f, "."),
            Self::Other => write!(f, " "),
            Self::Consumed => write!(f, "c"),
        }
    }
}

const L: Connect = [(-1, 0), (0, 1)];
const F: Connect = [(1, 0), (0, 1)];
const J: Connect = [(-1, 0), (0, -1)];
const _7: Connect = [(0, -1), (1, 0)];
const I: Connect = [(-1, 0), (1, 0)];
const H: Connect = [(0, -1), (0, 1)];
const NONE: Connect = [(0, 0), (0, 0)];

fn main() {
    let args: Vec<_> = env::args().collect();
    let file_name = if args.len() == 2 {
        args[1].as_str()
    } else {
        "p1.in"
    };


    const border_crossings:[char;3] = ['J', 'L', '|'];
    
    let mut grid = [[&NONE; N]; N];
    let mut chars = [['.'; N]; N];
    let mut shape = [[Shape::Point; N]; N];

    let mut start = (0, 0);
    for (y, line) in read_to_string(file_name).unwrap().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (y, x);
            } else {
                grid[y][x] = pipe(c);
            }
            chars[y][x] = c;
        }
    }

    let mut curr: (&Connect, (usize, usize)) = (&L, start);
    let mut arrival_dir = 0;

    for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        // if neighboor pipe allows connection from this
        let pos = (
            (curr.1 .0 as i32 + dir.0 as i32) as usize,
            (curr.1 .1 as i32 + dir.1 as i32) as usize,
        );
        // If was on the edge, secure from overflow
        if pos.0 > N || pos.1 > N{
            continue;
        }

        let pipe = grid[pos.0][pos.1];
        let neg_dir = neg_dir(&dir);
        println!("checked {:?}, {:?}", pipe, neg_dir);
        if let Some(arrival) = pipe.iter().position(|&p| p == neg_dir) {
            curr = (pipe, pos);
            arrival_dir = arrival;
            break;
        }
    }
    
    println!("starting with: {:?}", curr);
    shape[curr.1.0][curr.1.1] = if border_crossings.contains(&chars[curr.1.0][curr.1.1]){
         Shape::Border
    }else{
         Shape::Other   
    };
    
    
    loop {
        // get the other dir
        let dir = curr.0[(arrival_dir == 0) as usize];
        let pos = (
            (curr.1 .0 as i32 + dir.0 as i32) as usize,
            (curr.1 .1 as i32 + dir.1 as i32) as usize,
        );

        shape[pos.0][pos.1] = if border_crossings.contains(&chars[pos.0][pos.1]){
             Shape::Border
        }else{
             Shape::Other   
        };

        if pos == start {
            break;
        }

        // println!("try: {:?}", pos);
        let pipe = grid[pos.0][pos.1];
        let neg_dir = neg_dir(&dir);
        let arrival = pipe.iter().position(|&p| p == neg_dir).unwrap();
        curr = (pipe, pos);
        arrival_dir = arrival;
        println!("{:?}", pos);
    }

    // Special case for starting point 
    // shape[start.0][start.1] = if grid[start.0-1][start.1].contains(&(1,0)) 
    //     &&    (grid[start.0+1][start.1].contains(&(-1,0))
    //         || grid[start.0][start.1+1].contains(&(0,-1))
    //         || grid[start.0][start.1-1].contains(&(1,1))){
    //     Shape::Border // start in (|, L, J)
    // }else{
    //     Shape::Other
    // };
    shape[start.0][start.1] = Shape::Border;
    
    let mut points_inside = 0;
    for (y,row) in shape.iter().enumerate(){
        for (i,s) in row.iter().enumerate(){
            if *s == Shape::Point{
                // if raycast hit odd number of times
                if (row[..i].iter().filter(|&p| *p==Shape::Border).count()) % 2 == 1{
                    points_inside += 1;
                    print!("\x1b[31;1;4mI\x1b[0m");
                }else{
                    print!("O");
                }
            }else if *s == Shape::Border{
                print!("\x1b[93m{}\x1b[0m", chars[y][i]);
            }else{
                print!(" ");
            }
        }
        println!("");
    }
    // for row in shape{
    //     for s in row{
    //     }
    //     println!("");
    // }

    println!("{}", points_inside);
}

#[inline(always)]
fn pipe(c: char) -> &'static Connect {
    match c {
        'L' => &L,
        'F' => &F,
        '|' => &I,
        'J' => &J,
        '7' => &_7,
        '-' => &H,
        '.' => &NONE,
        _ => panic!("UNEXPECTED CHARACTER"),
    }
}
#[inline(always)]
fn neg_dir(dir: &(i8, i8)) -> (i8, i8) {
    (-dir.0, -dir.1)
}
