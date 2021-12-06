#![allow(dead_code)]
#![allow(unused_imports)]
use helper::{print_part_1, print_part_2, get_raw_input};
use std::collections::{HashMap, VecDeque};
use std::env::split_paths;

const FILENAME: &str = env!("CARGO_PKG_NAME");
const TEST: &str = "3,4,3,1,2";
const CYCLE: u64 = 7;
const NEW_FISH: u64 = 8;
const RESET: u64 = 6;

#[derive(Default, Debug, Clone, Copy)]
struct FishMap {
    zero: u64,
    one: u64,
    two: u64,
    three: u64,
    four: u64,
    five: u64,
    six: u64,
    seven: u64,
    eight: u64,
}

impl FishMap {
    fn new(fish : &Vec<u64>) -> Self{
        let mut fm = Self::default();
        for f in fish.iter() {
            match *f {
                0 => fm.zero +=1,
                1 => fm.one +=1,
                2 => fm.two +=1,
                3 => fm.three +=1,
                4 => fm.four +=1,
                5 => fm.five +=1,
                6 => fm.six +=1,
                7 => fm.seven +=1,
                8 => fm.eight +=1,
                _ => panic!("bad num")
            }
        }
        fm
    }

    fn age(&mut self) {
        let breed = self.zero;
        self.zero = self.one;
        self.one = self.two;
        self.two = self.three;
        self.three = self.four;
        self.four = self.five;
        self.five = self.six;
        self.six = self.seven;
        self.seven = self.eight;
        self.eight = breed;
        self.six +=breed
    }
    fn count(&self) -> u64{
        self.zero +self.one +self.two +self.three +self.four +self.five +self.six +self.seven +self.eight
    }

}

pub fn main() {
    // let fish = TEST.split(",").map(| s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
    let fish_str = get_raw_input(FILENAME);
    let fish = fish_str.split(",").map(| s| s.trim().parse::<u64>().unwrap()).collect::<Vec<_>>();

    let f = count_fish_fun(&fish.clone(), 80);
    print_part_1(f);

    let g = count_fish_fun(&fish.clone(), 256);
    print_part_2(g);

}

fn count_fish_fun(fish: &Vec<u64>, days: u64) -> u64 {
    let mut fish_map = FishMap::new(fish);
    for d in 1..=days {
        fish_map.age();
    }
    fish_map.count()

}

fn count_fish_simple(fish: &Vec<u64>, days: u64) -> u64 {
    let mut c: VecDeque<u64> = VecDeque::from(vec![0;9]);
    for f in fish.iter() {
        c[*f as usize] +=1
    }

    for d in 1..=days {
        let b = c.pop_front().unwrap();
        c.push_back(b);
        c[6] += b
    }
    c.iter().sum::<u64>()
}

fn count_fish_slow(fish: &Vec<u64>, days: u64) -> usize {
    let mut c = fish.clone();
    let mut new = Vec::new();

    for d in 1..=days {
        for f in c.iter_mut() {
            if *f == 0 {
                new.push(NEW_FISH);
                *f = RESET;
            } else {
                *f -= 1
            }
        }
        c.extend(new.iter());
        new.clear();
        println!("{} {} {:?}",d, c.len(), c)
    }
    c.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
