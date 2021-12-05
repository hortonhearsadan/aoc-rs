#![allow(dead_code)]
#![allow(unused_imports)]

use helper::{print_part_1, print_part_2, FromInput};
use ndarray::{s, Array2, ArrayViewMut};
use std::cmp::{max, min};
use std::collections::hash_map::Entry::Vacant;
use std::path::Prefix::Verbatim;
use std::str::FromStr;
use std::time::Instant;

const FILENAME: &str = env!("CARGO_PKG_NAME");
const TEST: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

#[derive(Debug)]
struct Vent {
    x_0: usize,
    y_0: usize,
    x_1: usize,
    y_1: usize,
}

impl FromStr for Vent {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x: Vec<_> = s.split(" -> ").collect();
        let zeros: Vec<_> = x[0]
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        let ones: Vec<_> = x[1]
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        Ok(Vent {
            x_0: zeros[0],
            y_0: zeros[1],
            x_1: ones[0],
            y_1: ones[1],
        })
    }
}

impl Vent {
    fn is_vertical_or_horizontal(&self) -> bool {
        self.is_vertical() || self.is_horizontal()
    }

    fn get_step(&self) -> i32 {
        match self.is_inverted() {
            true => -1,
            false => 1,
        }
    }

    fn is_inverted(&self) -> bool {
        (self.x_1 < self.x_0) ^ (self.y_1 < self.y_0)
    }

    fn is_vertical(&self) -> bool {
        self.y_0 == self.y_1
    }

    fn is_horizontal(&self) -> bool {
        self.x_0 == self.x_1
    }

    fn is_diagonal(&self) -> bool {
        !self.is_vertical_or_horizontal()
    }

    fn max_x(&self) -> usize {
        max(self.x_0, self.x_1)
    }

    fn max_y(&self) -> usize {
        max(self.y_0, self.y_1)
    }

    fn min_x(&self) -> usize {
        min(self.x_0, self.x_1)
    }

    fn min_y(&self) -> usize {
        min(self.y_0, self.y_1)
    }
}

pub fn main() {
    let vents = Vent::from_input(FILENAME);

    let d_x: usize = vents.iter().map(|v| v.max_x()).max().unwrap() + 1;
    let d_y: usize = vents.iter().map(|v| v.max_y()).max().unwrap() + 1;

    let mut field: Array2<usize> = Array2::<usize>::zeros((d_y, d_x));

    for v in vents.iter() {
        if v.is_vertical_or_horizontal() {
            add_vent_to_matrix(&mut field, v);
        }
    }

    let part_1 = count_overlaps(&field);

    for v in vents.iter() {
        if v.is_diagonal() {
            add_vent_to_matrix(&mut field, v);
        }
    }

    let part_2 = count_overlaps(&field);

    print_part_1(part_1);
    print_part_2(part_2);
}

fn add_vent_to_matrix(m: &mut Array2<usize>, vent: &Vent) {
    let step = vent.get_step();

    let mut slice = m.slice_mut(s![vent.min_y()..=vent.max_y(), vent.min_x()..=vent.max_x();step]);

    if vent.is_vertical_or_horizontal() {
        slice += 1
    } else {
        let mut diag = slice.diag_mut();
        diag += 1
    }
}

fn count_overlaps(m: &Array2<usize>) -> usize {
    m.iter().filter(|&&c| c > 1).count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
