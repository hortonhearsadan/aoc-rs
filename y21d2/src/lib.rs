#![allow(dead_code)]
use helper::{print_part_1, print_part_2, FromInput};
use num::complex::Complex;
use std::str::FromStr;

const FILENAME: &str = env!("CARGO_PKG_NAME");
const TEST: [Instruction; 6] = [
    Instruction {
        direction: Direction::Forward,
        amount: 5,
    },
    Instruction {
        direction: Direction::Down,
        amount: 5,
    },
    Instruction {
        direction: Direction::Forward,
        amount: 8,
    },
    Instruction {
        direction: Direction::Up,
        amount: 3,
    },
    Instruction {
        direction: Direction::Down,
        amount: 8,
    },
    Instruction {
        direction: Direction::Forward,
        amount: 2,
    },
];

enum Direction {
    Forward,
    Down,
    Up,
    Unknown,
}

struct Instruction {
    direction: Direction,
    amount: i32,
}

impl FromStr for Instruction {
    type Err = std::string::ParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let s: Vec<_> = string.split(' ').collect();
        let amount = s[1].parse::<i32>().unwrap();
        let direction = match s[0] {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => Direction::Unknown,
        };
        Ok(Instruction { direction, amount })
    }
}

pub fn main() {
    let instructions = Instruction::from_input(FILENAME);

    print_part_1(map1(&instructions));

    print_part_2(map2(&instructions));
}

fn map1(instructions: &[Instruction]) -> i32 {
    let mut position: Complex<i32> = Complex::new(0, 0);

    for i in instructions {
        action1(i, &mut position)
    }
    position.im * position.re
}

fn map2(instructions: &[Instruction]) -> i32 {
    let mut position: Complex<i32> = Complex::new(0, 0);
    let mut aim = 0;

    for i in instructions {
        action2(i, &mut position, &mut aim)
    }
    position.im * position.re
}

fn action1(i: &Instruction, position: &mut Complex<i32>) {
    match i.direction {
        Direction::Forward => position.re += i.amount,
        Direction::Up => position.im -= i.amount,
        Direction::Down => position.im += i.amount,
        Direction::Unknown => panic!("Bad direction"),
    };
}

fn action2(i: &Instruction, position: &mut Complex<i32>, aim: &mut i32) {
    match i.direction {
        Direction::Forward => {
            position.re += i.amount;
            position.im += *aim * i.amount
        }
        Direction::Up => *aim -= i.amount,
        Direction::Down => *aim += i.amount,
        Direction::Unknown => panic!("Bad direction"),
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
