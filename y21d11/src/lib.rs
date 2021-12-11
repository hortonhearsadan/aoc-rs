#![allow(dead_code)]
#![allow(unused_imports)]
use helper::{get_input, get_raw_input, print_part_1, print_part_2};
use ndarray::{arr2, s, Array2, ArrayBase, ArrayView, ArrayViewMut, ArrayViewMut2};
use std::cmp::{max, min};

const FILENAME: &str = env!("CARGO_PKG_NAME");
const TEST: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

const TEST1: &str = "11111
19991
19191
19991
11111";

pub fn main() {
    let s = get_raw_input(FILENAME);
    // let s = TEST.split('\n').collect::<Vec<_>>();
    let octos = s.chars().filter_map(|d| d.to_digit(10)).collect();

    let mut squid_grid: Array2<u32> = Array2::<u32>::from_shape_vec((10,10), octos).unwrap();

    let (num_flashes, steps_ran_1) = run_steps(&mut squid_grid, 100);

    print_part_1(num_flashes);

    let (_, steps_ran_2) = run_steps(&mut squid_grid, 1000);
    print_part_2(steps_ran_1 + steps_ran_2)
}

fn run_steps(squid_grid: &mut Array2<u32>, steps: u32) -> (usize, u32) {
    let mut total_flashes = 0;
    let mut ran = steps;
    for s in 1..=steps {
        let mut flashes = Vec::new();
        let mut v: ArrayViewMut2<u32> = squid_grid.view_mut();
        v += 1;
        find_flashes(&mut v, &mut flashes);

        total_flashes += flashes.len();
        if flashes.len() == squid_grid.len() {
            ran = s;
            break;
        }
    }
    (total_flashes, ran)
}

fn find_flashes(squid_grid: &mut ArrayViewMut2<u32>, flashes: &mut Vec<(usize, usize)>) {
    for y in 0..squid_grid.nrows() {
        for x in 0..squid_grid.ncols() {
            let s = squid_grid[(y, x)];
            if s <= 9 {
                continue;
            }

            if flashes.contains(&(y, x)) {
                continue;
            }

            let mut slice = squid_grid.slice_mut(s![
                start(y as i32)..=stop(y as i32),
                start(x as i32)..=stop(x as i32)
            ]);
            slice += 1;
            flashes.push((y, x));
            find_flashes(squid_grid, flashes)
        }
    }

    for i in flashes.iter() {
        squid_grid[*i] = 0
    }
}

fn start(i: i32) -> usize {
    max(i - 1, 0) as usize
}

fn stop(i: i32) -> usize {
    min(i + 1, 9) as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
