const INPUT_PATH: &str = "input/day09";

fn get_mismatched_number(input: &Vec<u64>, mut preamble: usize) -> u64 {
    let mut skip = 0;

    while let Some(&query) = input.iter().nth(preamble) {
        let data: Vec<&u64> = input.iter().skip(skip).take(preamble).collect();
        let mut should_break = false;

        for i in 0..data.len() {
            for j in 1..data.len() {
                if i != j && data[i] + data[j] == query {
                    should_break = true;
                    break;
                }
            }

            if should_break {
                break;
            }
        }

        if !should_break {
            return query;
        }

        preamble += 1;
        skip += 1;
    }

    panic!("All numbers match");
}

pub fn part1() -> u64 {
    let input = crate::get_input_as_vec::<u64>(INPUT_PATH);
    get_mismatched_number(&input, 25)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_part1() {
        assert_eq!(part1(), 1124361034);
    }
}
