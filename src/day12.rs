use std::convert::TryInto;
use std::ops::Add;

const INPUT_PATH: &str = "input/day12";

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Add<i32> for Direction {
    type Output = Direction;

    fn add(self, degrees: i32) -> Self {
        use Direction::*;
        let directions = [North, East, South, West];
        let curr_direction = directions
            .iter()
            .enumerate()
            .find(|&(_, &elem)| elem == self)
            .unwrap().0;

        match degrees {
            // Left
            -90 => directions[(curr_direction.wrapping_sub(1)) % directions.len()],
            -180 => directions[(curr_direction.wrapping_sub(2)) % directions.len()],
            -270 => directions[(curr_direction.wrapping_sub(3)) % directions.len()],
            // Right
            90 => directions[(curr_direction + 1) % directions.len()],
            180 => directions[(curr_direction + 2) % directions.len()],
            270 => directions[(curr_direction + 3) % directions.len()],
            // Return the same direction otherwise
            _ => self,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Coordinates {
    direction: Direction,
    north: i32,
    south: i32,
    east: i32,
    west: i32,
}

impl Coordinates {
    fn turn(&mut self, left_right: char, mut degrees: i32) {
        if left_right == 'L' {
            degrees = -degrees;
        }
        self.direction = self.direction + degrees;
    }

    fn forward(&mut self, amount: i32) {
        match self.direction {
            Direction::North => self.north += amount,
            Direction::South => self.south += amount,
            Direction::East => self.east += amount,
            Direction::West => self.west += amount,
        }
    }
}

fn manhattan_distance(input: &[&str]) -> u32 {
    let mut coordinates = Coordinates {
        north: 0,
        south: 0,
        east: 0,
        west: 0,
        direction: Direction::East,
    };

    for &instruction in input {
        let direction = instruction.chars().nth(0).unwrap();
        let amount: i32 = instruction[1..].parse().unwrap_or_default();

        match direction {
            'N' => coordinates.north += amount,
            'S' => coordinates.south += amount,
            'E' => coordinates.east += amount,
            'W' => coordinates.west += amount,
            dir @ 'L' => coordinates.turn(dir, amount),
            dir @ 'R' => coordinates.turn(dir, amount),
            'F' => coordinates.forward(amount),
            _ => panic!(),
        }
    }

    ((coordinates.north - coordinates.south).abs() + (coordinates.east - coordinates.west).abs())
        .try_into()
        .unwrap()
}

pub fn part1() -> u32 {
    let input = crate::get_input_as_vec::<String>(INPUT_PATH);
    let input: Vec<&str> = input.iter().map(|line| line.as_str()).collect();
    manhattan_distance(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_part1() {
        assert_eq!(part1(), 1838);
    }

    #[test]
    fn test_example() {
        let mut example = Vec::new();
        example.push("F10");
        example.push("N3");
        example.push("F7");
        example.push("R90");
        example.push("F11");

        assert_eq!(manhattan_distance(&example), 25);
    }
}
