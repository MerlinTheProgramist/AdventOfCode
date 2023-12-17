use std::default::Default;
use std::fs::read_to_string;

fn count(cfg: &str, nums: &[usize]) -> i32 {
    // println!("{} {:?} ", cfg, nums);
    // if we have no more cfg but we expect more
    if cfg.is_empty() {
        // println!("is_empty: {}", nums.is_empty() as i32);
        return nums.is_empty() as i32;
    }
    // if there is no more numbers expected check if we have yet a hash in cfg
    if nums.is_empty() {
        return (!cfg.contains('#')) as i32;
    }

    let mut result = 0;
    // skip
    if cfg.starts_with(".") || cfg.starts_with("?") {
        result += count(&cfg[1..], nums);
    }
    // consume
    if cfg.starts_with("#") || cfg.starts_with("?") {
        if nums[0] <= cfg.len()          // there must be enough springs left
        && !cfg[..nums[0]].contains('.') // and can't be any empty space in the next nums[0] of cfg
        && (nums[0] == cfg.len() || cfg.as_bytes()[nums[0]] !=b'#')
        // and next char can't be another #
        {
            if nums[0] == cfg.len() {
                result += count(Default::default(), &nums[1..]);
            } else {
                result += count(&cfg[(nums[0] + 1)..], &nums[1..]);
            }
        }
    }
    return result;
}

fn main() {
    let mut ans = 0;
    for line in read_to_string("p.in").unwrap().lines() {
        let (cond, nums) = line.split_once(' ').unwrap();
        let nums: Vec<_> = nums
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let res = count(cond, &nums);
        // println!("res {}", res);
        ans += res;
    }
    println!("{}", ans);
}
