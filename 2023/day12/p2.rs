use std::default::Default;
use std::fs::read_to_string;
use std::collections::HashMap;

fn count(cfg: &str, nums: &[usize], mem:&mut HashMap<(usize,usize), i64>) -> i64 {
    if let Some(result) = mem.get(&(cfg.len(), nums.len())){
        return *result;
    }
    // println!("{} {:?} ", cfg, nums);
    // if we have no more cfg but we expect more
    if cfg.is_empty() {
        // println!("is_empty: {}", nums.is_empty() as i64);
        return nums.is_empty() as i64;
    }
    // if there is no more numbers expected check if we have yet a hash in cfg
    if nums.is_empty() {
        return (!cfg.contains('#')) as i64;
    }

    let mut result = 0;
    // skip
    if cfg.starts_with(".") || cfg.starts_with("?") {
        result += count(&cfg[1..], nums, mem);
    }
    // consume
    if cfg.starts_with("#") || cfg.starts_with("?") {
        if nums[0] <= cfg.len()          // there must be enough springs left
        && !cfg[..nums[0]].contains('.') // and can't be any empty space in the next nums[0] of cfg
        && (nums[0] == cfg.len() || cfg.as_bytes()[nums[0]] !=b'#')
        // and next char can't be another #
        {
            if nums[0] == cfg.len() {
                result += count(Default::default(), &nums[1..], mem);
            } else {
                result += count(&cfg[(nums[0] + 1)..], &nums[1..], mem);
            }
        }
    }
    mem.insert((cfg.len(), nums.len()), result);
    return result;
}

const N:usize = 5;

fn main() {
    let mut ans = 0;
    for line in read_to_string("p.in").unwrap().lines() {
        let (cond, nums) = line.split_once(' ').unwrap();
        let cond = [cond;N].join("?");
        let nums = nums
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>().repeat(N);

        let mut mem = HashMap::<(usize,usize), i64>::new();
        
        let res = count(&cond, &nums, &mut mem);
        // println!("{} {:?} \n {}", cond, nums, res);
        ans += res;
    }
    println!("{}", ans);
}
