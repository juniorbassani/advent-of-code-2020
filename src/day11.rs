const INPUT_PATH: &str = "input/day11";

fn get_occupied_adjacent_seats(input: &Vec<Vec<char>>, line: usize, column: usize) -> usize {
    let mut seats = Vec::with_capacity(8);

    if column > 0 {
        seats.push(input.get(line).and_then(|line| line.get(column - 1)));
        seats.push(input.get(line + 1).and_then(|line| line.get(column - 1)));

        if line > 0 {
            seats.push(input.get(line - 1).and_then(|line| line.get(column - 1)));
        }
    }
    if line > 0 {
        seats.push(input.get(line - 1).and_then(|line| line.get(column)));
        seats.push(input.get(line - 1).and_then(|line| line.get(column + 1)));
    }
    seats.push(input.get(line).and_then(|line| line.get(column + 1)));
    seats.push(input.get(line + 1).and_then(|line| line.get(column)));
    seats.push(input.get(line + 1).and_then(|line| line.get(column + 1)));

    seats
        .iter()
        .filter(|&seat| seat.map_or(false, |&seat| seat == '#'))
        .count()
}

fn get_occupied_visible_seats(map: &Vec<Vec<char>>, line: usize, column: usize) -> usize {
    let mut seats = Vec::with_capacity(8);

    let up = map
        .iter()
        .enumerate()
        .filter(|&(index, _)| index < line)
        .rev()
        .skip_while(|&(_, line)| line[column] == '.')
        .next()
        .map(|line| line.1[column]);
    seats.push(up);

    let down = map
        .iter()
        .enumerate()
        .filter(|&(index, _)| index > line)
        .skip_while(|&(_, line)| line[column] == '.')
        .next()
        .map(|line| line.1[column]);
    seats.push(down);

    let right = map[line]
        .iter()
        .enumerate()
        .filter(|&(index, _)| index > column)
        .skip_while(|&(_, &seat)| seat == '.')
        .next()
        .map(|line| *line.1);
    seats.push(right);

    let left = map[line]
        .iter()
        .enumerate()
        .filter(|&(index, _)| index < column)
        .rev()
        .skip_while(|&(_, &seat)| seat == '.')
        .next()
        .map(|line| *line.1);
    seats.push(left);

    let mut c = column;
    let up_left = map
        .iter()
        .enumerate()
        .filter(|&(index, _)| index < line)
        .rev()
        .skip_while(|&(_, seat)| {
            if let Some(i) = c.checked_sub(1) {
                c = i;
                seat[c] == '.'
            } else {
                true
            }
        })
        .next()
        .map(|line| line.1[c]);
    seats.push(up_left);

    let mut c = column;
    let up_right = map
        .iter()
        .enumerate()
        .filter(|&(index, _)| index < line)
        .rev()
        .skip_while(|&(_, seat)| {
            c += 1;
            c < seat.len() && seat[c] == '.'
        })
        .next()
        .map(|line| if c < line.1.len() { line.1[c] } else { ' ' });
    seats.push(up_right);

    let mut c = column;
    let down_left = map
        .iter()
        .enumerate()
        .filter(|&(index, _)| index > line)
        .skip_while(|&(_, seat)| {
            if let Some(i) = c.checked_sub(1) {
                c = i;
                seat[c] == '.'
            } else {
                true
            }
        })
        .next()
        .map(|line| line.1[c]);
    seats.push(down_left);

    let mut c = column;
    let down_right = map
        .iter()
        .enumerate()
        .filter(|&(index, _)| index > line)
        .skip_while(|&(_, seat)| {
            c += 1;
            c < seat.len() && seat[c] == '.'
        })
        .next()
        .map(|line| if c < line.1.len() { line.1[c] } else { ' ' });
    seats.push(down_right);

    // TODO: Change it to filter_map()
    seats
        .iter()
        .map(|seat| seat.unwrap_or(' '))
        .filter(|&seat| seat == '#')
        .count()
}

fn count_occupied_seats(input: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;

    for line in input.iter() {
        count += line.iter().filter(|&&seat| seat == '#').count();
    }

    count as u32
}

pub fn part1() -> u32 {
    let mut input = crate::get_input_as_matrix(INPUT_PATH);
    let mut other = input.clone();
    let mut changed = true;
    let lines = input.len();
    let columns = input[0].len();

    while changed {
        changed = false;

        for i in 0..lines {
            for j in 0..columns {
                let seats = get_occupied_adjacent_seats(&input, i, j);

                if input[i][j] == 'L' && seats == 0 {
                    other[i][j] = '#';
                    changed = true;
                } else if input[i][j] == '#' && seats >= 4 {
                    other[i][j] = 'L';
                    changed = true;
                }
            }
        }

        input = other.clone();
    }

    count_occupied_seats(&input)
}

pub fn part2() -> u32 {
    let mut input = crate::get_input_as_matrix(INPUT_PATH);
    let mut other = input.clone();
    let mut changed = true;
    let lines = input.len();
    let columns = input[0].len();

    while changed {
        changed = false;

        for i in 0..lines {
            for j in 0..columns {
                let seats = get_occupied_visible_seats(&input, i, j);

                if input[i][j] == 'L' && seats == 0 {
                    other[i][j] = '#';
                    changed = true;
                } else if input[i][j] == '#' && seats >= 5 {
                    other[i][j] = 'L';
                    changed = true;
                }
            }
        }

        input = other.clone();
    }

    count_occupied_seats(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_part1() {
        assert_eq!(part1(), 2261);
    }

    #[test]
    fn compute_part2() {
        assert_eq!(part2(), 2039);
    }

    #[test]
    fn test_second_example() {
        let test1 = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '#', 'L', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '#', '.', '.', '.', '.', '.'],
        ];
        let test2 = vec![
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', 'L', '.', 'L', '.', '#', '.', '#', '.', '#', '.', '#', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
        ];
        let test3 = vec![
            vec!['.', '#', '#', '.', '#', '#', '.'],
            vec!['#', '.', '#', '.', '#', '.', '#'],
            vec!['#', '#', '.', '.', '.', '#', '#'],
            vec!['.', '.', '.', 'L', '.', '.', '.'],
            vec!['#', '#', '.', '.', '.', '#', '#'],
            vec!['#', '.', '#', '.', '#', '.', '#'],
            vec!['.', '#', '#', '.', '#', '#', '.'],
        ];
        let test4 = vec![
            vec!['#', '.', '#', '#', '.', '#', '#', '.', '#', '#'],
            vec!['#', '#', '#', '#', '#', '#', '#', '.', '#', '#'],
            vec!['#', '.', '#', '.', '#', '.', '.', '#', '.', '.'],
            vec!['#', '#', '#', '#', '.', '#', '#', '.', '#', '#'],
            vec!['#', '.', '#', '#', '.', '#', '#', '.', '#', '#'],
            vec!['#', '.', '#', '#', '#', '#', '#', '.', '#', '#'],
            vec!['.', '.', '#', '.', '#', '.', '.', '.', '.', '.'],
            vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
            vec!['#', '.', '#', '#', '#', '#', '#', '#', '.', '#'],
        ];
        let example = vec![
            vec!['L', '.', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
            vec!['L', 'L', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
            vec!['L', '.', 'L', '.', 'L', '.', '.', 'L', '.', '.'],
            vec!['L', 'L', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
            vec!['L', '.', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
            vec!['L', '.', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
            vec!['.', '.', 'L', '.', 'L', '.', '.', '.', '.', '.'],
            vec!['L', 'L', 'L', 'L', 'L', 'L', 'L', 'L', 'L', 'L'],
            vec!['L', '.', 'L', 'L', 'L', 'L', 'L', 'L', '.', 'L'],
        ];

        // get_occupied_visible_seats(&test1, 4, 3);
        // get_occupied_visible_seats(&test2, 1, 1);
        // get_occupied_visible_seats(&test3, 3, 3);
        assert_eq!(get_occupied_visible_seats(&test1, 4, 3), 8);
        assert_eq!(get_occupied_visible_seats(&test1, 0, 0), 2);
        assert_eq!(get_occupied_visible_seats(&test2, 1, 1), 0);
        assert_eq!(get_occupied_visible_seats(&test3, 3, 3), 0);
        assert_eq!(get_occupied_visible_seats(&test4, 1, 0), 4);
        // assert_eq!(part2(example), 26);
    }
}
