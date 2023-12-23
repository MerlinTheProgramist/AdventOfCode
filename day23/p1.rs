use std::fs::read_to_string;
use std::collections::{HashSet, HashMap};
use std::default::Default;

const N:usize = 141;

type Point = (usize,usize);
type Graph = HashMap::<Point,HashMap<Point,usize>>;

fn main() {
    let mut grid = [['\0';N];N];
    for (y, line) in read_to_string("p.in").unwrap().lines().enumerate() {
        for (x,c) in line.chars().enumerate(){
            grid[y][x] = c;
        }
    }

    let start = (0, grid[0].iter().position(|&x| x=='.').unwrap());
    let end = (N-1, grid[N-1].iter().position(|&x| x=='.').unwrap());

    let mut points = Vec::from([start,end]); // points with at least 3 neighbors

    for (y,row) in grid.iter().enumerate(){
        for (x,&p) in row.iter().enumerate(){
            if p =='#'{
                continue;
            }
            let mut neighbors = 0;
            for dir in [(-1,0),(1,0), (0,-1), (0,1)]{
                let pos = (y as i32 + dir.0, x as i32 +dir.1);
                // if pos in range
                if pos.0>=0 && pos.1>=0 && pos.0<N as i32 && pos.1<N as i32 && grid[pos.0 as usize][pos.1 as usize] !='#'{
                    neighbors += 1;
                }
            }
            if neighbors >=3{
                points.push((y,x));
            }
        }
    }

    let mut graph = Graph::new();
    for point in points.iter(){
        graph.insert(*point, HashMap::new());
    }
    
    for sp in points.clone().into_iter(){
        let mut stack = Vec::from([(0,sp)]);
        let mut seen = HashSet::from([sp]);
        while let Some((n,p)) = stack.pop(){
            if n!=0 && points.contains(&p){
                graph.get_mut(&sp).unwrap().insert(p, n);
                continue;
            }
            for dir in dirs(grid[p.0][p.1]){
                let pos = (p.0 as i32+dir.0, p.1 as i32+dir.1);
                if pos.0>=0 && pos.1>=0 && pos.0<N as i32 && pos.1<N as i32 
                    && grid[pos.0 as usize][pos.1 as usize]!='#' 
                    && !seen.contains(&(pos.0 as usize, pos.1 as usize)){
                    let pos = (pos.0 as usize, pos.1 as usize);
                    stack.push((n+1, pos));
                    seen.insert(pos);
                }
            }
        }
    }
    // println!("{:?}", graph);
    
    println!("{}", dfs(&graph, start, &end, &mut Default::default()));
}

fn dirs(c:char)->Vec<(i32,i32)>{
    match c{
        '^' => vec![(-1,0)],
        'v' => vec![(1,0)],
        '>' => vec![(0,1)],
        '<' => vec![(0,-1)],
        '.' => vec![(1,0),(-1,0),(0,-1),(0,1)],
        _ => panic!("Invalid dir argument"),
    }
}

fn dfs(graph:&Graph,pt:Point, end:&Point, seen:&mut HashSet<Point>)->usize{
    if pt == *end{
        return 0;
    }
    let mut dist = 0;
    seen.insert(pt);
    for (p,n) in graph.get(&pt).unwrap().iter(){
        if !seen.contains(p){
            dist = dist.max(dfs(graph, *p, end) + n);
        }
    }
    seen.remove(&pt);
    dist
}
