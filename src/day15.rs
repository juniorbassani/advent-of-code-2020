use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn count(numbers: &[i32]) -> i32 {
    let mut spoken = HashMap::with_capacity(50);

    for (i, &starting_number) in numbers.iter().enumerate() {
        spoken.insert(starting_number, (1, i, i));
    }

    let mut last = numbers[numbers.len() - 1];

    for i in numbers.len()..2020 {
        let next: i32;

        let &(count, prev, most_recent) = spoken.get(&last).unwrap();

        if count == 1 {
            next = 0;
        } else if count == last && most_recent - prev == 1 {
            next = 1;
        } else {
            next = (most_recent - prev) as i32;
        }

        spoken.entry(next).or_insert((0, i, i)).0 += 1;

        match spoken.entry(next) {
            Entry::Occupied(mut entry) => {
                let (count, prev, most_recent) = entry.get_mut();
                *count += 1;
                *prev = *most_recent;
                *most_recent = i;
            }
            Entry::Vacant(entry) => {
                entry.insert((0, i, i));
            }
        }

        last = next;
    }

    last
}

pub fn part1() -> i32 {
    let input = [14, 1, 17, 0, 3, 20];
    count(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_part1() {
        assert_eq!(part1(), 387);
    }

    // #[test]
    // fn compute_part2() {
    //     assert_eq!(part2(), 4335927555692);
    // }

    #[test]
    fn part1_examples() {
        let a = vec![0, 3, 6];
        let b = vec![1, 3, 2];
        let c = vec![2, 1, 3];
        let d = vec![1, 2, 3];
        let e = vec![2, 3, 1];
        let f = vec![3, 2, 1];
        let g = vec![3, 1, 2];

        assert_eq!(count(&a), 436);
        assert_eq!(count(&b), 1);
        assert_eq!(count(&c), 10);
        assert_eq!(count(&d), 27);
        assert_eq!(count(&e), 78);
        assert_eq!(count(&f), 438);
        assert_eq!(count(&g), 1836);
    }

    // #[test]
    // fn part2_example() {
    //     let mut instructions = Vec::new();
    //     instructions.push("mask = 000000000000000000000000000000X1001X");
    //     instructions.push("mem[42] = 100");
    //     instructions.push("mask = 00000000000000000000000000000000X0XX");
    //     instructions.push("mem[26] = 1");

    //     let memory = decode(&instructions, mask_addresses);

    //     assert_eq!(addresses_sum(&memory), 208);
    // }
}
