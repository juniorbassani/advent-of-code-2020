use regex::Regex;
use std::io::BufRead;

const INPUT_PATH: &str = "input/day04";

lazy_static! {
    // Valid pattern example:
    //
    // "byr:2001 iyr:2011
    // ecl:brn
    // pid:487702556 hcl:#602927
    // hgt:167cm eyr:2026\n"
    //
    static ref RE: Regex = Regex::new(r"([a-z]+):[[[:ascii:]]&&[^\s\n]]+[\s\n]").unwrap();
}

pub fn part1() -> u32 {
    let mut file = crate::get_buffered_input(INPUT_PATH);
    let mut buf = String::with_capacity(128);
    let mut valid_passports = 0;
    let mut should_break = false;

    loop {
        let bytes = file.read_line(&mut buf).expect("Can't read");

        if bytes == 0 {
            should_break = true;
        } else {
            if buf[buf.len() - bytes..buf.len() - bytes + 1].ne("\n") {
                continue;
            }
        }

        let required_docs = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let available_docs: Vec<_> = RE
            .captures_iter(&buf)
            .map(|cap| cap.get(1).unwrap().as_str())
            .collect();
        let mut valid = true;

        // Don't process the documentation if there's not enough
        if available_docs.len() >= 7 && available_docs.len() <= 8 {
            for required_doc in required_docs.iter() {
                if !available_docs.contains(required_doc) {
                    valid = false;
                    break;
                }
            }

            if valid {
                valid_passports += 1;
            }
        }

        if should_break {
            break;
        }

        buf.clear();
    }

    valid_passports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex() {
        let a = "byr:1994 hcl:#ceb3a1
pid:028071950 eyr:2022 ecl:gmt hgt:151in iyr:2016 ";
        let b = "byr:2001 iyr:2011
ecl:brn
pid:487702556 hcl:#602927
hgt:167cm eyr:2026\n";
        let c = "byr2001iyr2011";

        assert!(RE.is_match(a));
        assert!(RE.is_match(b));
        assert!(!RE.is_match(c));

        for caps in RE.captures_iter(a) {
            println!("{}", &caps[1]);
        }
        println!();
    }

    #[test]
    fn compute_part1() {
        let res = part1();
        assert!(res > 0);
        println!("{}", res);
    }
}
