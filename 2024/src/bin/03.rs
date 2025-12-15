advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    Some(
        re.captures_iter(input)
            .map(|caps| {
                let (_, [a, b]) = caps.extract();
                a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap()
            })
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let re =
        Regex::new(r"mul\((?P<a>\d{1,3}),(?P<b>\d{1,3})\)|(?P<do>do\(\))|(?P<dont>don't\(\)+)")
            .unwrap();
    let mut doing = true;
    Some(
        re.captures_iter(input)
            .map(|caps| {
                if caps.name("do").is_some() {
                    doing = true;
                } else if caps.name("dont").is_some() {
                    doing = false;
                } else if doing {
                    return caps
                        .name("a")
                        .map(|a| {
                            a.as_str().parse::<u64>().unwrap()
                                * caps.name("b").unwrap().as_str().parse::<u64>().unwrap()
                        })
                        .unwrap_or(0);
                }
                return 0;

                // a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap()
            })
            .sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
