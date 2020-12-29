use std::fs::File;
use std::io::{ErrorKind, Read, Seek, SeekFrom};

const INPUT_PATH: &str = "input/day03";

fn next_slope(file: &mut File, right: u64, down: u64) {
    let mut buf = [0; 1];

    // If we encounter a newline, we have to reposition the cursor to the beginning of the line
    for _ in 0..right {
        file.read_exact(&mut buf).unwrap();
        if buf[0] == b'\n' {
            // For some reason, seeking to -33 an removing the read causes a panic
            file.seek(SeekFrom::Current(-32)).unwrap();
            file.read_exact(&mut buf).unwrap();
        }
    }

    for i in 0..down {
        let seek = if i > 0 { 32 } else { 31 };
        file.seek(SeekFrom::Current(seek)).unwrap();
    }
}

fn trees_in_slopes(right: u64, down: u64) -> u64 {
    let mut file = File::open(INPUT_PATH).unwrap();
    file.seek(SeekFrom::Start(1)).unwrap();
    let mut buf = [0; 1];
    let mut num_of_trees = 0;

    loop {
        next_slope(&mut file, right, down);

        match file.read_exact(&mut buf) {
            Ok(()) => {
                if buf[0] == b'#' {
                    num_of_trees += 1;
                }
            }
            Err(err) => {
                if let ErrorKind::UnexpectedEof = err.kind() {
                    return num_of_trees;
                } else {
                    panic!("Unknown error");
                }
            }
        }
    }
}

pub fn part1() -> u64 {
    trees_in_slopes(3, 1)
}

pub fn part2() -> u64 {
    let a = trees_in_slopes(1, 1);
    let b = trees_in_slopes(3, 1);
    let c = trees_in_slopes(5, 1);
    let d = trees_in_slopes(7, 1);
    let e = trees_in_slopes(1, 2);

    a * b * c * d * e
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_slopes(right: u64, down: u64) {
        let mut file = File::open(INPUT_PATH).unwrap();
        file.seek(SeekFrom::Start(1)).unwrap();

        for i in 0..20 {
            next_slope(&mut file, right, down);
            let mut buf = [0; 1];
            file.read_exact(&mut buf).unwrap();
            println!("Line {}. Read = {}", i + 2, char::from(buf[0]));
        }
    }

    #[test]
    fn compute_part1() {
        let res = part1();
        assert!(res > 0);
        println!("Result: {}", res);
    }

    #[test]
    fn test_right1_down1() {
        test_slopes(1, 1);
    }

    #[test]
    fn test_right1_down2() {
        test_slopes(1, 2);
    }

    #[test]
    fn compute_part2() {
        let res = part2();
        assert!(res > 0);
        println!("Result: {}", res);
    }
}
