#![allow(dead_code)]
#![allow(unused_imports)]
use helper::{print_part_1, print_part_2, FromInput, get_raw_input};
use std::str::FromStr;
use std::collections::{HashMap, VecDeque};

const FILENAME: &str = env!("CARGO_PKG_NAME");
const OPENERS: [char;4] = ['(','[','{','<'];
const CLOSERS: [char;4] = [')',']','}','>'];

const TEST: &str ="[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

#[derive(Default,Debug, PartialEq)]
struct Linter {
    line: String,
    round: i32,
    square: i32,
    squiggly: i32,
    pointy: i32,
}

impl FromStr for Linter {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Linter{line:s.to_owned(), ..Default::default()})
    }
}

impl Linter {
    fn lint_char(&mut self, c: char) {
        match c {
            '(' => self.round +=1,
            ')' => self.round -=1,
            '[' => self.square +=1,
            ']' => self.square -=1,
            '{' => self.squiggly +=1,
            '}' => self.squiggly -=1,
            '<' => self.pointy +=1,
            '>' => self.pointy -=1,
            _ => panic!("bad char {}", c)
        }
    }

    fn check_corrupt(&self) -> Option<char> {
        if self.round < 0 {
            return Some(')')
        }

        if self.square < 0 {
            return Some(']')
        }

        if self.squiggly < 0 {
            return Some('}')
        }

        if self.pointy < 0 {
            return Some('>')
        }

        None
    }

    fn check_incomplete(&self) -> Option<char> {
        if self.round > 0 {
            return Some(')')
        }

        if self.square > 0 {
            return Some(']')
        }

        if self.squiggly > 0 {
            return Some('}')
        }

        if self.pointy > 0 {
            return Some('>')
        }

        None

    }



}

pub fn main() {
    // let s = TEST;
    let s = get_raw_input(FILENAME);
    let linters = Linter::from_strings(&s);
    let mut corrupt_linters = Vec::new();
    let mut corrupt_lints = Vec::new();
    let mut incomplete_linters = Vec::new();
    let mut incomplete_lints = Vec::new();
    for l in linters.iter() {
        let mut openers = VecDeque::new();
        // let mut closers = VecDeque::new();
        // let k = l.clone();
        let chars = l.line.clone();
        for c in chars.chars() {
            if OPENERS.contains(&c) {
                openers.push_back(c)
            }
            else if CLOSERS.contains(&c) {
                if openers.is_empty() {
                    corrupt_linters.push(l);
                    corrupt_lints.push(c);
                    break
                }

                else if openers.iter().last().unwrap() != &opposite(c)  {
                    corrupt_linters.push(l);
                    corrupt_lints.push(c);
                    break

                }
                let _ = openers.pop_back();
            }
        }
        if !corrupt_linters.contains(&l) &&  !openers.is_empty() {
            incomplete_linters.push(l);
            incomplete_lints.push(openers);
        }

    }

    let m = HashMap::from([(')',3),(']',57),('}',1197),('>',25137)]);

    let corrupt_score = corrupt_lints.iter().map(|c| m[c]).sum::<i32>();

    let mut close_lints = Vec::new();

    let n = HashMap::from([(')',1),(']',2),('}',3),('>',4)]);
    for l in incomplete_lints.iter() {
        let mut opp: Vec<char> = l.iter().map(|&c| opposite(c)).collect();
        opp.reverse();
        let mut score: i64 = 0;
        for o in opp.iter() {
            score *=5;
            score += n[o]
        }
        close_lints.push(score)
    }

    close_lints.sort_unstable();
     let c = close_lints[(close_lints.len()) / 2usize];


    let incomplete_score = incomplete_lints.iter().map(|v| v.iter().map(|c| m[&opposite(*c)]).sum::<i32>()).sum::<i32>();
    print_part_1(corrupt_score);
    print_part_2(c);

}

fn opposite(c: char) -> char {
    match c { '(' => ')',
    '[' => ']',
    '{' => '}',
    '<' => '>',
    ')' => '(',
    ']' => '[',
    '}' => '{',
    '>' => '<',
    _ => panic!("bad char {}",c)}

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
