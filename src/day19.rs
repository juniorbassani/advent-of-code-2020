use std::collections::HashMap;

const INPUT_PATH: &str = "input/day19";

fn valid_messages(rules: &HashMap<u32, &str>, messages: &[&str]) -> u32 {
    fn helper<'a>(
        rules: &HashMap<u32, &str>,
        rule: (u32, &str),
        mut msg: &'a str,
    ) -> (bool, &'a str) {
        if rule.1.contains('\"') {
            let letter = rule.1.chars().nth(1).unwrap();

            if let Some(a) = msg.chars().nth(0) {
                return (a == letter, &msg[1..]);
            } else {
                return (true, "");
            }
        }

        let tmp = msg;
        (
            rule.1.split("|").any(|rule| {
                msg = tmp;
                rule.trim().split_whitespace().all(|rule| {
                    if msg.is_empty() {
                        return true;
                    }
                    let (&r, &val) = rules.get_key_value(&rule.parse::<u32>().unwrap()).unwrap();
                    let res = helper(rules, (r, val), msg);
                    msg = res.1;
                    res.0
                })
            }),
            msg,
        )
    }

    messages.iter().fold(0, |acc, &msg| {
        let rule = rules.get_key_value(&0).unwrap();
        let rule = (*rule.0, *rule.1);

        if let (true, "") = helper(rules, rule, msg) {
            acc + 1
        } else {
            acc
        }
    })
}

fn parse_input(input: &str) -> (HashMap<u32, &str>, Vec<&str>) {
    let mut rules = HashMap::with_capacity(150);
    let mut msgs = Vec::with_capacity(256);
    let mut input = input.split("\n\n");
    let r = input.next().unwrap();

    for rule in r.lines() {
        let mut a = rule.split(": ");
        let lhs = a.next().unwrap();
        let rhs = a.next().unwrap();
        rules.insert(lhs.parse::<u32>().unwrap(), rhs);
    }

    for msg in input.next().unwrap().lines() {
        msgs.push(msg);
    }

    (rules, msgs)
}

pub fn part1() -> u32 {
    let input = crate::get_input_as_string(INPUT_PATH);
    let (rules, msgs) = parse_input(&input);

    valid_messages(&rules, &msgs)
}

pub fn part2() -> u32 {
    let input = crate::get_input_as_string(INPUT_PATH);
    let (mut rules, msgs) = parse_input(&input);

    rules.insert(8, "42 | 42 8");
    rules.insert(11, "42 31 | 42 11 31");

    valid_messages(&rules, &msgs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_part1() {
        assert_eq!(part1(), 139);
    }

    #[test]
    fn compute_part2() {
        println!("{}", part2());
        // assert_eq!(part2(), 0);
    }

    #[test]
    fn part1_example() {
        let mut rules1 = HashMap::new();
        rules1.insert(0, "1 2");
        rules1.insert(1, "\"a\"");
        rules1.insert(2, "1 3 | 3 1");
        rules1.insert(3, "\"b\"");
        let msgs = ["aab", "aba", "bab"];

        assert_eq!(valid_messages(&rules1, &msgs), 2);

        let mut rules2 = rules1;
        rules2.clear();

        rules2.insert(0, "4 1 5");
        rules2.insert(1, "2 3 | 3 2");
        rules2.insert(2, "4 4 | 5 5");
        rules2.insert(3, "4 5 | 5 4");
        rules2.insert(4, "\"a\"");
        rules2.insert(5, "\"b\"");
        let msgs = ["ababbb", "bababa", "abbbab", "aaabbb", "aaaabbb"];

        assert_eq!(valid_messages(&rules2, &msgs), 2);
    }

    #[test]
    fn part2_example() {
        let mut rules = HashMap::new();
        rules.insert(42, "9 14 | 10 1");
        rules.insert(9, "14 27 | 1 26");
        rules.insert(10, "23 14 | 28 1");
        rules.insert(1, "\"a\"");
        rules.insert(11, "42 31");
        rules.insert(5, "1 14 | 15 1");
        rules.insert(19, "14 1 | 14 14");
        rules.insert(12, "24 14 | 19 1");
        rules.insert(16, "15 1 | 14 14");
        rules.insert(31, "14 17 | 1 13");
        rules.insert(6, "14 14 | 1 14");
        rules.insert(2, "1 24 | 14 4");
        rules.insert(0, "8 11");
        rules.insert(13, "14 3 | 1 12");
        rules.insert(15, "1 | 14");
        rules.insert(17, "14 2 | 1 7");
        rules.insert(23, "25 1 | 22 14");
        rules.insert(28, "16 1");
        rules.insert(4, "1 1");
        rules.insert(20, "14 14 | 1 15");
        rules.insert(3, "5 14 | 16 1");
        rules.insert(27, "1 6 | 14 18");
        rules.insert(14, "\"b\"");
        rules.insert(21, "14 1 | 1 14");
        rules.insert(25, "1 1 | 1 14");
        rules.insert(22, "14 14");
        rules.insert(8, "42");
        rules.insert(26, "14 22 | 1 20");
        rules.insert(18, "15 15");
        rules.insert(7, "14 5 | 1 21");
        rules.insert(24, "14 1");

        let msgs = [
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
            "bbabbbbaabaabba",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "bbbbbbbaaaabbbbaaabbabaaa",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "baabbaaaabbaaaababbaababb",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "aaaaabbaabaaaaababaa",
            "aaaabbaaaabbaaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "babaaabbbaaabaababbaabababaaab",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        ];

        assert_eq!(valid_messages(&rules, &msgs), 3);

        rules.insert(8, "42 | 42 8");
        rules.insert(11, "42 31 | 42 11 31");

        assert_eq!(valid_messages(&rules, &msgs), 12);
    }
}
