use helper::{print_part_1, print_part_2};

const FILENAME: &str = env!("CARGO_PKG_NAME");

pub fn main() {
    let inputs = helper::get_input(FILENAME);
    let timestamp: i64 = inputs[0].parse().unwrap();

    let mut buses = Vec::new();
    let mut offsets = Vec::new();
    for (j, i) in inputs[1].split(',').enumerate() {
        if let Ok(n) = i.parse::<i64>() {
            buses.push(n);
            offsets.push(j as i64);
        }
    }

    let bus = find_earliest_bus(timestamp, &buses);

    let time = run_2(&offsets, &buses);

    print_part_1(bus * get_wait(&bus, timestamp));
    print_part_2(time);
}

fn find_earliest_bus(timestamp: i64, buses: &[i64]) -> i64 {
    *buses
        .iter()
        .min_by_key(|b| get_wait(*b, timestamp))
        .unwrap()
}

fn get_wait(b: &i64, timestamp: i64) -> i64 {
    b - (timestamp % b)
}

fn run_2(minutes: &[i64], buses: &[i64]) -> i64 {
    let n: i64 = buses.iter().product();
    minutes
        .iter()
        .zip(buses.iter())
        .map(|(&x, &y)| ((y - x) * n / y) * (mod_pow(n / y, y - 2, y)))
        .sum::<i64>()
        % n
}

fn mod_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus
    }
    result
}

// #[cfg(test)]
// mod tests {
//     use crate::find_earliest_bus;
//
//     #[test]
//     fn it_works_13() {
//         let t = 939;
//         let v = vec![7, 13, 59, 31, 19];
//         assert_eq!(find_earliest_bus(t, &v), (59, 5));
//     }
// }
