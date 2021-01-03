// shiny gold bags contain 3 wavy gold bags, 2 plaid chartreuse bags, 2 shiny lime bags, 5 dull indigo bags.
// wavy gold bags contain 5 dull black bags, 1 muted fuchsia bag, 4 dark red bags, 1 light crimson bag.
// dull black bags contain 2 shiny lime bags, 3 muted indigo bags, 5 faded beige bags.
// shiny lime bags contain 1 faded teal bag, 1 light crimson bag.
// faded teal bags contain no other bags.
// light crimson bags contain no other bags.

use std::collections::HashMap;

const INPUT_PATH: &str = "input/day07";

fn shiny_gold_holders(bags_policy: HashMap<&str, Vec<&str>>) -> u32 {
    // TODO: use memoization to optimize recursive calls
    fn supports_shiny_gold_bag(map: &HashMap<&str, Vec<&str>>, value: &Vec<&str>) -> bool {
        if value.contains(&"no other bags") {
            return false;
        }

        if value.contains(&"shiny gold") {
            return true;
        } else {
            for &entry in value.iter() {
                if let Some(value) = map.get(entry) {
                    if supports_shiny_gold_bag(map, value) {
                        return true;
                    }
                }
            }
        }

        false
    }

    bags_policy
        .iter()
        .filter(|(&key, value)| key != "shiny gold" && supports_shiny_gold_bag(&bags_policy, value))
        .count() as u32
}

pub fn part1() -> u32 {
    let file = crate::get_input_as_string(INPUT_PATH);
    let mut bags_policy = HashMap::with_capacity(128);

    for policy in file.lines() {
        let bag = policy.split(" bags").next().unwrap();
        let supported_bags = policy
            .split("contain ")
            .skip(1)
            .next()
            .unwrap()
            .trim_end_matches('.')
            .split(", ")
            .map(|bag| {
                bag.trim_start_matches(char::is_numeric)
                    .trim_end_matches("bag")
                    .trim_end_matches("bags")
                    .trim()
            })
            .collect::<Vec<_>>();

        // The inserted key/value pairs will be of the form:
        // k: "muted lavender" | v: ["dull brown", "pale maroon", "drab orange"]
        bags_policy.insert(bag, supported_bags);
    }

    shiny_gold_holders(bags_policy)
}

fn number_of_bags(map: &HashMap<&str, Vec<&str>>) -> u32 {
    // TODO: use memoization to optimize recursive calls
    fn helper(bag: &Vec<&str>, map: &HashMap<&str, Vec<&str>>) -> u32 {
        if bag.contains(&"no other") {
            return 1;
        }

        let mut counter = 0;
        for &value in bag.iter() {
            let qty = value.chars().nth(0).unwrap().to_digit(10).unwrap();
            let value = value.trim_start_matches(char::is_numeric).trim();
            let next = map.get(value).unwrap();
            let res = helper(next, map);

            counter += qty * res;
            if res > 1 {
                counter += qty;
            }
        }

        counter
    }

    let shiny_gold = map
        .get(&"shiny gold")
        .expect("Input doesn't contain shiny gold bag");

    helper(shiny_gold, map)
}

pub fn part2() -> u32 {
    let file = crate::get_input_as_string(INPUT_PATH);
    let mut bags_policy = HashMap::with_capacity(128);

    for policy in file.lines() {
        let bag = policy.split(" bags").next().unwrap();
        let supported_bags = policy
            .split("contain ")
            .skip(1)
            .next()
            .unwrap()
            .trim_end_matches('.')
            .split(", ")
            .map(|bag| bag.trim_end_matches("bag").trim_end_matches("bags").trim())
            .collect::<Vec<_>>();

        // The inserted key/value pairs will be of the form:
        // k: "muted lavender" | v: ["5 dull brown", "4 pale maroon", "2 drab orange"]
        bags_policy.insert(bag, supported_bags);
    }

    number_of_bags(&bags_policy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_part1() {
        assert_eq!(part1(), 248);
    }

    #[test]
    fn compute_part2() {
        assert_eq!(part2(), 57281);
    }

    #[test]
    fn test_helper() {
        let mut map = HashMap::new();
        map.insert("shiny gold", vec!["1 dark olive", "2 vibrant plum"]);
        map.insert("dark olive", vec!["3 faded blue", "4 dotted black"]);
        map.insert("faded blue", vec!["no other"]);
        map.insert("dotted black", vec!["no other"]);
        map.insert("vibrant plum", vec!["5 faded blue", "6 dotted black"]);
        assert_eq!(32, number_of_bags(&map));

        let mut map = HashMap::new();
        map.insert("shiny gold", vec!["2 dark red"]);
        map.insert("dark red", vec!["2 dark orange"]);
        map.insert("dark orange", vec!["2 dark yellow"]);
        map.insert("dark yellow", vec!["2 dark green"]);
        map.insert("dark green", vec!["2 dark blue"]);
        map.insert("dark blue", vec!["2 dark violet"]);
        map.insert("dark violet", vec!["no other"]);
        assert_eq!(126, number_of_bags(&map));
    }
}
