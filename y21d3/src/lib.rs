#![allow(dead_code)]
#![allow(unused_imports)]
use helper::{get_input, print_part_1, print_part_2};
use std::cmp::Ordering;

const FILENAME: &str = env!("CARGO_PKG_NAME");
const TEST: [&str; 12] = [
    "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
    "00010", "01010",
];

pub fn main() {
    let strings = get_input(FILENAME);
    // let strings = TEST.iter().map(|s| s.to_string()).collect::<Vec<String>>();
    let codes: Vec<Vec<u32>> = strings.iter().map(|s| str_to_ints(s)).collect();

    print_part_1(part1(&codes));

    print_part_2(part2(&codes));
}

fn part1(codes: &[Vec<u32>]) -> u32 {
    let g = get_gamma_bin(&codes);
    let e = get_epsilon_bin(&codes);

    get_dec_from_bin(&g) * get_dec_from_bin(&e)
}

fn part2(codes: &[Vec<u32>]) -> u32 {
    let o = get_oxygen_rating(codes);
    let c = get_co2_rating(codes);

    get_dec_from_bin(&o) * get_dec_from_bin(&c)
}

fn str_to_ints(s: &str) -> Vec<u32> {
    s.chars()
        .map(|c| c.to_string().parse::<u32>().unwrap())
        .collect()
}

fn get_gamma_bin(codes: &[Vec<u32>]) -> Vec<u32> {
    (0..codes[0].len())
        .map(|i| get_most_common_bit(codes, i))
        .collect::<Vec<u32>>()
}

fn get_epsilon_bin(codes: &[Vec<u32>]) -> Vec<u32> {
    (0..codes[0].len())
        .map(|i| get_least_common_bit(codes, i))
        .collect::<Vec<u32>>()
}

fn get_oxygen_rating(codes: &[Vec<u32>]) -> Vec<u32> {
    let mut new_codes = codes.to_owned();

    for i in 0..codes[0].len() {
        let b = get_most_common_bit(&new_codes, i);
        new_codes = new_codes.into_iter().filter(|c| c[i] == b).collect();

        if new_codes.len() == 1 {
            break;
        }
    }
    new_codes[0].clone()
}
fn get_co2_rating(codes: &[Vec<u32>]) -> Vec<u32> {
    let mut new_codes = codes.to_owned();

    for i in 0..codes[0].len() {
        let b = get_least_common_bit(&new_codes, i);

        new_codes = new_codes.into_iter().filter(|c| c[i] == b).collect();

        if new_codes.len() == 1 {
            break;
        }
    }
    new_codes[0].clone()
}

fn get_most_common_bit(codes: &[Vec<u32>], i: usize) -> u32 {
    let l2 = ((codes.len() + 1) / 2) as u32;
    let s = codes.iter().map(|v| v[i]).sum::<u32>();

    match s.cmp(&l2) {
        Ordering::Less => 0,
        Ordering::Equal => 1,
        Ordering::Greater => 1,
    }
}

fn get_least_common_bit(codes: &[Vec<u32>], i: usize) -> u32 {
    let l2 = ((codes.len() + 1) / 2) as u32;
    let s = codes.iter().map(|v| v[i]).sum::<u32>();

    match s.cmp(&l2) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => 0,
    }
}

fn get_dec_from_bin(code: &[u32]) -> u32 {
    let str: String = code.iter().map(|i| i.to_string()).collect::<String>();
    u32::from_str_radix(&*str, 2).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
