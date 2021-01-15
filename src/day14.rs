use std::collections::HashMap;

const INPUT_PATH: &str = "input/day14";

fn decode<F>(instructions: &[&str], mut f: F) -> HashMap<u64, u64>
where
    F: FnMut(&mut HashMap<u64, u64>, &str, u64, u64),
{
    let mut mask = "";
    let mut address;
    let mut value;
    let mut memory = HashMap::with_capacity(128);

    for instruction in instructions {
        let mut instructions = instruction.split(" = ");
        let lhs = instructions.next().unwrap();
        let rhs = instructions.next().unwrap();

        if let "mask" = lhs {
            mask = rhs;
        } else {
            let lhs = lhs.trim_start_matches("mem[").trim_end_matches("]");
            address = lhs.parse().unwrap();
            value = rhs.parse().unwrap();

            f(&mut memory, mask, address, value);
        }
    }

    memory
}

fn save_to_memory(memory: &mut HashMap<u64, u64>, mask: &str, addr: u64, mut value: u64) {
    mask.chars()
        .enumerate()
        .filter(|(_, bit)| bit.is_numeric())
        .map(|(i, bit)| (i, bit.to_digit(2).unwrap()))
        .for_each(|(i, bit)| {
            let bit = bit as u64;
            if bit == 1 {
                value |= bit << (36 - i - 1);
            } else {
                value &= !(1 << (36 - i - 1));
            }
        });

    memory.insert(addr, value);
}

fn mask_addresses(memory: &mut HashMap<u64, u64>, mask: &str, addr: u64, value: u64) {
    let mut address = String::with_capacity(36);

    mask.chars().enumerate().for_each(|(i, bit)| {
        if let Some(bit) = bit.to_digit(2) {
            let mut res = '1';

            if bit == 0 {
                let b = ((addr >> (36 - i - 1)) & 1) as u8;
                res = format!("{}", b).chars().nth(0).unwrap();
            }

            address.push(res);
        } else {
            address.push('X');
        }
    });

    let count = address.chars().filter(|&bit| bit == 'X').count();
    let total = 2usize.pow(count as u32);
    let mut addresses = Vec::with_capacity(total);

    for i in 0..total {
        let mut curr = address.clone();

        for j in (0..count).rev() {
            let bit = (i >> j) & 1;
            curr = curr.replacen('X', &format!("{}", bit), 1);
        }
        addresses.push(curr);
    }

    for address in &addresses {
        let address = address.parse::<u128>().unwrap();
        memory.insert(address as u64, value);
    }
}

fn addresses_sum(memory: &HashMap<u64, u64>) -> u64 {
    memory.values().sum()
}

pub fn part1() -> u64 {
    let input = crate::get_input_as_vec::<String>(INPUT_PATH);
    let input: Vec<&str> = input.iter().map(|line| line.as_str()).collect();

    let memory = decode(&input, save_to_memory);

    addresses_sum(&memory)
}

pub fn part2() -> u64 {
    let input = crate::get_input_as_vec::<String>(INPUT_PATH);
    let input: Vec<&str> = input.iter().map(|line| line.as_str()).collect();

    let memory = decode(&input, mask_addresses);

    addresses_sum(&memory)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_part1() {
        assert_eq!(part1(), 7817357407588);
    }

    #[test]
    fn compute_part2() {
        assert_eq!(part2(), 4335927555692);
    }

    #[test]
    fn part1_example() {
        let mut instructions = Vec::new();
        instructions.push("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        instructions.push("mem[8] = 11");
        instructions.push("mem[7] = 101");
        instructions.push("mem[8] = 0");

        let memory = decode(&instructions, save_to_memory);

        assert_eq!(addresses_sum(&memory), 165);
    }

    #[test]
    fn part2_example() {
        let mut instructions = Vec::new();
        instructions.push("mask = 000000000000000000000000000000X1001X");
        instructions.push("mem[42] = 100");
        instructions.push("mask = 00000000000000000000000000000000X0XX");
        instructions.push("mem[26] = 1");

        let memory = decode(&instructions, mask_addresses);

        assert_eq!(addresses_sum(&memory), 208);
    }
}
