use regex::Regex;
use std::collections::HashMap;
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
    static ref RE: Regex = Regex::new(r"([a-z]+):([[[:ascii:]]&&[^\s\n]]+)[\s\n]").unwrap();
}

fn valid_passports<F>(validation_rules: F) -> u32
where
    F: Fn(&Vec<(&str, &str)>) -> bool,
{
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

        let available_docs: Vec<_> = RE
            .captures_iter(&buf)
            .map(|cap| (cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str()))
            .collect();

        // Don't process the documentation if there's not enough
        if available_docs.len() >= 7 && available_docs.len() <= 8 {
            if validation_rules(&available_docs) {
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

fn check_required_docs(docs: &Vec<(&str, &str)>) -> bool {
    let required_docs = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let docs: Vec<&str> = docs.iter().map(|doc| doc.0).collect();

    for required_doc in required_docs.iter() {
        if !docs.contains(required_doc) {
            return false;
        }
    }

    true
}

pub fn part1() -> u32 {
    valid_passports(check_required_docs)
}

// The answer is one above the correct
pub fn part2() -> u32 {
    let mut all_docs = HashMap::with_capacity(8);
    all_docs.insert("byr", Regex::new(r"\d{4}").unwrap());
    all_docs.insert("iyr", Regex::new(r"\d{4}").unwrap());
    all_docs.insert("eyr", Regex::new(r"\d{4}").unwrap());
    all_docs.insert("hgt", Regex::new(r"(\d{2,3})(cm|in)").unwrap());
    all_docs.insert("hcl", Regex::new(r"#[[0-9][a-f]]{6}").unwrap());
    all_docs.insert("ecl", Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap());
    all_docs.insert("pid", Regex::new(r"\d{9}").unwrap());
    all_docs.insert("cid", Regex::new(r".").unwrap());

    valid_passports(|passport| {
        if !check_required_docs(passport) {
            return false;
        }

        let mut valid = false;

        for doc in passport {
            let value = doc.1;
            let doc = doc.0;
            valid = false;

            if let Some(pattern) = all_docs.get(doc) {
                if pattern.is_match(value) {
                    match doc {
                        "byr" => {
                            // byr (Birth Year) - four digits; at least 1920 and at most 2002
                            if let Ok(1920..=2002) = value.parse() {
                                valid = true;
                            }
                        }
                        "iyr" => {
                            // iyr (Issue Year) - four digits; at least 2010 and at most 2020
                            if let Ok(2010..=2020) = value.parse() {
                                valid = true;
                            }
                        }
                        "eyr" => {
                            // eyr (Expiration Year) - four digits; at least 2020 and at most 2030
                            if let Ok(2020..=2030) = value.parse() {
                                valid = true;
                            }
                        }
                        "hgt" => {
                            // hgt (Height) - a number followed by either cm or in:
                            //     If cm, the number must be at least 150 and at most 193.
                            //     If in, the number must be at least 59 and at most 76.
                            let unit = pattern.captures(value).unwrap();
                            let height = unit[1].parse();
                            if &unit[2] == "cm" {
                                if let Ok(150..=193) = height {
                                    valid = true;
                                }
                            } else {
                                if let Ok(59..=76) = height {
                                    valid = true;
                                }
                            }
                        }
                        "hcl" | "ecl" | "pid" | "cid" => {
                            // Already validated by the regex
                            valid = true;
                        }
                        _ => {}
                    }
                }
            }

            if !valid {
                break;
            }
        }

        valid
    })
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

        let hcl = Regex::new(r"#[[0-9][a-f]]{6}").unwrap();
        let hgt = Regex::new(r"(\d){1,3}(cm|in)").unwrap();
        let cid = Regex::new(r".").unwrap();

        assert!(hcl.is_match("#353bc9"));
        assert!(hgt.is_match("170cm"));
        assert!(hgt.is_match("167cm"));
        assert!(hgt.is_match("59in"));
        assert!(cid.is_match("aaaaa"));
        assert!(cid.is_match(" "));

        assert!(!hcl.is_match("#11bdh2"));
        assert!(!hcl.is_match("11bda2"));
        assert!(!hgt.is_match("59"));
        assert!(!hgt.is_match("59em"));
    }

    #[test]
    fn test_regex_alternation() {
        let re = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();
        let a = "amb";
        let b = "oth";
        let c = "gry";
        let d = "blr";

        assert!(re.is_match(a));
        assert!(re.is_match(b));
        assert!(re.is_match(c));
        assert!(!re.is_match(d));
    }

    #[test]
    fn compute_part1() {
        let res = part1();
        assert!(res > 0);
        println!("{}", res);
    }

    #[test]
    fn compute_part2() {
        let res = part2();
        assert!(res > 0);
        println!("{}", res);
    }
}
