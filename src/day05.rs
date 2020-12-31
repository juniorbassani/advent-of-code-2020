use std::cmp;
use std::io::BufRead;

const INPUT_PATH: &str = "input/day05";
const MAX_ROW: u32 = 127;
const MAX_COLUMN: u32 = 7;
const NUM_OF_COLUMNS: u32 = MAX_COLUMN + 1;

fn seat_id(seat: &str) -> u32 {
    let mut row = 0..MAX_ROW;
    for char in seat[..7].chars() {
        if char == 'F' {
            row = row.start..(row.start + row.end) / 2;
        } else {
            row = (row.start + row.end) / 2 + 1..row.end;
        }
    }

    let mut column = 0..MAX_COLUMN;
    for char in seat[7..].chars() {
        if char == 'L' {
            column = column.start..(column.start + column.end) / 2;
        } else {
            column = (column.start + column.end) / 2 + 1..column.end;
        }
    }

    row.start * NUM_OF_COLUMNS + column.start
}

fn parse_line(line: &str) -> &str {
    line.split_whitespace().next().expect("Invalid seat")
}

pub fn part1() -> u32 {
    let file = crate::get_buffered_input(INPUT_PATH);
    let mut max_seat_id = 0;

    for seat in file.lines() {
        max_seat_id = cmp::max(max_seat_id, seat_id(parse_line(&seat.unwrap())));
    }

    max_seat_id
}

pub fn part2() -> u32 {
    let file = crate::get_buffered_input(INPUT_PATH);
    let mut all_seats = file
        .lines()
        .map(|seat| seat_id(parse_line(&seat.unwrap())))
        .collect::<Vec<u32>>();

    all_seats.sort_unstable();

    let mut prev = all_seats[0];
    for seat in all_seats.into_iter().skip(1) {
        if seat != prev + 1 {
            return seat - 1;
        }
        prev = seat;
    }

    panic!("All seats occupied");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_id() {
        let a = "FBFBBFFRLR";
        let b = "BFFFBBFRRR";
        let c = "FFFBBBFRRR";
        let d = "BBFFBBFRLL";

        assert_eq!(seat_id(a), 357);
        assert_eq!(seat_id(b), 567);
        assert_eq!(seat_id(c), 119);
        assert_eq!(seat_id(d), 820);
    }

    #[test]
    fn compute_part1() {
        let res = part1();
        assert_eq!(res, 933);
    }

    #[test]
    fn compute_part2() {
        let res = part2();
        assert_eq!(res, 711);
    }
}
