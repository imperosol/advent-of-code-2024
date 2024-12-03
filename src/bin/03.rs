use regex::Regex;
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
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))").unwrap();
    let mut dont = false;
    let res = re
        .find_iter(input)
        .map(|m| m.as_str())
        .filter_map(|s| match s {
            "do()" => {
                dont = false;
                None
            }
            "don't()" => {
                dont = true;
                None
            }
            s => {
                if dont {
                    None
                } else {
                    s.parse::<MulStmt>().map(|i| i.0).ok()
                }
            }
        })
        .sum();
    Some(res)
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
