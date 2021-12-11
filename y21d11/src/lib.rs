#![allow(dead_code)]
#![allow(unused_imports)]
use helper::{print_part_1, print_part_2, get_raw_input, get_input};
use ndarray::{Array2, ArrayBase, arr2, s, ArrayView, ArrayViewMut, ArrayViewMut2};
use std::cmp::{max, min};

const FILENAME: &str = env!("CARGO_PKG_NAME");
const TEST :&str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

const TEST1 :&str = "11111
19991
19191
19991
11111";

pub fn main() {
    let s = get_input(FILENAME);
    // let s = TEST.split('\n').collect::<Vec<_>>();
    let t = s.iter().map(|c| c.chars().map(|d| d.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>()[..].concat();

    // let u = t[..].concat().to_owned();
    let mut grid: Array2<u32> = Array2::<u32>::from_shape_vec((s.len(),s[0].len()), t).unwrap();
    // for (i,v) in t.iter().enumerate() {
    //     for (j,u) in v.iter().enumerate() {
    //         grid[[i,j]] = *u
    //     }
    // }

    println!("{:?}", grid.len());
    let (num_flashes, steps_ran_1) = run_steps(&mut grid,100);
    println!("{:?}", grid);

    print_part_1(num_flashes);

    let (_,steps_ran_2) = run_steps(&mut grid,1000);
    print_part_2(steps_ran_1 + steps_ran_2)

}

fn run_steps(grid: &mut Array2<u32>, steps: u32) -> (usize,u32) {
    let mut total_flashes = 0;
    let mut ran =steps;
    for s in 1..=steps {
        let mut flashes = Vec::new();
        let mut v: ArrayViewMut2<u32> = grid.view_mut();
        v +=1;
        find_flashes(&mut v,&mut flashes);

        total_flashes += flashes.len();
        if flashes.len() == grid.len() {
            println!("{}", s);
            ran = s;
            break
        }

    }
    (total_flashes, ran)
}

fn find_flashes(grid: &mut ArrayViewMut2<u32>, flashes: &mut Vec<(usize, usize)>) {
    for y in 0..grid.nrows() {
        for x in 0..grid.ncols() {
            let s = grid[(y,x)];
            if s <= 9 {
                continue
             }

            if flashes.contains(&(y,x)){
                continue
            }

            let mut slice = grid.slice_mut(s![start(y as i32)..=stop(y as i32),start(x as i32)..=stop(x as i32)]);
            slice +=1;
            flashes.push((y,x));
            find_flashes(grid,flashes)
    }

        }

    for i in flashes.iter() {
        grid[*i] = 0
    }
}

fn start(i:i32) -> usize{
    max(i-1,0) as usize
}

fn stop(i:i32) -> usize{
    min(i+1,9) as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
