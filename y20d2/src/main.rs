extern crate helper;
use helper::{get_input_as_structs, print_part_1, print_part_2};
use std::str::FromStr;

const FILENAME: &str = env!("CARGO_PKG_NAME");

#[derive(Default)]
struct PasswordPolicy {
    min: u32,
    max: u32,
    char: String,
    password: String,
}

impl FromStr for PasswordPolicy {
    type Err = std::num::ParseIntError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let new_str = string.split_whitespace().collect::<Vec<_>>();
        let num: Vec<&str> = new_str[0].split('-').collect::<Vec<_>>();
        let min = num[0].parse::<u32>().unwrap();
        let max = num[1].parse::<u32>().unwrap();
        let password = new_str[2].to_owned();
        let char = new_str[1].chars().next().unwrap().to_string();
        Ok(Self {
            min,
            max,
            char,
            password,
        })
    }
}

impl PasswordPolicy {
    fn is_valid(&self) -> bool {
        let count = self.password.matches(&self.char).count() as u32;
        self.min <= count && count <= self.max
    }

    fn is_really_valid(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();
        let first = chars[(self.min - 1) as usize].to_string() == self.char;
        let second = chars[(self.max - 1) as usize].to_string() == self.char;

        first ^ second
    }
}

fn main() {
    let passwords: Vec<PasswordPolicy> = get_input_as_structs(FILENAME, PasswordPolicy::default());
    let count = passwords.iter().filter(|p| (*p).is_valid()).count();
    print_part_1(count);
    let count = passwords.iter().filter(|p| (*p).is_really_valid()).count();
    print_part_2(count);
}
