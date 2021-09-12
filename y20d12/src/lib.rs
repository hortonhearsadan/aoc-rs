use crate::Direction::{East, Forward, Left, North, Right, South, West};
use helper::{print_part_1, FromInput};
use num::complex::Complex;
use std::str::FromStr;

const FILENAME: &str = env!("CARGO_PKG_NAME");

const NORTH: Complex<i32> = Complex::new(0, 1);
const SOUTH: Complex<i32> = Complex::new(0, -1);
const EAST: Complex<i32> = Complex::new(1, 0);
const WEST: Complex<i32> = Complex::new(-1, 0);
const LEFT: Complex<i32> = Complex::new(0, 1);
const RIGHT: Complex<i32> = Complex::new(0, -1);
const FORWARD: Complex<i32> = Complex::new(0, 0);

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl Direction {
    fn to_complex(&self) -> Complex<i32> {
        match self {
            North => NORTH,
            South => SOUTH,
            East => EAST,
            West => WEST,
            Forward => FORWARD,
            Left => LEFT,
            Right => RIGHT,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    value: i32,
}

impl FromStr for Instruction {
    type Err = std::string::ParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let (dir, num) = string.split_at(1);
        let direction = match dir {
            "N" => North,
            "S" => South,
            "W" => West,
            "E" => East,
            "L" => Left,
            "R" => Right,
            "F" => Forward,
            _ => panic!("bad instruction"),
        };
        let value = num.parse::<i32>().unwrap();
        Ok(Instruction { direction, value })
    }
}

pub fn main() {
    let instructions = Instruction::from_input(FILENAME);
    let position = travel(&instructions);
    print_part_1(position.re.abs() + position.im.abs())
}

fn travel(instructions: &[Instruction]) -> Complex<i32> {
    let mut position = Complex::new(0, 0);
    let mut bearing = EAST;
    for instr in instructions {
        match instr.direction {
            Left | Right => bearing *= instr.direction.to_complex().powi(instr.value / 90),
            Forward => position += bearing * instr.value,
            _ => position += instr.direction.to_complex() * instr.value,
        }
    }
    position
}

#[cfg(test)]
mod tests {
    use crate::Direction::{Forward, North, Right};
    use crate::{travel, turn, Instruction};
    use num::Complex;

    #[test]
    fn test_travel() {
        let instructions = vec![
            Instruction {
                direction: Forward,
                value: 10,
            },
            Instruction {
                direction: North,
                value: 3,
            },
            Instruction {
                direction: Forward,
                value: 7,
            },
            Instruction {
                direction: Right,
                value: 90,
            },
            Instruction {
                direction: Forward,
                value: 11,
            },
        ];
        let position = travel(&instructions);
        println!("{:?}", &instructions);
        assert_eq!(position, Complex::new(17, -8));
    }
    #[test]
    fn test_travel_ok() {
        let instructions = vec![
            Instruction {
                direction: Forward,
                value: 10,
            },
            Instruction {
                direction: Right,
                value: 180,
            },
            Instruction {
                direction: Forward,
                value: 10,
            },
        ];
        let position = travel(&instructions);
        println!("{:?}", &instructions);
        assert_eq!(position, Complex::new(0, 0));
    }
}
