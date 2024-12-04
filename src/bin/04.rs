use crate::Letter::{A, M, S, X};
use itertools::Itertools;
use std::str::FromStr;

advent_of_code::solution!(4);

#[derive(Debug, Eq, PartialEq, Clone)]
enum Letter {
    X,
    M,
    A,
    S,
}

impl From<char> for Letter {
    fn from(s: char) -> Self {
        match s {
            'X' => X,
            'M' => M,
            'A' => A,
            'S' => S,
            _ => unreachable!(),
        }
    }
}

const XMAS: [Letter; 4] = [X, M, A, S];
const SAMX: [Letter; 4] = [S, A, M, X];

#[derive(Debug)]
struct Grid(Vec<Vec<Letter>>);

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = s
            .lines()
            .map(|l| l.chars().map(Letter::from).collect())
            .collect();
        Ok(Self(res))
    }
}

impl Grid {
    /// Count the number of occurrences of XMAS
    /// - in the rows
    /// - in the diagonal from top left to bottom right
    fn xmas_half_count(&self) -> u32 {
        let on_rows = self
            .0
            .iter()
            .map(|line| {
                line.iter()
                    .tuple_windows::<(_, _, _, _)>()
                    .filter(|word| [(&X, &M, &A, &S), (&S, &A, &M, &X)].contains(word))
                    .count() as u32
            })
            .sum::<u32>();
        let on_diagonal = (0..self.0.len())
            .map(|row_ind| {
                (0..self.0[0].len())
                    .filter(|col_ind| {
                        [XMAS, SAMX].iter().any(|word| {
                            (0..=4).zip(word.iter()).all(|(i, letter)| {
                                self.0.get(row_ind + i).map(|r| r.get(col_ind + i))
                                    == Some(Some(letter))
                            })
                        })
                    })
                    .count() as u32
            })
            .sum::<u32>();
        on_rows + on_diagonal
    }

    fn rotated(&self) -> Self {
        Self(
            (0..self.0[0].len())
                .map(|col| {
                    (0..self.0.len())
                        .rev()
                        .map(|row| self.0[row][col].clone())
                        .collect()
                })
                .collect(),
        )
    }

    fn cross_mass_count(&self) -> u32 {
        (1..self.0.len() - 1)
            .cartesian_product(1..self.0[0].len() - 1)
            .filter(|(row, col)| {
                self.0[*row][*col] == A
                    && [(&S, &M), (&M, &S)]
                        .contains(&(&self.0[row - 1][col - 1], &self.0[row + 1][col + 1]))
                    && [(&S, &M), (&M, &S)]
                        .contains(&(&self.0[row - 1][col + 1], &self.0[row + 1][col - 1]))
            })
            .count() as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input).unwrap();
    Some(grid.xmas_half_count() + grid.rotated().xmas_half_count())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(Grid::from_str(input).unwrap().cross_mass_count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
