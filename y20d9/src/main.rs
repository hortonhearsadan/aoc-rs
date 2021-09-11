use helper::{get_input_as_int64, print_part_1, print_part_2};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::iter::FromIterator;

const FILENAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    let nums = get_input_as_int64(FILENAME);
    let a = go_1(&nums, 25);
    let b = go_2(&nums, a);
    print_part_1(a);
    print_part_2(b);
}

fn assess(n: i64, preamble: &VecDeque<i64>) -> bool {
    for p in preamble {
        let q = n - p;
        if preamble.contains(&q) && p != &q {
            return true;
        };
    }
    false
}

fn go_1(nums: &[i64], preamble_num: usize) -> i64 {
    let (pre, new_nums) = nums.split_at(preamble_num);
    let mut numbers: VecDeque<i64> = VecDeque::from(new_nums.to_owned());
    let mut preamble: VecDeque<i64> = VecDeque::from(pre.to_owned());
    let invalid_num: i64;
    loop {
        let n = numbers.pop_front().unwrap();
        let is_valid = assess(n, &preamble);
        if is_valid {
            preamble.pop_front();
            preamble.push_back(n);
        } else {
            invalid_num = n;
            break;
        }
    }
    invalid_num
}

fn go_2(nums: &[i64], invalid_num: i64) -> i64 {
    let mut contiguous_array = VecDeque::new();
    let mut deque_nums = VecDeque::from_iter(nums);

    let mut total = 0;
    let weakness: i64;
    loop {
        match total.cmp(&invalid_num) {
            Ordering::Greater => total -= contiguous_array.pop_back().unwrap(),
            Ordering::Less => {
                let n = deque_nums.pop_front().unwrap();
                total += n;
                contiguous_array.push_front(n);
            }
            Ordering::Equal => {
                weakness = *contiguous_array.iter().min().unwrap()
                    + *contiguous_array.iter().max().unwrap();
                break;
            }
        }
    }
    weakness
}

#[cfg(test)]
mod tests {
    use crate::go;

    #[test]
    fn test_go() {
        let nums = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let (a, b) = go(nums, 5);
        assert_eq!(a, 127);
        assert_eq!(b, 62)
    }
}
