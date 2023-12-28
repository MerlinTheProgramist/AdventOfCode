use std::fs::read_to_string;

fn main() {
    let mut ans = 0;
    for line in read_to_string("p.in").unwrap().lines() {
        let mut first = None;
        let mut last = None;
        for c in line.chars(){
            if let Some(d) = c.to_digit(10){
                if first.is_none(){
                    first = Some(d);
                }else{
                    last = Some(d);
                }
            }
        }
        let first = first.expect("This map is not valid, it is advised to contant your nearby trebuchet advisor");
        if let Some(last) = last{
            ans += first*10 + last;
        }else{
            ans += first*11;
        }
    }
    println!("{}", ans);
}
