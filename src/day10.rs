use std::collections::HashMap;

const INPUT_PATH: &str = "input/day10";

fn get_differences(input: &mut Vec<u32>) -> u32 {
    input.sort_unstable();

    let mut prev = 0;
    let mut differences = [0; 3];

    for &entry in input.iter() {
        let diff = entry - prev;
        differences[diff as usize - 1] += 1;
        prev = entry;
    }

    // Our built-in adapter
    differences[2] += 1;

    differences[0] * differences[2]
}

fn possible_arrangements(input: &mut Vec<u32>) -> usize {
    fn helper(input: &Vec<u32>, index: usize, cache: &mut HashMap<usize, usize>) -> usize {
        if cache.contains_key(&index) {
            return *cache.get(&index).unwrap();
        }
        if index == input.len() - 1 {
            return 1;
        }

        let mut arrangements = 0;
        // This only works if all entries in input are unique
        for i in (index + 1)..=(index + 3) {
            if i < input.len() && input[i] - input[index] <= 3 {
                arrangements += helper(input, i, cache);
            }
        }
        cache.insert(index, arrangements);

        arrangements
    }

    // We also need the charging outlet and our built-in adapter
    input.push(0);
    input.push(3 + *input.iter().max().unwrap());
    input.sort_unstable();

    helper(input, 0, &mut HashMap::with_capacity(64))
}

pub fn part1() -> u32 {
    let mut input = crate::get_input_as_vec::<u32>(INPUT_PATH);
    get_differences(&mut input)
}

pub fn part2() -> usize {
    let mut input = crate::get_input_as_vec::<u32>(INPUT_PATH);
    possible_arrangements(&mut input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_part1() {
        assert_eq!(part1(), 2475);
    }

    #[test]
    fn compute_part2() {
        assert_eq!(part2(), 442136281481216);
    }
}
