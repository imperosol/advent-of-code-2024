use itertools::Itertools;
use rayon::prelude::{ParallelSliceMut};
use std::cmp::{max, min};

advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|l| {
            let mut split = l.split_whitespace();
            (
                split.next().unwrap().parse::<u32>().unwrap(),
                split.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .unzip();

    left.par_sort_unstable();
    right.par_sort_unstable();
    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (left, right) = parse_input(input);
    let res = left
        .iter()
        .zip(right.iter())
        .map(|(i, j)| max(i, j) - min(i, j))
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_input(input);
    let (mut left, mut right) = (left.into_iter().peekable(), right.into_iter().peekable());
    let mut res = 0;
    while let Some(i) = left.next() {
        let count_i = 1 + left.peeking_take_while(|&n| n == i).count();
        // remove elements of right that are < i
        // (both iterators are sorted, so those cannot be in left)
        // `.count()` is here just to force the consumption of the iterator
        let _ = right.peeking_take_while(|&n| n < i).count();
        if right.peek() == Some(&i) {
            res += (count_i * right.peeking_take_while(|&n| n == i).count()) as u32 * i;
        }
    }
    Some(res)
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
