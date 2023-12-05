use std::fs::read_to_string;

const N: usize = 203;

fn main() {
    let mut copies = [0;N];
    let mut card_count = [0;N];

    let mut lines = 0;
    for (line_n, line) in read_to_string("p1.in").unwrap().lines().enumerate() {
        lines+=1;
        
        let mut game_p = 0;
        let mut is_win = [false; 100];
        let line = line.split_once(':').unwrap().1;
        let (winning, select) = line.split_once('|').unwrap();
        for w in winning.split_whitespace() {
            // println!("{}", w);
            is_win[w.parse::<usize>().unwrap()] = true;
        }
        for s in select.split_whitespace() {
            let s = s.parse::<usize>().unwrap();
            if !is_win[s] {
                continue;
            }
            copies[line_n]+=1;
            if game_p == 0 {
                game_p = 1;
            } else {
                game_p *= 2;
            }
        }
        // println!("\n");
    }

    let mut ans = 0;
    for i in (0..lines).rev(){
        let mut a = 1;
        for c in 1..=copies[i]{
            a += card_count[i+c];
        }
        card_count[i] = a;
        println!("{}", a);
        ans += a;
    }

    println!("{}", ans);
}
