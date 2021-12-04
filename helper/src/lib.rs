use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

pub trait FromInput<T> {
    fn from_input(filename: &str) -> Vec<T>;
    fn from_multiline_input(filename: &str) -> Vec<T>;
    fn from_multiline(input: &str) -> Vec<T>;
}

impl<T: FromStr> FromInput<T> for T {
    fn from_input(filename: &str) -> Vec<T> {
        let reader = get_reader(filename);
        reader
            .lines()
            .map(|l| T::from_str(l.unwrap().as_str()).ok().unwrap())
            .collect()
    }

    fn from_multiline_input(filename: &str) -> Vec<T> {
        let input = get_raw_input(filename);
        T::from_multiline(&input)
    }

    fn from_multiline(input: &str) -> Vec<T> {
        let blobs = input.split("\n\n").collect::<Vec<_>>();
        blobs
            .iter()
            .map(|b| T::from_str(*b).ok().unwrap())
            .collect::<Vec<T>>()
    }
}

pub fn get_input(filename: &str) -> Vec<String> {
    let reader = get_reader(filename);
    reader.lines().map(|l| l.unwrap()).collect()
}

pub fn get_raw_input(filename: &str) -> String {
    let mut file = File::open(format!("input/{}.txt", filename)).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

pub fn get_reader(filename: &str) -> BufReader<File> {
    let file = File::open(format!("input/{}.txt", filename)).expect("file not found");
    BufReader::new(file)
}

pub fn get_input_as_int(filename: &str) -> Vec<i32> {
    get_reader(filename)
        .lines()
        .map(|i| i.unwrap().parse::<i32>().unwrap())
        .collect()
}

pub fn get_input_as_int64(filename: &str) -> Vec<i64> {
    get_reader(filename)
        .lines()
        .map(|i| i.unwrap().parse::<i64>().unwrap())
        .collect()
}

pub fn print_part_1<T: std::fmt::Display>(result: T) {
    println!("Part 1: {}", result)
}
pub fn print_part_2<T: std::fmt::Display>(result: T) {
    println!("Part 2: {}", result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
