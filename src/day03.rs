use std::fs::File;
use std::io::{ErrorKind, Read, Seek, SeekFrom};

const INPUT_PATH: &str = "input/day03-input";

pub fn part1() -> u32 {
    let mut file = File::open(INPUT_PATH).unwrap();
    file.seek(SeekFrom::Start(1)).expect("Can't seek");
    let mut buf = [0; 1];
    let mut num_of_trees = 0;

    let mut line = 2;

    loop {
        for _ in 0..10 {
            let pos = file.seek(SeekFrom::Current(34)).expect("Can't seek");
            match file.read_exact(&mut buf) {
                Ok(()) => {
                    if buf[0] == b'#' {
                        num_of_trees += 1;
                    } // else if buf[0] == b'\n' {
                      //     file.seek(SeekFrom::Current(-32)).expect("Can't seek");
                      //     println!("newline");
                      //     continue;
                      // }
                }
                Err(err) => {
                    if let ErrorKind::UnexpectedEof = err.kind() {
                        return num_of_trees;
                    } else {
                        panic!("Unexpected error");
                    }
                }
            }
            println!(
                "line = {}, pos = {}, char = {}",
                line,
                pos + 1,
                if buf[0] == b'\n' {
                    'n'
                } else {
                    char::from(buf[0])
                }
            );
            line += 1;
        }
        println!();
        file.seek(SeekFrom::Current(-31)).expect("Can't seek");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_part1() {
        let res = part1();
        assert!(res > 0);
        println!("Result: {}", res);
    }
}
