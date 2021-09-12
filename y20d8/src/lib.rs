use crate::Instruction::{Acc, Jmp, Nop};
use helper::{print_part_1, print_part_2, FromInput};
use std::str::FromStr;

const FILENAME: &str = env!("CARGO_PKG_NAME");

struct Op {
    pub op: Instruction,
    pub num: i32,
}

impl FromStr for Op {
    type Err = std::string::ParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut data = string.split_whitespace();
        let op = match data.next().unwrap() {
            "jmp" => Jmp,
            "acc" => Acc,
            "nop" => Nop,
            _ => Nop,
        };
        let num = data.next().unwrap().parse::<i32>().unwrap();

        Ok(Op { op, num })
    }
}

impl Op {
    fn switch(&mut self) {
        self.op = match self.op {
            Jmp => Nop.to_owned(),
            Nop => Jmp.to_owned(),
            _ => self.op,
        }
    }
}

pub enum State {
    Infinite(i32),
    Finite(i32),
}
#[derive(Copy, Clone, PartialOrd, PartialEq)]
enum Instruction {
    Jmp,
    Nop,
    Acc,
}

pub fn main() {
    let mut ops = Op::from_input(FILENAME);
    let state = run_game(&ops);
    let acc = match state {
        State::Infinite(v) => v,
        State::Finite(v) => v,
    };

    print_part_1(acc);

    let jmps_nops: Vec<_> = ops
        .iter()
        .enumerate()
        .filter(|(_, o)| o.op != Acc)
        .map(|(i, _)| i)
        .collect();
    for o in jmps_nops.iter() {
        {
            let op = &mut ops[*o];
            op.switch();
        }
        match run_game(&ops) {
            State::Infinite(_) => {}
            State::Finite(v) => {
                print_part_2(v);
                break;
            }
        }
        {
            let op = &mut ops[*o];
            op.switch();
        }
    }
}

fn run_game(ops: &[Op]) -> State {
    let mut acc = 0;
    let mut index: i32 = 0;
    let len = ops.len();
    let mut indices = Vec::with_capacity(len);
    loop {
        let current_op = &ops[index as usize];
        let operation = &current_op.op;
        match operation {
            Acc => {
                acc += current_op.num;
                index += 1;
            }
            Jmp => index += current_op.num,
            Nop => index += 1,
        }
        if indices.contains(&index) {
            return State::Infinite(acc);
        } else {
            indices.push(index);
        }
        if index as usize >= len {
            return State::Finite(acc);
        }
    }
}
