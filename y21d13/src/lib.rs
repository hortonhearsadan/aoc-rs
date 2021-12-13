#![allow(dead_code)]
#![allow(unused_imports)]
use helper::{debug, get_raw_input, print_part_1, print_part_2};
use ndarray::{s, Array2};
use std::ops::AddAssign;

const FILENAME: &str = env!("CARGO_PKG_NAME");
const TEST: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

pub fn main() {
    let s = get_raw_input(FILENAME);
    // let s = TEST;
    let (points_str, instrs_str) = s.trim().split_once("\n\n").unwrap();
    let points: Vec<(_, _)> = points_str
        .split('\n')
        .map(|p| {
            p.split_once(',')
                .map(|(q, r)| {
                    (
                        q.trim().parse::<usize>().unwrap(),
                        r.trim().parse::<usize>().unwrap(),
                    )
                })
                .unwrap()
        })
        .collect();
    let instrs1: Vec<(_, _)> = instrs_str
        .split('\n')
        .map(|i| {
            i.split_whitespace().collect::<Vec<_>>()[2]
                .split_once('=')
                .unwrap()
        })
        .collect();
    let instrs: Vec<(_, _)> = instrs1
        .iter()
        .map(|(v, w)| (v.to_string(), w.trim().parse::<usize>().unwrap()))
        .collect();

    let x_max = instrs
        .iter()
        .filter(|p| p.0 == "x")
        .map(|p| p.1)
        .max()
        .unwrap()
        * 2;
    // let y_max = points.iter().map(|p| p.1).max().unwrap();
    let y_max = instrs
        .iter()
        .filter(|p| p.0 == "y")
        .map(|p| p.1)
        .max()
        .unwrap()
        * 2;
    let mut grid: Array2<i32> = Array2::zeros((y_max + 1, x_max + 1));

    for (x, y) in points.iter() {
        grid[[*y, *x]] = 1
    }

    for i in instrs[0..1].iter() {
        fold(&mut grid, i)
    }
    print_part_1(count(&grid));

    for i in instrs[1..].iter() {
        fold(&mut grid, i)
    }
    println!("Part 2:");
    println!("{:?}", read_grid(&grid))
}

fn read_grid(grid: &Array2<i32>) -> Array2<char> {
    grid.map(|&x| read(x))
}
fn read(i: i32) -> char {
    if i > 0 {
        '#'
    } else {
        ' '
    }
}
fn count(grid: &Array2<i32>) -> usize {
    grid.iter().filter(|&&p| p > 0).count()
}
fn fold(grid: &mut Array2<i32>, instruction: &(String, usize)) {
    let slice_range = match instruction.0.as_str() {
        "x" => s![..,instruction.1+1..;-1],
        "y" => s![instruction.1+1..;-1,..],
        _ => panic!("bad instr"),
    };
    let collapse_range = match instruction.0.as_str() {
        "x" => s![.., ..instruction.1],
        "y" => s![..instruction.1, ..],
        _ => panic!("bad instr"),
    };
    let g: Array2<i32> = grid.clone();
    let slice = g.slice(slice_range);

    grid.slice_collapse(collapse_range);
    grid.add_assign(&slice);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
