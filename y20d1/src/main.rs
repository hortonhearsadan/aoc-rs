extern crate helper;
use helper::{get_input_as_int, print_part_1, print_part_2};
const TARGET: i32 = 2020;
fn main() {
    let input: Vec<i32> = get_input_as_int("d01.txt");
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
