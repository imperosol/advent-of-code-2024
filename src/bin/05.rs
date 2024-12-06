use std::cmp::Ordering;
use std::collections::HashMap;

use rayon::prelude::*;
use std::str::FromStr;
use tinyvec::ArrayVec;

advent_of_code::solution!(5);

/// Set of rules a list must respect.
#[derive(Debug)]
struct Rules(HashMap<u32, Vec<u32>>);

impl FromStr for Rules {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules = s
            .lines()
            .map(|l| l.split_once('|').unwrap())
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()));
        let mut res = HashMap::new();
        rules.for_each(|(key, val)| {
            res.entry(key)
                .and_modify(|v: &mut Vec<u32>| v.push(val))
                .or_insert(vec![val]);
        });
        Ok(Self(res))
    }
}

/// A list given in the second part of the input
struct List(ArrayVec<[u32; 25]>);

impl FromStr for List {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = s.split(',').map(|i| i.parse::<u32>().unwrap()).collect();
        Ok(Self(res))
    }
}

impl List {
    /// Return true if the list respects the rules
    /// false otherwise
    fn is_valid(&self, rules: &Rules) -> bool {
        self.0
            .iter()
            .enumerate()
            .all(|(idx, n)| match rules.0.get(n) {
                Some(rule) => !self.0[..idx].iter().any(|i| rule.contains(i)),
                None => true,
            })
    }

    /// Take an invalid list and sort it in-place
    /// with the given rules
    fn fix(&mut self, rules: &Rules) {
        // This sorting method isn't transitive at all
        // and doesn't give the same results as the example.
        // However, the element that should be in the middle
        // will always be the right one, and that's
        // the only thing we care about.
        self.0.sort_by(|a, b| match rules.0.get(b) {
            None => Ordering::Equal,
            Some(rule) => match rule.contains(a) {
                true => Ordering::Less,
                false => Ordering::Equal,
            },
        });
    }

    fn middle_element(&self) -> u32 {
        self.0[self.0.len() / 2]
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, lists) = input.split_once("\n\n").unwrap();
    let rules = rules.parse::<Rules>().unwrap();
    let res = lists
        .par_lines()
        .filter_map(|l| {
            let l = List::from_str(l).unwrap();
            match l.is_valid(&rules) {
                true => Some(l.middle_element()),
                false => None,
            }
        })
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, lists) = input.split_once("\n\n").unwrap();
    let rules = rules.parse::<Rules>().unwrap();
    let res = lists
        .par_lines()
        .filter_map(|l| {
            let mut list = List::from_str(l).unwrap();
            match list.is_valid(&rules) {
                true => None,
                false => {
                    list.fix(&rules);
                    Some(list.middle_element())
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
