#![allow(dead_code)]
#![allow(unused_imports)]
use helper::{get_raw_input, print_part_1, print_part_2};

const FILENAME: &str = env!("CARGO_PKG_NAME");
const POINTS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

const TEST: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

pub fn main() {
    // let s = TEST;
    let s = get_raw_input(FILENAME);
    let grid = s
        .split('\n')
        .filter(|r| !r.is_empty())
        .map(|r| {
            r.chars()
                .map(|c| c.to_string().trim().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let low_coords = part_1(&grid);

    let basins = part_2(&grid, &low_coords);

    print_part_1(
        low_coords
            .iter()
            .map(|(i, j)| grid[*i][*j] + 1)
            .sum::<i32>(),
    );

    print_part_2(basins[..3].iter().product::<usize>());
}

fn part_1(grid: &[Vec<i32>]) -> Vec<(usize, usize)> {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut low_coords = Vec::new();

    for i in 0..num_rows {
        for j in 0..num_cols {
            let point = grid[i][j];
            let neighbours = get_neighbours(grid, i, j);

            if neighbours.iter().any(|n| *n <= point) {
                continue;
            }
            low_coords.push((i, j))
        }
    }
    low_coords
}

fn part_2(grid: &[Vec<i32>], low_coords: &[(usize, usize)]) -> Vec<usize> {
    let mut basins = Vec::with_capacity(low_coords.len());

    for (i, j) in low_coords.iter() {
        let point = grid[*i][*j];
        let mut neighbours = vec![(*i, *j)];
        get_recursive_higher_neighbours(grid, *i, *j, point, &mut neighbours);
        basins.push(neighbours.len())
    }

    basins.sort_unstable();
    basins.reverse();
    basins
}

fn get_neighbours(grid: &[Vec<i32>], row: usize, col: usize) -> Vec<i32> {
    let mut neighbours = Vec::with_capacity(4);

    for (x, y) in POINTS {
        if let Some(r) = grid.get((row as i32 + y) as usize) {
            if let Some(c) = r.get((col as i32 + x) as usize) {
                neighbours.push(*c)
            }
        }
    }
    neighbours
}

fn get_recursive_higher_neighbours(
    grid: &[Vec<i32>],
    row: usize,
    col: usize,
    point: i32,
    existing_points: &mut Vec<(usize, usize)>,
) {
    for (x, y) in POINTS {
        let r_c = (row as i32 + y) as usize;
        if let Some(r) = grid.get(r_c) {
            let c_c = (col as i32 + x) as usize;
            if let Some(c) = r.get(c_c) {
                if c > &point && c != &9 && !existing_points.contains(&(r_c, c_c)) {
                    existing_points.push((r_c, c_c));
                    get_recursive_higher_neighbours(grid, r_c, c_c, *c, existing_points);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
