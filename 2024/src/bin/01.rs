advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let (mut arr_l, mut arr_r) = (Vec::new(), Vec::new());
    for line in input.lines() {
        let (l, r) = line.split_once("   ").unwrap();
        arr_l.push(l.parse::<u64>().unwrap());
        arr_r.push(r.parse::<u64>().unwrap());
    }
    arr_l.sort();
    arr_r.sort();
    let diff_sum = arr_l
        .iter()
        .zip(arr_r.iter())
        .fold(0, |acc, (&l, &r)| acc + l.abs_diff(r));
    Some(diff_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut arr_l, mut arr_r) = (Vec::new(), Vec::new());
    for line in input.lines() {
        let (l, r) = line.split_once("   ").unwrap();
        arr_l.push(l.parse::<u64>().unwrap());
        arr_r.push(r.parse::<u64>().unwrap());
    }
    arr_l.sort();
    arr_r.sort();

    let mut sum = 0;
    let mut i = 0;

    let mut l_iter = arr_l.iter().peekable();
    'main: while let Some(&l) = l_iter.next() {
        let mut times = 1;
        while l_iter.peek() == Some(&&l) {
            times += 1;
            l_iter.next();
        }
        while arr_r[i] <= l {
            sum += times * l * (arr_r[i] == l) as u64;
            i += 1;
            if i >= arr_r.len() {
                break 'main;
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
