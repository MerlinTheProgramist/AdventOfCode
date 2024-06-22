use std::fs::read_to_string;

fn extrapolate(nums: Vec<i32>) -> i32 {
    let mut res = Vec::with_capacity(nums.len() - 1);
    let val: i32 = nums[0];
    let mut exit = true;
    for i in 1..nums.len() {
        res.push(nums[i] - nums[i - 1]);
        if nums[i] != val {
            exit = false;
        }
    }
    if !exit {
        println!("{:?}", res.clone());
        nums[nums.len() - 1].clone() + extrapolate(res)
    } else {
        val
    }
}

fn main() {
    let mut ans = 0;
    for line in read_to_string("p1.in").unwrap().lines() {
        let nums: Vec<_> = line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
        let res = extrapolate(nums);
        println!("{}\n", res);
        ans += res;
    }
    println!("{}", ans);
}
