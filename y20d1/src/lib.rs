extern crate helper;
use helper::{get_input_as_int, print_part_1, print_part_2};

const FILENAME: &str = env!("CARGO_PKG_NAME");
const TARGET: i32 = 2020;

pub fn main() {
    let input: Vec<i32> = get_input_as_int(FILENAME);
    for i in &input {
        let j = TARGET - i;
        if input.contains(&j) {
            print_part_1(i * j);
            break;
        }
    }

    'outer: for i in &input {
        for j in &input {
            let k = TARGET - i - j;
            if input.contains(&k) {
                print_part_2(i * j * k);
                break 'outer;
            }
        }
    }
}
