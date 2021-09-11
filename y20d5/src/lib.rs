extern crate helper;
use helper::{get_input, print_part_1, print_part_2};

const FILENAME: &str = env!("CARGO_PKG_NAME");

pub fn main() {
    let seats = get_input(FILENAME);
    let seat_ids = seats.iter().map(|s| get_id(&*s));
    print_part_1(seat_ids.max().unwrap());

    let seat_ids = seats.iter().map(|s| get_id(&*s)).collect::<Vec<_>>();
    let max_id = get_id("BBBBBBBRRR");
    let my_id = (0..max_id)
        .into_iter()
        .find(|id| {
            !seat_ids.contains(&id) && seat_ids.contains(&(id - 1)) && seat_ids.contains(&(id + 1))
        })
        .unwrap();
    print_part_2(my_id)
}

fn get_id(seat_string: &str) -> u32 {
    let (row, column) = seat_string.split_at(7);
    let row_num = get_row(row);
    let col_num = get_column(column);
    row_num * 8 + col_num
}

fn get_row(row: &str) -> u32 {
    let row_bin = row.replace('F', "0").replace('B', "1");
    u32::from_str_radix(&*row_bin, 2).unwrap()
}

fn get_column(row: &str) -> u32 {
    let col_bin = row.replace('L', "0").replace('R', "1");
    u32::from_str_radix(&*col_bin, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{get_column, get_id, get_row};

    #[test]
    fn test_row() {
        let row = "FBFBBFF";
        assert_eq!(44, get_row(row))
    }
    #[test]
    fn test_column() {
        let row = "RLR";
        assert_eq!(5, get_column(row))
    }
    #[test]
    fn test_id() {
        assert_eq!(567, get_id("BFFFBBFRRR"));
        assert_eq!(119, get_id("FFFBBBFRRR"));
        assert_eq!(820, get_id("BBFFBBFRLL"));
    }
}
