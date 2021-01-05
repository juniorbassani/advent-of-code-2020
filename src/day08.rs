use itertools::Itertools;
use std::io::BufRead;

const INPUT_PATH: &str = "input/day08";

fn get_input() -> Vec<String> {
    let file = crate::get_buffered_input(INPUT_PATH);
    let mut instructions = Vec::with_capacity(636);

    for line in file.lines() {
        instructions.push(line.unwrap());
    }

    instructions
}

fn process_input<F>(instructions: &Vec<String>, mut index: usize, mut process: F)
where
    F: FnMut(usize, &str, &str, isize) -> usize,
{
    let mut instructions: Vec<_> = instructions
        .iter()
        .zip(vec![false; instructions.len()])
        .collect();

    while let Some((instruction, visited @ false)) = instructions.get_mut(index) {
        *visited = true;
        let (op, arg) = instruction.split(' ').collect_tuple().unwrap();
        let signal = &arg[..1];
        let arg = arg[1..].parse::<isize>().unwrap();
        index = process(index, op, signal, arg);
    }
}

fn get_acc(input: &Vec<String>) -> isize {
    let mut acc = 0;

    process_input(&input, 0, |mut index, op, signal, arg| {
        match op {
            "acc" => {
                if signal == "+" {
                    acc += arg;
                } else {
                    acc -= arg;
                }
                index += 1;
            }
            "jmp" => {
                if signal == "+" {
                    index += arg as usize;
                } else {
                    index -= arg as usize;
                }
            }
            _ => {
                index += 1;
            }
        }

        index
    });

    acc
}

pub fn part1() -> isize {
    get_acc(&get_input())
}

pub fn part2() -> isize {
    fn helper(instructions: &Vec<String>, mut index: usize) -> Result<(), Vec<usize>> {
        let mut possibly_corrupted = Vec::with_capacity(32);

        process_input(&instructions, index, |mut i, op, signal, arg| {
            match op {
                "jmp" => {
                    if arg > 1 {
                        possibly_corrupted.push(i);
                    }
                    if signal == "+" {
                        i += arg as usize;
                    } else {
                        i -= arg as usize;
                    }
                }
                _ => {
                    if op == "nop" && arg != 0 {
                        possibly_corrupted.push(i);
                    }
                    i += 1;
                }
            }

            index = i;
            i
        });

        if index < instructions.len() {
            Err(possibly_corrupted)
        } else {
            Ok(())
        }
    };

    let mut instructions = get_input();
    let res = helper(&instructions, 0);

    // Once the helper function returns, we know one of the entries is the corrupted one; so use
    // brute force to identify which one it is
    for index in res.unwrap_err().into_iter() {
        let instruction = instructions.get_mut(index).unwrap();
        let ins = instruction.to_owned();
        let (op, arg) = ins.split(' ').collect_tuple().unwrap();
        let signal = &arg[..1];
        let arg = &arg[1..];

        if op.contains("jmp") {
            *instruction = String::from("nop ");
        } else {
            *instruction = String::from("jmp ");
        }

        instruction.push_str(signal);
        instruction.push_str(arg);

        if let Ok(()) = helper(&instructions, index) {
            break;
        } else {
            // Restore the modified instruction, so we can test another one
            let instruction = instructions.get_mut(index).unwrap();
            *instruction = format!("{} {}{}", op, signal, arg);
        }
    }

    get_acc(&instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_part1() {
        assert_eq!(part1(), 1939);
    }

    #[test]
    fn compute_part2() {
        assert_eq!(part2(), 2212);
    }
}
