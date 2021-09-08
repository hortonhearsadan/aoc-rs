use helper::{print_part_1, print_part_2, FromInput};
use std::str::FromStr;
const FILENAME: &str = env!("CARGO_PKG_NAME");

struct Group {
    answers: String,
}

impl FromStr for Group {
    type Err = std::num::ParseIntError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let answers = string.to_owned();
        Ok(Self { answers })
    }
}
fn main() {
    let groups = Group::from_multiline_input(FILENAME);
    let count = groups
        .iter()
        .map(|g| count_anyone(g.answers.as_str()))
        .sum::<usize>();
    print_part_1(count);
    let count = groups
        .iter()
        .map(|g| count_everyone(g.answers.as_str()))
        .sum::<usize>();
    print_part_2(count);
}

fn count_anyone(string: &str) -> usize {
    let mut v: Vec<_> = string.replace('\n', "").as_str().chars().collect();
    v.sort_unstable();
    v.dedup();
    v.len()
}

fn count_everyone(string: &str) -> usize {
    let v = string.replace('\n', "");

    let mut u = v.chars().collect::<Vec<char>>();
    let len = string.split_terminator('\n').count();
    u.sort_unstable();
    u.dedup();
    u.iter().filter(|c| v.matches(**c).count() == len).count()
}

#[cfg(test)]
mod tests {
    use crate::{count_anyone, count_everyone};

    #[test]
    fn test_count_anyone() {
        let s = "abc";
        assert_eq!(3, count_anyone(&s));
        let s = "a\nb\nc";
        assert_eq!(3, count_anyone(&s));
        let s = "ab\nbc";
        assert_eq!(3, count_anyone(&s));
        let s = "a\na\na\na";
        assert_eq!(1, count_anyone(&s));
        let s = "b";
        assert_eq!(1, count_anyone(&s));
        let s = "tbsafengq\neabqsftgndmx\nbeaintysgqhof\n";
        assert_eq!(16, count_anyone(&s));
    }
    #[test]
    fn test_count_everyone() {
        let s = "abc";
        assert_eq!(3, count_everyone(&s));
        let s = "a\nb\nc";
        assert_eq!(0, count_everyone(&s));
        let s = "ab\nbc";
        assert_eq!(1, count_everyone(&s));
        let s = "a\na\na\na";
        assert_eq!(1, count_everyone(&s));
        let s = "b";
        assert_eq!(1, count_everyone(&s));
    }
}
