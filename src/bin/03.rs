use regex::{Regex, RegexBuilder};
use std::str::FromStr;

advent_of_code::solution!(3);

struct MulStmt(u32);

impl FromStr for MulStmt {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = s[4..]
            .trim_end_matches(')')
            .splitn(2, ',')
            .map(|i| i.parse::<u32>().unwrap())
            .product();
        Ok(Self(res))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let res = re
        .find_iter(input)
        .map(|m| m.as_str().parse::<MulStmt>().unwrap().0)
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    // just remove everything after a "don't()" that isn't closed by a "do()"
    // then do part 1 with the cleaned string
    let re = RegexBuilder::new(r"don't\(\)(.*?)(do\(\)|$)")
        .dot_matches_new_line(true)
        .build()
        .unwrap();
    part_one(&re.replace_all(input, ""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(188741603));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(67269798));
    }
}
