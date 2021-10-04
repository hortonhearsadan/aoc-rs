use helper::{print_part_1, print_part_2};

const DATA: [i64; 6] = [0, 14, 6, 20, 1, 4];

pub fn main() {
    print_part_1(run(2020));
    print_part_2(run(30_000_000));
}

fn run(turns: usize) -> usize {
    let mut prevs = init(turns);
    let mut p = *DATA.last().unwrap() as usize;
    for t in DATA.len()..turns {
        let s = match prevs[p] {
            -1 => 0,
            n => t as i64 - n,
        };
        prevs[p] = t as i64;
        p = s as usize;
    }
    p
}

fn init(turns: usize) -> Vec<i64> {
    let mut prevs: Vec<i64> = vec![-1; turns];

    for (i, t) in DATA.iter().enumerate() {
        prevs[*t as usize] = (i + 1) as i64;
    }
    prevs
}