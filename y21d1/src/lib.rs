#![allow(dead_code)]
#![allow(unused_imports)]

use helper::{get_input_as_int, print_part_1, print_part_2};

const FILENAME: &str = env!("CARGO_PKG_NAME");
const TEST: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

pub fn main() {
    let inputs = get_input_as_int(FILENAME);

    let count_1 = count_increases(&inputs, 1);
    let count_2 = count_increases(&inputs, 3);

    print_part_1(count_1);
    print_part_2(count_2);
}

fn count_increases(depths: &[i32], step: usize) -> usize {
    depths.windows(step + 1).filter(|w| w[0] < w[step]).count()
}

fn count_increases_2(depths: &[i32], step: usize) -> usize {
    depths
        .iter()
        .zip(depths[step..].iter())
        .filter(|(x, y)| x < y)
        .count()
}

fn count_increases_3(depths: &[i32]) -> (usize, usize) {
    let mut count_1 = 0;
    let mut count_2 = 0;
    let l = depths.len();
    for (i, d) in depths.iter().enumerate() {
        let j = i + 1;
        let k = i + 3;
        if j < l && d < &depths[j] {
            count_1 += 1
        }
        if k < l && d < &depths[k] {
            count_2 += 1
        }
    }
    (count_1, count_2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
