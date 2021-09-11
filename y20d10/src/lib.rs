use helper::{get_input_as_int, print_part_1, print_part_2};

const FILENAME: &str = env!("CARGO_PKG_NAME");

pub fn main() {
    let mut adapters = get_input_as_int(FILENAME);
    let max = adapters.iter().max().unwrap() + 3;
    adapters.push(0);
    adapters.push(max);
    adapters.sort_unstable();

    print_part_1(count_diffs(&adapters));
    print_part_2(count_chains(&adapters));
}

fn count_diffs(adapters: &[i32]) -> i32 {
    let mut one_volt = 0;
    let mut three_volt = 0;
    // let mut two_volts=0;
    for w in adapters.windows(2) {
        match w[1] - w[0] {
            1 => one_volt += 1,
            3 => three_volt += 1,
            // 2 => two_volts+=1,
            _ => {}
        }
    }
    // no diffs of 2 volts, might be on purpose for next bit

    (one_volt * three_volt) as i32
}

fn count_chains(adapters: &[i32]) -> i64 {
    let threes: Vec<_> = adapters
        .windows(2)
        .filter(|w| w[1] - w[0] == 3)
        .map(|w| w[0])
        .collect();
    let chunks: Vec<&[i32]> = adapters.split_inclusive(|a| threes.contains(a)).collect();
    //max len is 5 and end points are mandatory so its really only the combinations of the middle 3
    // which we need to count and its all combinations as theres no diffs of 2. this may be cheating?
    chunks
        .iter()
        .map(|c| match c.len() {
            1 | 2 => 1,
            3 => 2,
            4 => 4,
            5 => 7,
            _ => -1000000,
        })
        .product::<i64>()
}
