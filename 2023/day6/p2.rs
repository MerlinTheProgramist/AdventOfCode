use std::fs::read_to_string;

fn main() {
    let file = read_to_string("p1.in").unwrap();
    let lines = file.lines().collect::<Vec<_>>();
    let t = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse::<f64>().unwrap();
    let r = lines[1]
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse::<f64>().unwrap();

    // quadratic:
    // x*(t-x) > r <=> 0 > x^2 - tx + r
    // x = \frac{t \pm \sqrt{t^2-4r}}{2}
    // ans = \floor{x_1-x_0}
    let delta = (t * t - 4.0 * r).sqrt();
    if delta == 0.0 {
        // no values bolow 0, no solution
        return;
    }

    let x0 = (t - delta) / 2.0;
    let x1 = (t + delta) / 2.0;

    let m = ((x1 - 1.0).ceil() - x0.floor()) as i64;
    println!("d={}, x0={}, x1={} -> {}", delta, x0, x1, m);

    println!("{}", m);
}
