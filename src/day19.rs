use std::collections::HashMap;

const INPUT_PATH: &str = "input/day19";

fn valid_messages(rules: &HashMap<u32, &str>, messages: &[&str]) -> u32 {
    fn helper<'a>(rules: &HashMap<u32, &str>, rule: &str, mut msg: &'a str) -> &'a str {
        if rule.contains('\"') {
            let letter = rule.chars().nth(1).unwrap();

            if msg.chars().nth(0).unwrap() == letter {
                return &msg[1..];
            } else {
                return msg;
            }
        }

        let tmp = msg;
        rule.split(" | ").any(|rule| {
            msg = tmp;
            rule.split_whitespace().all(|rule| {
                let val = rules[&rule.parse::<u32>().unwrap()];
                let res = helper(rules, val, msg);
                let result = res.len() < msg.len();

                if result {
                    msg = res;
                } else {
                    msg = tmp;
                }

                result
            })
        });

        msg
    }

    messages.iter().fold(0, |acc, &msg| {
        if helper(rules, rules[&0], msg).is_empty() {
            acc + 1
        } else {
            acc
        }
    })
}

pub fn part1() -> u32 {
    let mut rules = HashMap::with_capacity(150);
    let mut msgs = Vec::with_capacity(256);
    let input = crate::get_input_as_string(INPUT_PATH);
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

    valid_messages(&rules, &msgs)
}

pub fn part2() -> i32 {
    0
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
        assert_eq!(true, true);
    }
}
