use std::fs::read_to_string;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::default::Default;
use std::collections::HashMap;

const TURNS:usize = 1000000000;

fn main() {
    let mut board:Vec<Vec<char>> = Default::default();
    for line in read_to_string("p.in").unwrap().lines(){
        board.push(line.chars().collect());
    }

    let mut seen = HashMap::new();
    let mut scores = Vec::new();
    for turn in 0..TURNS{
        // north
        roll_north(&mut board);
        board = rot_right(board.clone());

        // west
        roll_north(&mut board);
        board = rot_right(board.clone());
        // south
        roll_north(&mut board);
        board = rot_right(board.clone());
        // east
        roll_north(&mut board);
        board = rot_right(board.clone());

        scores.push(weight(&mut board));

        // println!("{score}");
        match seen.entry(board.clone()){
            Vacant(e) => _ = e.insert(turn),
            Occupied(e) => {
                // draw(&mut board);
                println!("{}", 
                    scores[e.get()+(TURNS-e.get()-1).max(0)%(turn-e.get())]);
                return;
            }
        }
    }
}

fn draw(v:&mut Vec<Vec<char>>){
    for row in v{
        for c in row{
            print!("{c}");
        }
        println!("");
    }
    println!("");
}

fn rot_right(mut v: Vec<Vec<char>>)->Vec<Vec<char>>{
    
    // transpose
    let len = v.len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    v = (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    
    for row in v.iter_mut(){
        row.reverse();
    }
    v
}

fn weight(board: &mut Vec<Vec<char>>)->usize{
    let mut w = 0;
    let n = board.len();
    for i in 0..board.len(){
        for c in &board[i]{
            match c{
                'O' => w+=n-i,
                _ => ()
            }
        }
    }
    return w;
}

fn roll_north(board: &mut Vec<Vec<char>>) {
    /*
    for i in range(ncols):
        rocks = [-1] + list(np.where(array[:, i] == -1)[0]) + [None]
        for j in range(len(rocks) - 1):
            left, right = rocks[j] + 1, rocks[j + 1]
            array[left:right, i] = np.sort(array[left:right, i])
    return array
    */
    let mut pos = vec![0;board.len()];
    for y in 0..board.len(){
        for x in 0..board.len(){
            match board[y][x]{
                'O' => {
                    board[y][x] = '.';
                    board[pos[x]][x] = 'O'; 
                    pos[x]+=1;
                },
                '#' => pos[x] = y+1,
                _ => (),
            }
        }
    }
}
