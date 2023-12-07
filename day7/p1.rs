use std::fs::read_to_string;

struct Hand {
    hand: Vec<u8>,
    bid: i32,
    order: u8,
}

const cards: [u8] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

fn main() {
    let mut ans = 0;
    let mut hands = Vec::new();
    for line in read_to_string("p1.in").unwrap().lines() {
        let (hand, bid) = line.split_once(' ').unwrap();

        let bid = bid.parse::<i32>();
        let points = hand
            .map(|c| cards.iter().position(|&r| r == c).unwrap())
            .collect::<Vec<_>>();

        let counts = HashMap::<u8, i32>::new();
        for p in points {
            counts[p] += 1;
        }
    }
    println!("{}", ans);
}

fn eval_hand(hand: &str) {
    // five of kind
    if counts.len() == 1 {}
}
