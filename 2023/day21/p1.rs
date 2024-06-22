use std::fs::read_to_string;

const N:usize = 131;
const STEPS:usize = 64;

fn main() {
    let mut grid = [[false;N];N];
    let mut tiles = Vec::new();
    for (y, line) in read_to_string("p.in").unwrap().lines().enumerate() {
        for (x,c) in line.chars().enumerate(){
            grid[y][x] = if c == 'S'{
                tiles.push((y,x));
                false
            }else{
                c == '.'
            };
        }
    }
    let mut ans = tiles.len();
    for i in 1..=STEPS{
        
        let mut new_tiles = Vec::new();
        for tile in tiles.iter(){
            for dir in [(1,0), (-1,0), (0,1), (0,-1)]{
                let n = (tile.0 as i32 + dir.0, tile.1 as i32 + dir.1);
                if n.0 >=0 
                && n.0 < N as i32
                && n.1 >= 0 
                && n.1 < N as i32
                && grid[n.0 as usize][n.1 as usize]{
                    grid[n.0 as usize][n.1 as usize] = false;
                    new_tiles.push((n.0 as usize, n.1 as usize));
                }
            }
        }
        tiles = new_tiles;
        
        if i%2==0{
            ans += tiles.len();
        }
    }
    println!("{}", ans);
}
