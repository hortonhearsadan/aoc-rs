#![allow(dead_code)]
#![allow(unused_imports)]
use helper::{get_raw_input, print_part_1, print_part_2};

const FILENAME: &str = env!("CARGO_PKG_NAME");
const TEST: &str = "16,1,2,0,4,2,7,1,2,14";

pub fn main() {
    let s = get_raw_input(FILENAME);

    let mut crabs: Vec<_> = s
        .split(',')
        .map(|c| c.trim().parse::<i32>().unwrap())
        .collect();

    crabs.sort_unstable();

    let (mid, median, skew) = analyse_crabs(&crabs);

    let c1 = cost_1(&crabs, median);

    let (start, stop) = match skew {
        1 => (mid, crabs.len() - 1),
        _ => (0, mid),
    };

    let c2 = crabs[start..=stop]
        .iter()
        .map(|&x| cost_2(&crabs, x))
        .min()
        .unwrap();

    print_part_1(c1);
    print_part_2(c2);
}

fn cost_1(crabs: &[i32], pos: i32) -> i32 {
    crabs.iter().map(|&c| (c - pos).abs()).sum()
}

fn cost_2(crabs: &[i32], pos: i32) -> i32 {
    crabs.iter().map(|&c| triangle_sum((c - pos).abs())).sum()
}

fn analyse_crabs(crabs: &[i32]) -> (usize, i32, i32) {
    let m = crabs.len() / 2;
    let median = crabs[m];
    let mean: i32 = crabs.iter().sum::<i32>() / crabs.len() as i32;
    let skew: i32 = if mean > median { 1 } else { -1 };
    (m, median, skew)
}

fn triangle_sum(a: i32) -> i32 {
    (a * (a + 1)) / 2
}

#[cfg(test)]
mod tests {
    use crate::triangle_sum;

    #[test]
    fn it_works() {
        assert_eq!(triangle_sum(3), 6);
    }
}
