#![allow(dead_code)]
#![allow(unused_imports)]
use helper::{print_part_1, print_part_2, FromInput};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

const FILENAME: &str = env!("CARGO_PKG_NAME");

const TEST: &str =
    "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

#[derive(Default, Debug)]
struct SevenSegmentDisplay {
    raw_signal_pattern: Vec<String>,
    raw_output_value: Vec<String>,
    zero: String,
    one: String,
    two: String,
    three: String,
    four: String,
    five: String,
    six: String,
    seven: String,
    eight: String,
    nine: String,
    a: char,
    b: char,
    c: char,
    d: char,
    e: char,
    f: char,
    g: char,
    parsed_signal_pattern: Vec<u8>,
    parsed_output_value: Vec<u8>,
}

impl FromStr for SevenSegmentDisplay {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pattern, output) = s.split_once(" | ").unwrap();
        let raw_pattern: Vec<_> = pattern
            .split(' ')
            .map(|s| s.chars().sorted().collect::<String>())
            .collect();
        let raw_output: Vec<_> = output
            .split(' ')
            .map(|s| s.chars().sorted().collect::<String>())
            .collect();

        Ok(SevenSegmentDisplay {
            raw_signal_pattern: raw_pattern,
            raw_output_value: raw_output,
            ..Default::default()
        })
    }
}

impl SevenSegmentDisplay {
    fn decode(&mut self) {
        self.deduce_numbers();
        self.deduce_segments();

        let digit_map = HashMap::from([
            (&self.zero, 0),
            (&self.one, 1),
            (&self.two, 2),
            (&self.three, 3),
            (&self.four, 4),
            (&self.five, 5),
            (&self.six, 6),
            (&self.seven, 7),
            (&self.eight, 8),
            (&self.nine, 9),
        ]);

        self.parsed_signal_pattern = self
            .raw_signal_pattern
            .iter()
            .map(|d| *digit_map.get(d).unwrap() as u8)
            .collect_vec();
        self.parsed_output_value = self
            .raw_output_value
            .iter()
            .map(|d| *digit_map.get(d).unwrap() as u8)
            .collect_vec();
    }

    fn deduce_numbers(&mut self) {
        self.one = self.deduce_one();
        self.four = self.deduce_four();
        self.seven = self.deduce_seven();
        self.eight = self.deduce_eight();
        self.nine = self.deduce_nine();
        self.six = self.deduce_six();
        self.zero = self.deduce_zero();
        self.three = self.deduce_three();
        self.five = self.deduce_five();
        self.two = self.deduce_two();
    }

    fn deduce_segments(&mut self) {
        self.a = self.deduce_a();
        self.b = self.deduce_b();
        self.c = self.deduce_c();
        self.d = self.deduce_d();
        self.e = self.deduce_e();
        self.f = self.deduce_f();
        self.g = self.deduce_g();
    }

    fn deduce_zero(&self) -> String {
        self.raw_signal_pattern
            .iter()
            .find(|&x| x.len() == 6 && x != &(self.nine) && x != &(self.six))
            .unwrap()
            .to_owned()
    }

    fn deduce_one(&self) -> String {
        self.raw_signal_pattern
            .iter()
            .find(|&x| x.len() == 2)
            .unwrap()
            .to_owned()
    }

    fn deduce_two(&self) -> String {
        self.raw_signal_pattern
            .iter()
            .find(|&x| x.len() == 5 && x != &(self.five) && x != &(self.three))
            .unwrap()
            .to_owned()
    }

    fn deduce_three(&self) -> String {
        self.raw_signal_pattern
            .iter()
            .find(|&x| x.len() == 5 && set(x).is_superset(&set(&(self.one))))
            .unwrap()
            .to_owned()
    }

    fn deduce_four(&self) -> String {
        self.raw_signal_pattern
            .iter()
            .find(|&x| x.len() == 4)
            .unwrap()
            .to_owned()
    }

    fn deduce_five(&self) -> String {
        self.raw_signal_pattern
            .iter()
            .find(|&x| x.len() == 5 && set(x).is_subset(&set(&(self.six))))
            .unwrap()
            .to_owned()
    }

    fn deduce_six(&self) -> String {
        self.raw_signal_pattern
            .iter()
            .find(|&x| x.len() == 6 && !set(x).is_superset(&set(&(self.one))))
            .unwrap()
            .to_owned()
    }

    fn deduce_seven(&self) -> String {
        self.raw_signal_pattern
            .iter()
            .find(|&x| x.len() == 3)
            .unwrap()
            .to_owned()
    }

    fn deduce_eight(&self) -> String {
        self.raw_signal_pattern
            .iter()
            .find(|&x| x.len() == 7)
            .unwrap()
            .to_owned()
    }

    fn deduce_nine(&self) -> String {
        self.raw_signal_pattern
            .iter()
            .find(|&x| x.len() == 6 && set(x).is_superset(&set(&(self.four))))
            .unwrap()
            .to_owned()
    }

    fn deduce_a(&self) -> char {
        set(&(self.seven))
            .difference(&set(&self.one))
            .next()
            .unwrap()
            .to_owned()
    }

    fn deduce_b(&self) -> char {
        set(&(self.four))
            .difference(&set(&self.three))
            .next()
            .unwrap()
            .to_owned()
    }

    fn deduce_c(&self) -> char {
        set(&(self.one))
            .difference(&set(&self.five))
            .next()
            .unwrap()
            .to_owned()
    }

    fn deduce_d(&self) -> char {
        set(&(self.eight))
            .difference(&set(&self.zero))
            .next()
            .unwrap()
            .to_owned()
    }

    fn deduce_e(&self) -> char {
        set(&(self.six))
            .difference(&set(&self.five))
            .next()
            .unwrap()
            .to_owned()
    }

    fn deduce_f(&self) -> char {
        set(&(self.one))
            .difference(&set(&self.two))
            .next()
            .unwrap()
            .to_owned()
    }

    fn deduce_g(&self) -> char {
        set(&(self.three))
            .difference(&set(&self.four))
            .find(|&x| x != &self.a)
            .unwrap()
            .to_owned()
    }

    fn output(&self) -> u32 {
        self.parsed_output_value
            .iter()
            .fold(0, |acc, elem| acc * 10 + *elem as u32)
    }
}

fn set(s: &str) -> HashSet<char> {
    s.chars().collect::<HashSet<char>>()
}

pub fn main() {
    // let mut displays = SevenSegmentDisplay::from_strings(TEST);
    let mut displays = SevenSegmentDisplay::from_input(FILENAME);
    for d in displays.iter_mut() {
        d.decode()
    }

    print_part_1(count_simples(&displays));

    print_part_2(sum_outputs(&displays));
}

fn sum_outputs(displays: &[SevenSegmentDisplay]) -> u32 {
    displays.iter().map(|d| d.output()).sum::<u32>()
}

fn count_simples(displays: &[SevenSegmentDisplay]) -> usize {
    let simples = [1, 4, 7, 8];
    displays
        .iter()
        .map(|d| {
            d.parsed_output_value
                .iter()
                .filter(|&s| simples.contains(s))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::SevenSegmentDisplay;
    use std::str::FromStr;

    #[test]
    fn it_works() {
        let s = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
        let mut ssd = SevenSegmentDisplay::from_str(s).unwrap();
        println!("{:?}", ssd);
        assert_eq!(
            ssd.raw_signal_pattern,
            vec![
                "be", "abcdefg", "bcdefg", "acdefg", "bceg", "cdefg", "abdefg", "bcdef", "abcdf",
                "bde"
            ]
        );
        assert_eq!(
            ssd.raw_output_value,
            vec!["abcdefg", "bcdef", "bcdefg", "bceg"]
        );

        ssd.decode();

        assert_eq!(ssd.zero, "abdefg");
        assert_eq!(ssd.one, "be");
        assert_eq!(ssd.two, "abcdf");
        assert_eq!(ssd.three, "bcdef");
        assert_eq!(ssd.four, "bceg");
        assert_eq!(ssd.five, "cdefg");
        assert_eq!(ssd.six, "acdefg");
        assert_eq!(ssd.seven, "bde");
        assert_eq!(ssd.eight, "abcdefg");
        assert_eq!(ssd.nine, "bcdefg");

        assert_eq!(ssd.a, 'd');
        assert_eq!(ssd.b, 'g');
        assert_eq!(ssd.c, 'b');
        assert_eq!(ssd.d, 'c');
        assert_eq!(ssd.e, 'a');
        assert_eq!(ssd.f, 'e');
        assert_eq!(ssd.g, 'f');

        println!("{:?}", ssd);
    }
}
