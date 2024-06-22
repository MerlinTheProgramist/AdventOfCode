use std::fs::read_to_string;
use std::ops::Range;

// range
// diff
struct Trans {
    start: i64,
    end: i64,
    diff: i64,
}

impl Trans {
    // 1 0 2
    pub fn new(from: i64, to: i64, len: i64) -> Self {
        Self {
            start: from,
            end: from + len,
            diff: to - from,
        }
    }
    // return range that mached with the translation and ranges that were cut off
    pub fn split(&self, range: &Range<i64>) -> (Option<Range<i64>>, Vec<Range<i64>>) {
        // if range didn't mach the translation -> return (None, [range])
        if range.end <= self.start || self.end <= range.start {
            return (None, vec![range.clone()]);
        }
        // if range is smaller than translation -> return (ranger, [])
        if self.start <= range.start && range.end <= self.end {
            return (Some(self.translate(range)), vec![]);
        }
        // if range was bigger on the right -> return ()
        if self.start <= range.start && self.end <= range.end {
            return (
                Some(self.translate(&(range.start..self.end))),
                vec![self.end..range.end],
            );
        }
        // if range was bigger on the left
        if range.start <= self.start && range.end <= self.end {
            return (
                Some(self.translate(&(self.start..range.end))),
                vec![range.start..self.start],
            );
        }

        // range was bigger on both sides
        return (
            Some(self.translate(&(self.start..self.end))),
            vec![range.start..self.start, self.end..range.end],
        );
    }
    fn translate(&self, r: &Range<i64>) -> Range<i64> {
        (r.start + self.diff)..(r.end + self.diff)
    }
}

fn main() {
    let binding = read_to_string("p1.in").unwrap();
    let mut f = binding.lines();

    let seeds: Vec<_> = f
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut ranges = Vec::<Range<i64>>::new();
    for i in (0..seeds.len()).step_by(2) {
        ranges.push(seeds[i]..seeds[i] + seeds[i + 1]);
        // println!("input: {:?}", ranges.last().unwrap());
    }

    let mut new_ranges = Vec::<Range<i64>>::new();
    for line in f.skip(1) {
        if line.contains(':') {
            continue;
        }
        if line.is_empty() {
            for r in &ranges {
                if !r.is_empty() {
                    new_ranges.push(r.clone());
                }
            }
            ranges.extend(new_ranges);
            new_ranges = Vec::with_capacity(ranges.len());

            println!("\n");
            continue;
        }

        let split: Vec<_> = line.split_whitespace().collect();
        let tr = Trans::new(
            split[1].parse::<i64>().unwrap(),
            split[0].parse::<i64>().unwrap(),
            split[2].parse::<i64>().unwrap(),
        );
        println!(
            "translate with <{:?}, diff:{:?}>:",
            tr.start..tr.end,
            tr.diff
        );

        let mut leftovers = Vec::new();
        for range in ranges {
            let (moved, left) = tr.split(&range);
            if let Some(m) = moved {
                println!("{:?} -> {:?} + remainder: {:?}", range, m, left);
                new_ranges.push(m.clone());
            }
            leftovers.extend(left);
        }
        ranges = leftovers;
    }

    ranges.extend(new_ranges);
    // println!("end: {:?}", &ranges);
    let mut ans = i64::MAX;
    for r in &ranges {
        ans = ans.min(r.start);
    }

    println!("{}", ans);
}
