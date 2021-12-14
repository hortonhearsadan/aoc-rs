#![allow(dead_code)]
#![allow(unused_imports)]
use helper::{debug, get_raw_input, print_part_1, print_part_2};
use itertools::Itertools;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

const FILENAME: &str = env!("CARGO_PKG_NAME");
const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const BUFFER: u8 = u8::MAX;
const TEST: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

fn find_idx(c: char) -> u8 {
    ALPHABET.find(c).unwrap() as u8
}

pub fn main() {
    // let a = TEST;
    let a = get_raw_input(FILENAME);
    let s: Vec<_> = a.split_terminator('\n').collect();
    let polymer_template: Vec<u8> = [
        vec![BUFFER],
        s[0].chars().map(find_idx).collect::<Vec<u8>>(),
        vec![BUFFER],
    ]
    .to_vec()
    .concat();
    let rules: HashMap<(u8, u8), u8> = s[1..]
        .iter()
        .filter_map(|l| l.split(" -> ").collect_tuple::<(&str, &str)>())
        .map(|(lhs, rhs)| {
            let mut l = lhs.chars();
            (
                (find_idx(l.next().unwrap()), find_idx(l.next().unwrap())),
                find_idx(rhs.chars().next().unwrap()),
            )
        })
        .collect();
    let mut rule_map: HashMap<_, _> = HashMap::new();
    for (k, v) in rules.iter() {
        rule_map.insert(*k, *v);
    }

    let mut poly_map: HashMap<(u8, u8), usize> =
        polymer_template.into_iter().tuple_windows().counts();

    for _ in 1..=10 {
        polymerise(&mut poly_map, &rule_map)
    }

    print_part_1(count_diff(&poly_map));

    for _ in 11..=40 {
        polymerise(&mut poly_map, &rule_map)
    }

    print_part_2(count_diff(&poly_map));
}

fn count_diff(poly_map: &HashMap<(u8, u8), usize>) -> usize {
    let mut counts: HashMap<&u8, usize> = HashMap::new();
    for ((k1, k2), v) in poly_map.iter() {
        if *k1 != BUFFER {
            *counts.entry(k1).or_insert(0) += v;
        }
        if *k2 != BUFFER {
            *counts.entry(k2).or_insert(0) += v;
        }
    }

    let max_1 = &counts.values().max().unwrap();
    let min_1 = &counts.values().min().unwrap();
    (*max_1 - *min_1) / 2
}

fn polymerise(polymap: &mut HashMap<(u8, u8), usize>, rule_map: &HashMap<(u8, u8), u8>) {

    for ((u, v), c) in polymap.clone() {
        if let Some(&r) = rule_map.get(&(u, v)) {
            *polymap.entry((u, r)).or_insert(0) += c;
            *polymap.entry((r, v)).or_insert(0) += c;
            *polymap.entry((u, v)).or_insert(0) -= c;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
