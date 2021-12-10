#![allow(dead_code)]
#![allow(unused_imports)]
use helper::{get_input, get_raw_input, print_part_1, print_part_2, FromInput};
use std::collections::{HashMap, VecDeque};

const FILENAME: &str = env!("CARGO_PKG_NAME");
const OPENERS: [char; 4] = ['(', '[', '{', '<'];
const CLOSERS: [char; 4] = [')', ']', '}', '>'];
const TEST: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

pub fn main() {
    // let linters = TEST.split('\n').collect::<Vec<String>>();
    let linters = get_input(FILENAME);

    let (corrupt_lints, incomplete_lints) = part1(&linters);

    let corrupt_scores = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let corrupt_score = corrupt_lints.iter().map(|c| corrupt_scores[c]).sum::<i64>();

    let mut scores = part2(&incomplete_lints);

    scores.sort_unstable();
    let c = scores[(scores.len()) / 2usize];

    print_part_1(corrupt_score);
    print_part_2(c)
}

fn part1(linters: &[String]) -> (Vec<char>, Vec<VecDeque<char>>) {
    let mut corrupt_lints = Vec::new();
    let mut incomplete_lints = Vec::new();

    'outer: for l in linters.iter() {
        let mut openers = VecDeque::new();
        for c in l.chars() {
            if OPENERS.contains(&c) {
                openers.push_back(c);
                continue;
            }

            if CLOSERS.contains(&c) {
                if is_corrupted(&openers, c) {
                    corrupt_lints.push(c);
                    continue 'outer;
                }

                let _ = openers.pop_back();
            }
        }
        if is_incomplete(&openers) {
            incomplete_lints.push(openers)
        }
    }
    (corrupt_lints, incomplete_lints)
}

fn is_corrupted(openers: &VecDeque<char>, c: char) -> bool {
    openers.is_empty() || openers.iter().last().unwrap() != &opposite(c)
}

fn is_incomplete(openers: &VecDeque<char>) -> bool {
    !openers.is_empty()
}

fn part2(incomplete_lints: &[VecDeque<char>]) -> Vec<i64> {
    let incomplete_scores = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

    incomplete_lints
        .iter()
        .map(|l| score_lint(l, &incomplete_scores))
        .collect::<Vec<i64>>()
}

fn score_lint(l: &VecDeque<char>, incomplete_scores: &HashMap<char, i64>) -> i64 {
    l.iter()
        .rev()
        .map(|&c| opposite(c))
        .fold(0, |acc, elem| acc * 5 + incomplete_scores[&elem])
}

fn opposite(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("bad char {}", c),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
