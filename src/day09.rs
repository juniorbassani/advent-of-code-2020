const INPUT_PATH: &str = "input/day09";

fn get_mismatched_number(input: &Vec<u64>, mut preamble: usize) -> u64 {
    let mut skip = 0;

    while let Some(&query) = input.iter().nth(preamble) {
        let data: Vec<&u64> = input.iter().skip(skip).take(preamble).collect();
        let mut should_break = false;

        'outer: for i in 0..data.len() {
            for j in 1..data.len() {
                if i != j && data[i] + data[j] == query {
                    should_break = true;
                    break 'outer;
                }
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

fn find_set(input: &mut Vec<u64>, query: u64) -> u64 {
    let preceding = input.split(|&num| num == query).next().unwrap();
    let mut i = preceding.len() - 1;
    let mut j = i - 1;
    let mut found = None;

    while let None = found {
        let mut sum = preceding[j..=i].iter().sum::<u64>();

        while j > 0 && sum < query {
            j -= 1;
            sum = preceding[j..=i].iter().sum::<u64>();
        }

        if sum == query {
            let min = *preceding[j..=i].iter().min().unwrap();
            let max = *preceding[j..=i].iter().max().unwrap();
            found = Some(min + max);
        } else {
            i -= 1;
            j = i - 1;
        }
    }

    found.unwrap()
}

pub fn part1() -> u64 {
    let input = crate::get_input_as_vec::<u64>(INPUT_PATH);
    get_mismatched_number(&input, 25)
}

pub fn part2() -> u64 {
    const QUERY: u64 = 1124361034;
    let mut input = crate::get_input_as_vec::<u64>(INPUT_PATH);

    find_set(&mut input, QUERY)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_part1() {
        assert_eq!(part1(), 1124361034);
    }

    #[test]
    fn compute_part2() {
        assert_eq!(part2(), 129444555);
    }
}
