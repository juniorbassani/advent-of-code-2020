use ndarray::{Array2, Array3, Array4};
use std::collections::HashMap;

const INPUT_PATH: &str = "input/day17";

fn count_active_cells<T>(pocket: T) -> usize
where
    T: IntoIterator,
    T::Item: PartialEq<bool>,
{
    pocket.into_iter().filter(|cell| *cell == true).count()
}

fn count_active_neighbors<'a, T>(neighbors: T) -> usize
where
    T: Iterator<Item = Option<&'a bool>>,
{
    neighbors.filter(|n| matches!(n, Some(true))).count()
}

fn active_neighbors_3d(pocket: &Array3<bool>, x: usize, y: usize, z: usize) -> usize {
    let mut neighbors = Vec::with_capacity(26);

    for i in x.checked_sub(1).unwrap_or_default()..=x + 1 {
        for j in y.checked_sub(1).unwrap_or_default()..=y + 1 {
            for k in z.checked_sub(1).unwrap_or_default()..=z + 1 {
                if k != z || i != x || j != y {
                    neighbors.push(pocket.get((i, j, k)));
                }
            }
        }
    }

    count_active_neighbors(neighbors.into_iter())
}

fn active_neighbors_4d(pocket: &Array4<bool>, x: usize, y: usize, z: usize, w: usize) -> usize {
    let mut neighbors = Vec::with_capacity(80);

    for i in x.checked_sub(1).unwrap_or_default()..=x + 1 {
        for j in y.checked_sub(1).unwrap_or_default()..=y + 1 {
            for k in z.checked_sub(1).unwrap_or_default()..=z + 1 {
                for h in w.checked_sub(1).unwrap_or_default()..=w + 1 {
                    if k != z || i != x || j != y || h != w {
                        neighbors.push(pocket.get((i, j, k, h)));
                    }
                }
            }
        }
    }

    count_active_neighbors(neighbors.into_iter())
}

fn cubes_after_sixth_cycle(initial_state: &Array2<bool>) -> usize {
    const ADDITIONAL: usize = 6 * 2;
    const DIMS: usize = ADDITIONAL + 1;

    let lines = initial_state.shape()[0];
    let columns = initial_state.shape()[1];
    let floor = f32::floor((DIMS / 2) as f32) as usize;
    let ceil = f32::ceil((DIMS / 2) as f32) as usize;
    let ceil = if floor == ceil { ceil + 1 } else { ceil };

    let mut pocket = Array3::default((lines + ADDITIONAL, columns + ADDITIONAL, DIMS));
    let shape = pocket.shape();
    let lines = shape[0];
    let columns = shape[1];

    let mut init = initial_state.iter();
    for x in ADDITIONAL / 2..lines - ADDITIONAL / 2 {
        for y in ADDITIONAL / 2..columns - ADDITIONAL / 2 {
            for z in floor..ceil {
                pocket[[x, y, z]] = *init.next().unwrap();
            }
        }
    }

    let mut changed = HashMap::with_capacity(64);

    for i in 1..=6 {
        for x in (ADDITIONAL / 2).checked_sub(i).unwrap_or_default()..lines - ADDITIONAL / 2 + i {
            for y in
                (ADDITIONAL / 2).checked_sub(i).unwrap_or_default()..columns - ADDITIONAL / 2 + i
            {
                for z in floor.checked_sub(i).unwrap_or_default()..ceil + i {
                    let neighbors = active_neighbors_3d(&pocket, x, y, z);

                    if pocket[[x, y, z]] {
                        if neighbors < 2 || neighbors > 3 {
                            changed.insert((x, y, z), false);
                        }
                    } else {
                        if neighbors == 3 {
                            changed.insert((x, y, z), true);
                        }
                    }
                }
            }
        }

        for (&(x, y, z), &v) in &changed {
            pocket[[x, y, z]] = v;
        }
        changed.clear();
    }

    count_active_cells(pocket.into_iter().copied())
}

fn hypercubes_after_sixth_cycle(initial_state: &Array2<bool>) -> usize {
    const ADDITIONAL: usize = 6 * 2;
    const DIMS: usize = ADDITIONAL + 1;

    let lines = initial_state.shape()[0];
    let columns = initial_state.shape()[1];
    let floor = f32::floor((DIMS / 2) as f32) as usize;
    let ceil = f32::ceil((DIMS / 2) as f32) as usize;
    let ceil = if floor == ceil { ceil + 1 } else { ceil };

    let mut pocket = Array4::default((lines + ADDITIONAL, columns + ADDITIONAL, DIMS, DIMS));
    let shape = pocket.shape();
    let lines = shape[0];
    let columns = shape[1];

    let mut init = initial_state.iter();
    for x in ADDITIONAL / 2..lines - ADDITIONAL / 2 {
        for y in ADDITIONAL / 2..columns - ADDITIONAL / 2 {
            for z in floor..ceil {
                for w in floor..ceil {
                    pocket[[x, y, z, w]] = *init.next().unwrap();
                }
            }
        }
    }

    let mut changed = HashMap::with_capacity(128);

    for i in 1..=6 {
        for x in (ADDITIONAL / 2).checked_sub(i).unwrap_or_default()..lines - ADDITIONAL / 2 + i {
            for y in
                (ADDITIONAL / 2).checked_sub(i).unwrap_or_default()..columns - ADDITIONAL / 2 + i
            {
                for z in floor.checked_sub(i).unwrap_or_default()..ceil + i {
                    for w in floor.checked_sub(i).unwrap_or_default()..ceil + i {
                        let neighbors = active_neighbors_4d(&pocket, x, y, z, w);

                        if pocket[[x, y, z, w]] {
                            if neighbors < 2 || neighbors > 3 {
                                changed.insert((x, y, z, w), false);
                            }
                        } else {
                            if neighbors == 3 {
                                changed.insert((x, y, z, w), true);
                            }
                        }
                    }
                }
            }
        }

        for (&(x, y, z, w), &v) in &changed {
            pocket[[x, y, z, w]] = v;
        }
        changed.clear();
    }

    count_active_cells(pocket.into_iter().copied())
}

fn parse_input() -> Array2<bool> {
    let mut input = Array2::default((8, 8));
    let raw_input = crate::get_input_as_matrix(INPUT_PATH);

    for (x, line) in raw_input.iter().enumerate() {
        for (y, &cell) in line.iter().enumerate() {
            input[[x, y]] = if cell == '#' { true } else { false };
        }
    }

    input
}

pub fn part1() -> usize {
    let input = parse_input();
    cubes_after_sixth_cycle(&input)
}

pub fn part2() -> usize {
    let input = parse_input();
    hypercubes_after_sixth_cycle(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_part1() {
        assert_eq!(part1(), 401);
    }

    #[test]
    fn compute_part2() {
        assert_eq!(part2(), 2224);
    }

    #[test]
    fn part1_example() {
        let initial_state = ndarray::arr2(&[
            [false, true, false],
            [false, false, true],
            [true, true, true],
        ]);

        assert_eq!(cubes_after_sixth_cycle(&initial_state), 112);
    }

    #[test]
    fn part2_example() {
        let initial_state = ndarray::arr2(&[
            [false, true, false],
            [false, false, true],
            [true, true, true],
        ]);

        assert_eq!(hypercubes_after_sixth_cycle(&initial_state), 848);
    }
}
