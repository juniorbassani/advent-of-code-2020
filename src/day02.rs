use regex::Regex;
use std::io::BufRead;
use std::str::FromStr;

const INPUT_PATH: &str = "input/day02-input";

lazy_static! {
    // valid regex example: 14-19 x: lrxxxtxxxxxxxzxxxxx
    static ref RE: Regex = Regex::new(
        r"(?x)
        ^(?P<min>[0-9]*)
        -
        (?P<max>[0-9]*)
        \s
        (?P<char>[a-z]):
        \s
        (?P<text>[a-z]*)$
        "
    )
    .expect("Couldn't create Regex");
}

fn valid_passwords<P>(policy: P) -> usize
where
    P: Fn(&str) -> bool,
{
    crate::get_buffered_input(INPUT_PATH)
        .lines()
        .filter(|line| policy(line.as_ref().unwrap()))
        .count()
}

pub fn part1() -> usize {
    valid_passwords(|line| {
        let caps = RE.captures(line).unwrap();
        let text = &caps["text"];
        let matches = text.matches(&caps["char"]).count();

        matches >= caps["min"].parse().unwrap() && matches <= caps["max"].parse().unwrap()
    })
}

pub fn part2() -> usize {
    valid_passwords(|line| {
        let caps = RE.captures(line).unwrap();
        let text = &caps["text"];
        let letter = char::from_str(&caps["char"]).unwrap();
        let first_idx = &caps["min"].parse::<usize>().unwrap() - 1;
        let last_idx = &caps["max"].parse::<usize>().unwrap() - 1;

        // XOR operation
        (text.chars().nth(first_idx).unwrap() == letter)
            ^ (text.chars().nth(last_idx).unwrap() == letter)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_par1() {
        let valid_pwds = part1();
        println!("Result: {}", valid_pwds);
    }

    #[test]
    fn compute_part2() {
        let valid_pwds = part2();
        println!("Result: {}", valid_pwds);
    }

    #[test]
    fn test_regex() {
        let regex = Regex::new(r"^[0-9]*-[0-9]* [a-z]: [a-z]*$").expect("Couldn't create Regex");
        assert!(regex.is_match("2-5 l: fllxf"));
        assert!(regex.is_match("14-19 x: lrxxxtxxxxxxxzxxxxx"));
        assert!(regex.is_match("12-16 t: vdtbdtxtttttrctttkt"));
        assert!(!regex.is_match("a-16 t: vdtbdtxtttttrctttkt"));
        assert!(!regex.is_match("12-16 t: 1vdtbdtxtttttrctttkt"));
    }
}
