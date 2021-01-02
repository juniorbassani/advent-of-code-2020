use std::collections::HashMap;

const INPUT_PATH: &str = "input/day07";

fn shiny_gold_holders(bags_policy: HashMap<&str, Vec<&str>>) -> u32 {
    // Still unoptimized, but input is small
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_part1() {
        assert_eq!(part1(), 248);
    }
}
