use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

const INPUT_PATH: &str = "input/day06";

lazy_static! {
    static ref CONTENTS: String = fs::read_to_string(INPUT_PATH).unwrap();
}

pub fn part1() -> usize {
    CONTENTS.split("\n\n").fold(0, |acc, answers| {
        let answers = answers.split_whitespace().collect::<String>();
        let count = answers.chars().unique().count();
        acc + count
    })
}

pub fn part2() -> usize {
    let mut answer_counter = HashMap::with_capacity(26);
    let mut all_answers = String::with_capacity(128);

    CONTENTS.split("\n\n").fold(0, |acc, answers| {
        for question in 'a'..='z' {
            answer_counter.insert(question, 0);
        }

        let group_size = answers.split_whitespace().count();
        all_answers = answers.split_whitespace().collect();

        for answer in all_answers.chars() {
            *answer_counter.get_mut(&answer).unwrap() += 1;
        }

        acc + answer_counter
            .values()
            .filter(|&&count| count == group_size)
            .count()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_part1() {
        assert_eq!(part1(), 6273);
    }

    #[test]
    fn compute_part2() {
        assert_eq!(part2(), 3254);
    }
}
