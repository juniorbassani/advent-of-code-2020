use std::fs::File;
use std::io::{ErrorKind, Read, Seek, SeekFrom};

const INPUT_PATH: &str = "input/day03-input";

fn next_slope(file: &mut File) {
    let mut buf = [0; 1];

    // If we encounter a newline, we have to reposition the cursor to the beginning of the line
    for _ in 0..3 {
        file.read_exact(&mut buf).unwrap();
        if buf[0] == b'\n' {
            // For some reason, seeking to -33 an removing the read causes a panic
            file.seek(SeekFrom::Current(-32)).unwrap();
            file.read_exact(&mut buf).unwrap();
        }
    }

    file.seek(SeekFrom::Current(31)).unwrap();
}

pub fn part1() -> u32 {
    let mut file = File::open(INPUT_PATH).unwrap();
    file.seek(SeekFrom::Start(1)).unwrap();
    let mut buf = [0; 1];
    let mut num_of_trees = 0;

    loop {
        next_slope(&mut file);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_slope() {
        let mut file = File::open(INPUT_PATH).unwrap();
        file.seek(SeekFrom::Start(1)).unwrap();

        for i in 0..20 {
            next_slope(&mut file);
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
}
