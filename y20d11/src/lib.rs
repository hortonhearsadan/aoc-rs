extern crate helper;
use helper::{get_input, print_part_1, print_part_2};

const FILENAME: &str = env!("CARGO_PKG_NAME");

const NEIGHBOURS: [(i32, i32); 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
];
const OCCUPIED: char = '#';
const EMPTY: char = 'L';

pub fn main() {
    let seats = get_input(FILENAME);
    let count = occupy_1(&seats);
    print_part_1(count);
    let count = occupy_2(&seats);
    print_part_2(count);
}

fn occupy_1(seats: &[String]) -> usize {
    let mut seating_plan: Vec<_> = seats
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let max_changes = seating_plan.len() * seating_plan[0].len();
    let mut changes: Vec<(usize, usize, char)> = Vec::with_capacity(max_changes);
    loop {
        for (i, row) in seating_plan.iter().enumerate() {
            for (j, seat) in row.iter().enumerate() {
                match *seat {
                    EMPTY => {
                        if adjacent_seats_1(i as i32, j as i32, &seating_plan) == 0 {
                            changes.push((i, j, OCCUPIED))
                        }
                    }
                    OCCUPIED => {
                        if adjacent_seats_1(i as i32, j as i32, &seating_plan) >= 4 {
                            changes.push((i, j, EMPTY))
                        }
                    }
                    _ => (),
                };
            }
        }
        if changes.is_empty() {
            break;
        } else {
            apply_changes(&changes, &mut seating_plan);
            changes.clear();
        }
    }

    seating_plan
        .into_iter()
        .flatten()
        .filter(|j| *j == OCCUPIED)
        .count()
}
fn occupy_2(seats: &[String]) -> usize {
    let mut seating_plan: Vec<_> = seats
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let max_changes = seating_plan.len() * seating_plan[0].len();
    let mut changes: Vec<(usize, usize, char)> = Vec::with_capacity(max_changes);
    loop {
        for (i, row) in seating_plan.iter().enumerate() {
            for (j, seat) in row.iter().enumerate() {
                match *seat {
                    EMPTY => {
                        if adjacent_seats_2(i as i32, j as i32, &seating_plan) == 0 {
                            changes.push((i, j, OCCUPIED))
                        }
                    }
                    OCCUPIED => {
                        if adjacent_seats_2(i as i32, j as i32, &seating_plan) >= 5 {
                            changes.push((i, j, EMPTY))
                        }
                    }
                    _ => (),
                };
            }
        }
        if changes.is_empty() {
            break;
        } else {
            apply_changes(&changes, &mut seating_plan);
            changes.clear();
        }
    }

    seating_plan
        .into_iter()
        .flatten()
        .filter(|j| *j == OCCUPIED)
        .count()
}

fn apply_changes(changes: &[(usize, usize, char)], seats: &mut Vec<Vec<char>>) {
    changes.iter().for_each(|(i, j, c)| seats[*i][*j] = *c);
}

fn adjacent_seats_1(row: i32, seat: i32, plan: &[Vec<char>]) -> i32 {
    NEIGHBOURS
        .iter()
        .filter(|(n, m)| is_occupied(row, seat, *n, *m, plan))
        .count() as i32
}

fn adjacent_seats_2(row: i32, seat: i32, plan: &[Vec<char>]) -> i32 {
    NEIGHBOURS
        .iter()
        .filter(|(r, s)| is_occupied_2(row, seat, *r, *s, plan))
        .count() as i32

}

fn is_occupied_2(row: i32, seat: i32, n: i32, m: i32, plan: &[Vec<char>]) -> bool {
    let mut new_row = row;
    let mut new_seat = seat;
    loop {
        new_row += n;
        new_seat += m;

        if let Some(state) = get_state(new_row as usize, new_seat as usize, plan) {
            match state {
                OCCUPIED => return true,
                EMPTY => return false,
                _ => continue,
            }
        } else {
            return false;
        }
    }
}

fn is_occupied(row: i32, seat: i32, n: i32, m: i32, plan: &[Vec<char>]) -> bool {
    let n_row = row + n;
    let n_col = seat + m;
    get_state(n_row as usize, n_col as usize, plan) == Some(OCCUPIED)
}

fn get_state(row: usize, seat: usize, plan: &[Vec<char>]) -> Option<char> {
    if let Some(r) = plan.get(row as usize) {
        if let Some(c) = r.get(seat as usize) {
            return Some(*c);
        }
    }
    None
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
