use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_input(filename: &str) -> Vec<String> {
    let reader = get_reader(filename);
    reader.lines().map(|l| l.unwrap()).collect()
}

pub fn get_reader(filename: &str) -> BufReader<File> {
    let file = File::open(format!("input/{}", filename)).expect("file not found");
    BufReader::new(file)
}

pub fn get_input_as_int(filename: &str) -> Vec<i32> {
    get_reader(filename)
        .lines()
        .map(|i| i.unwrap().parse::<i32>().unwrap())
        .collect()
}

pub fn print_part_1<T: std::fmt::Display>(result: T) {
    print!("\nPart 1: {}\n", result)
}
pub fn print_part_2<T: std::fmt::Display>(result: T) {
    print!("\nPart 2: {}\n", result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
