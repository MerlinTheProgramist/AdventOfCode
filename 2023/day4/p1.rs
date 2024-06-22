use std::fs::read_to_string;

fn main() {
    let mut ans = 0;
    for line in read_to_string("p1.in").unwrap().lines() {
        let mut game_p = 0;
        let mut is_win = [false; 100];
        let line = line.split_once(':').unwrap().1;
        let (winning, select) = line.split_once('|').unwrap();
        for w in winning.split_whitespace() {
            println!("{}", w);
            is_win[w.parse::<usize>().unwrap()] = true;
        }
        for s in select.split_whitespace() {
            if !is_win[s.parse::<usize>().unwrap()] {
                continue;
            }
            if game_p == 0 {
                game_p = 1;
            } else {
                game_p *= 2;
            }
        }
        ans += game_p;
        println!("\n");
    }
    println!("{}", ans);
}
