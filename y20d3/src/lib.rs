extern crate helper;
use helper::{get_input, print_part_1, print_part_2};

const FILENAME: &str = env!("CARGO_PKG_NAME");
const TREE: &str = "#";

pub fn main() {
    let grid = get_input(FILENAME);
    let length = grid[0].len();
    let height = grid.len();
    let right = 3;
    let down = 1;
    let count = count_trees(&grid, &right, &down, height, length);

    print_part_1(count);

    let slopes: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let count: usize = slopes
        .iter()
        .map(|(right, down)| count_trees(&grid, right, down, height, length))
        .product();

    print_part_2(count)
}

fn count_trees(
    grid: &[String],
    right: &usize,
    down: &usize,
    height: usize,
    length: usize,
) -> usize {
    (0..height / down)
        .filter(|i| &grid[i * down][(i * right) % length..((i * right) % length) + 1] == TREE)
        .count()
}
