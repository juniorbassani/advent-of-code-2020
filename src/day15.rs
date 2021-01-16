use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn count(numbers: &[i32], up_to: usize) -> i32 {
    let mut spoken = HashMap::with_capacity(64);

    for (i, &starting_number) in numbers.iter().enumerate() {
        spoken.insert(starting_number, (1, i, i));
    }

    let mut last = numbers[numbers.len() - 1];

    for i in numbers.len()..up_to {
        let next;
        let &(count, prev, most_recent) = spoken.get(&last).unwrap();

        if count == 1 {
            next = 0;
        } else if count == last && most_recent - prev == 1 {
            next = 1;
        } else {
            next = (most_recent - prev) as i32;
        }

        match spoken.entry(next) {
            Entry::Occupied(mut entry) => {
                let (count, prev, most_recent) = entry.get_mut();
                *count += 1;
                *prev = *most_recent;
                *most_recent = i;
            }
            Entry::Vacant(entry) => {
                entry.insert((1, i, i));
            }
        }

        last = next;
    }

    last
}

pub fn part1() -> i32 {
    let input = [14, 1, 17, 0, 3, 20];
    count(&input, 2020)
}

pub fn part2() -> i32 {
    let input = [14, 1, 17, 0, 3, 20];
    count(&input, 30000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_part1() {
        assert_eq!(part1(), 387);
    }

    #[test]
    fn compute_part2() {
        assert_eq!(part2(), 6428);
    }

    #[test]
    fn part1_examples() {
        let a = vec![0, 3, 6];
        let b = vec![1, 3, 2];
        let c = vec![2, 1, 3];
        let d = vec![1, 2, 3];
        let e = vec![2, 3, 1];
        let f = vec![3, 2, 1];
        let g = vec![3, 1, 2];

        assert_eq!(count(&a, 2020), 436);
        assert_eq!(count(&b, 2020), 1);
        assert_eq!(count(&c, 2020), 10);
        assert_eq!(count(&d, 2020), 27);
        assert_eq!(count(&e, 2020), 78);
        assert_eq!(count(&f, 2020), 438);
        assert_eq!(count(&g, 2020), 1836);
    }

    #[test]
    fn part2_examples() {
        let a = vec![0, 3, 6];
        assert_eq!(count(&a, 30000000), 175594);
    }
}
