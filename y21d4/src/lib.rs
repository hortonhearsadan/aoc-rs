#![allow(dead_code)]
#![allow(unused_imports)]
use helper::{get_input, get_raw_input, print_part_1, print_part_2, FromInput};
use std::str::FromStr;

const FILENAME: &str = env!("CARGO_PKG_NAME");
const MARK: i32 = -1;

const TEST: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

#[derive(Debug, Clone)]
struct Board {
    pub tiles: Vec<Vec<i32>>,
}

impl FromStr for Board {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b: Vec<Vec<i32>> = s
            .split('\n')
            .filter(|n| !n.is_empty())
            .map(|r| {
                r.split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect();
        Ok(Board { tiles: b })
    }
}

impl Board {
    fn mark(&mut self, x: i32) {
        'outer: for (i, r) in self.tiles.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c == x {
                    self.tiles[i][j] = MARK;
                    break 'outer;
                }
            }
        }
    }

    fn is_solved(&self) -> bool {
        self.is_any_row_solved() || self.is_any_column_solved()
    }

    fn is_any_row_solved(&self) -> bool {
        (0..self.tiles.len()).any(|row| self.is_row_solved(row))
    }

    fn is_any_column_solved(&self) -> bool {
        (0..self.tiles[0].len()).any(|col| self.is_column_solved(col))
    }

    fn is_column_solved(&self, col: usize) -> bool {
        self.tiles.iter().all(|r| r[col] == MARK)
    }

    fn is_row_solved(&self, row: usize) -> bool {
        self.tiles[row].iter().all(|&n| n == MARK)
    }

    fn score(&self, multiplier: i32) -> i32 {
        self.tiles
            .iter()
            .map(|r| r.iter().filter(|c| **c != MARK).sum::<i32>())
            .sum::<i32>()
            * multiplier
    }
}

pub fn main() {
    // let (numbers, mut boards) = parse_input();
    //
    // print_part_1(part_1(&numbers, &mut boards).unwrap_or(-999));
    //
    // print_part_2(part_2(&numbers, &mut boards).unwrap_or(-999))
}

// fn parse_input() -> (Vec<i32>, Vec<Board>) {
//     // let input_str: String = get_raw_input(FILENAME);
//     // let input = input_str.split_once("\n\n").unwrap();
//     //
//     // let numbers: Vec<_> = input
//     //     .0
//     //     .split(',')
//     //     .map(|s| s.parse::<i32>().unwrap())
//     //     .collect();
//     //
//     // let boards = Board::from_multiline(input.1);
//     //
//     // (numbers, boards)
// }

fn part_1(numbers: &[i32], boards: &mut Vec<Board>) -> Option<i32> {
    for n in numbers.iter() {
        for b in boards.iter_mut() {
            b.mark(*n);
            if b.is_solved() {
                return Some(b.score(*n));
            }
        }
    }
    None
}

fn part_2(numbers: &[i32], boards: &mut Vec<Board>) -> Option<i32> {
    let mut solved_boards = Vec::new();
    let l = boards.len();
    for n in numbers.iter() {
        for (i, b) in boards.iter_mut().enumerate() {
            if solved_boards.contains(&i) {
                continue;
            }

            b.mark(*n);

            if !b.is_solved() {
                continue;
            }

            if solved_boards.len() == l - 1 {
                return Some(b.score(*n));
            }
            solved_boards.push(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::Board;
    use std::str::FromStr;

    #[test]
    fn it_works() {
        let s = "1 2 3\n
        4 5 6\n
        7 8 9";

        let b = Board::from_str(s).unwrap();

        assert_eq!(b.tiles, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    }
}
