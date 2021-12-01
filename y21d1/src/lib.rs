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
    print_part_1(count_2);
}

fn count_increases(depths: &[i32], step: usize) -> usize {
    depths.windows(step + 1).filter(|w| w[0] < w[step]).count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
