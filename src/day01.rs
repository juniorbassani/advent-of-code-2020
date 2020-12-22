use std::path::PathBuf;
use std::io::{BufReader, BufRead};
use std::fs::File;

const QUERY: u32 = 2020;

fn get_input(path: &PathBuf) -> Vec<u32> {
    let file = BufReader::new(File::open(path).expect("File not found"));
    file.lines()
        .map(|line| line.unwrap().parse::<u32>().expect("Non number found"))
        .collect()
}

pub fn part1(path: &PathBuf) -> Option<u32> {
    let numbers = get_input(path);

    if numbers.len() < 2 {
        return None;
    }

    for i in 0..numbers.len() {
        for j in 1..numbers.len() {
            let num1 = numbers.get(i).unwrap();
            let num2 = numbers.get(j).unwrap();

            if num1 + num2 == QUERY {
                return Some(num1 * num2);
            }
        }
    }

    None
}

pub fn part2(path: &PathBuf) -> Option<u32> {
    let numbers = get_input(path);

    if numbers.len() < 3 {
        return None;
    }

    // Horrible performance, but works
    for i in 0..numbers.len() {
        for j in 1..numbers.len() {
            for k in 2..numbers.len() {
                let num1 = numbers.get(i).unwrap();
                let num2 = numbers.get(j).unwrap();
                let num3 = numbers.get(k).unwrap();

                if num1 + num2 + num3 == QUERY {
                    return Some(num1 * num2 * num3);
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_part1() {
        let result = part1(&PathBuf::from("input/day01-input"));
        assert!(result.is_some());
        println!("Result: {}", result.unwrap());
    }

    #[test]
    fn compute_part2() {
        let result = part2(&PathBuf::from("input/day01-input"));
        assert!(result.is_some());
        println!("Result: {}", result.unwrap());
    }
}