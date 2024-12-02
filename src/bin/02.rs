use itertools::Itertools;
use rayon::prelude::{ParallelIterator, ParallelString};
use std::cmp::{max, min};
use std::str::FromStr;
use tinyvec::ArrayVec;

advent_of_code::solution!(2);

struct Line(ArrayVec<[u32; 10]>);

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s.split_whitespace().map(|i| i.parse::<u32>().unwrap());
        Ok(Self(ArrayVec::<[u32; 10]>::from_iter(line)))
    }
}

impl Line {
    fn check_intervals<'a>(iter: impl Iterator<Item = &'a u32>) -> bool {
        iter.tuple_windows().all(|(a, b)| interval_ok(a, b))
    }

    fn is_correct(&self) -> bool {
        let small_interval = Self::check_intervals(self.0.iter());
        small_interval && (self.0.is_sorted() || self.0.iter().rev().is_sorted())
    }

    /// is the line correct, when the element at index `skip` is skipped.
    fn is_correct_with_skip(&self, skip: usize) -> bool {
        let iter = self.0.iter().take(skip).chain(self.0.iter().skip(skip + 1));
        let small_interval = Self::check_intervals(iter.clone());
        small_interval && (iter.clone().is_sorted() || iter.rev().is_sorted())
    }
}

#[inline(always)]
fn interval_ok(a: &u32, b: &u32) -> bool {
    (1..=3).contains(&(max(a, b) - min(a, b)))
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .par_lines()
        .map(|l| l.parse::<Line>().unwrap())
        .filter(|l| l.is_correct())
        .count();
    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = input
        .par_lines()
        .map(|l| l.parse::<Line>().unwrap())
        .filter(|l| l.is_correct() || (0..l.0.len()).any(|i| l.is_correct_with_skip(i)))
        .count();
    Some(res as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
