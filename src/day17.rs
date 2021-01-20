use ndarray::Array3;
use std::collections::HashMap;

const INPUT_PATH: &str = "input/day17";

fn count_active_cells(pocket: &Array3<bool>) -> usize {
    pocket.iter().filter(|&&cell| cell == true).count()
}

fn active_neighbors(pocket: &Array3<bool>, z: usize, x: usize, y: usize) -> usize {
    let mut neighbors = Vec::with_capacity(26);

    for k in z.checked_sub(1).unwrap_or_default()..=z + 1 {
        for i in x.checked_sub(1).unwrap_or_default()..=x + 1 {
            for j in y.checked_sub(1).unwrap_or_default()..=y + 1 {
                if k != z || i != x || j != y {
                    neighbors.push(pocket.get((k, i, j)));
                }
            }
        }
    }

    neighbors
        .iter()
        .filter(|&n| matches!(n, Some(true)))
        .count()
}

fn cubes_after_sixth_cycle(initial_state: &Array3<bool>) -> usize {
    const ADDITIONAL: usize = 6 * 2;
    const DIMS: usize = ADDITIONAL + 1;

    let lines = initial_state.shape()[1];
    let columns = initial_state.shape()[2];
    let floor = f32::floor((DIMS / 2) as f32) as usize;
    let ceil = f32::ceil((DIMS / 2) as f32) as usize;
    let ceil = if floor == ceil { ceil + 1 } else { ceil };

    let mut pocket = Array3::<bool>::default((DIMS, lines + ADDITIONAL, columns + ADDITIONAL));
    let shape = pocket.shape();
    let lines = shape[1];
    let columns = shape[2];

    let mut init = initial_state.iter();
    for z in floor..ceil {
        for x in ADDITIONAL / 2..lines - ADDITIONAL / 2 {
            for y in ADDITIONAL / 2..columns - ADDITIONAL / 2 {
                pocket[[z, x, y]] = *init.next().unwrap();
            }
        }
    }

    let mut changed = HashMap::with_capacity(64);

    for i in 1..=6 {
        for z in floor.checked_sub(i).unwrap_or_default()..ceil + i {
            for x in (ADDITIONAL / 2).checked_sub(i).unwrap_or_default()..lines - ADDITIONAL / 2 + i
            {
                for y in (ADDITIONAL / 2).checked_sub(i).unwrap_or_default()
                    ..columns - ADDITIONAL / 2 + i
                {
                    let neighbors = active_neighbors(&pocket, z, x, y);

                    if pocket[[z, x, y]] {
                        if neighbors < 2 || neighbors > 3 {
                            changed.insert((z, x, y), false);
                        }
                    } else {
                        if neighbors == 3 {
                            changed.insert((z, x, y), true);
                        }
                    }
                }
            }
        }

        for (&(z, x, y), &v) in &changed {
            pocket[[z, x, y]] = v;
        }
        changed.clear();
    }

    count_active_cells(&pocket)
}

pub fn part1() -> usize {
    let mut input = Array3::<bool>::default((1, 8, 8));
    let raw_input = crate::get_input_as_matrix(INPUT_PATH);

    for (x, line) in raw_input.iter().enumerate() {
        for (y, &cell) in line.iter().enumerate() {
            input[[0, x, y]] = if cell == '#' { true } else { false };
        }
    }

    cubes_after_sixth_cycle(&input)
}

pub fn part2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr3;

    #[test]
    fn compute_part1() {
        assert_eq!(part1(), 401);
    }

    #[test]
    fn compute_part2() {
        println!("{}", part2());
        // assert_eq!(part2(), 0);
    }

    #[test]
    fn part1_example() {
        let initial_state = arr3(&[[
            [false, true, false],
            [false, false, true],
            [true, true, true],
        ]]);

        assert_eq!(cubes_after_sixth_cycle(&initial_state), 112);
    }

    #[test]
    fn part2_example() {
        assert_eq!(true, true);
    }
}
