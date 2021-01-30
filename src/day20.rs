use ndarray::Array2;
use std::fmt::{self, Display, Formatter};

const INPUT_PATH: &str = "input/day20";

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Pixel {
    On,
    Off,
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Pixel::On => write!(f, "#"),
            Pixel::Off => write!(f, "."),
        }
    }
}

impl From<char> for Pixel {
    fn from(elem: char) -> Self {
        match elem {
            '#' => Self::On,
            '.' => Self::Off,
            _ => Self::default(),
        }
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self::Off
    }
}

#[derive(Default)]
struct Tile {
    id: u32,
    image: Array2<Pixel>,

    // Neighbors' ids
    up: Option<u32>,
    down: Option<u32>,
    left: Option<u32>,
    right: Option<u32>,
}

impl Tile {
    fn neighbors(&self) -> u32 {
        let mut counter = 0;

        if self.up.is_some() {
            counter += 1;
        }
        if self.down.is_some() {
            counter += 1;
        }
        if self.left.is_some() {
            counter += 1;
        }
        if self.right.is_some() {
            counter += 1;
        }

        counter
    }

    fn corner(&self) -> bool {
        self.neighbors() == 2
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Tile {}

fn multiply_corners(tiles: &[Tile]) -> usize {
    tiles
        .iter()
        .filter(|tile| tile.corner())
        .map(|tile| tile.id as usize)
        .product()
}

fn edges_match(this: &mut Tile, other: &Tile) {
    let image = &this.image;
    let (lines, columns) = image.dim();
    let up = image.row(0);
    let down = image.row(lines - 1);
    let left = image.column(0);
    let right = image.column(columns - 1);

    let this_iter = vec![
        (up, &mut this.up),
        (down, &mut this.down),
        (left, &mut this.left),
        (right, &mut this.right),
    ]
    .into_iter();

    let other_image = &other.image;
    let (lines, columns) = other_image.dim();
    let up = other_image.row(0);
    let down = other_image.row(lines - 1);
    let left = other_image.column(0);
    let right = other_image.column(columns - 1);

    let other_iter = vec![up, down, left, right];

    for (border, dir) in this_iter {
        for o in &other_iter {
            if border == *o || border.iter().eq(o.iter().rev()) {
                *dir = Some(other.id);
                break;
            }
        }
    }
}

fn rearrange_tiles(tiles: &mut [Tile]) {
    for i in 0..tiles.len() {
        let tile = &mut tiles[i] as *mut Tile;

        for j in 0..tiles.len() {
            let other = &tiles[j];

            unsafe {
                if *tile != *other {
                    edges_match(&mut *tile, &other);
                }
            }
        }
    }
}

pub fn part1() -> usize {
    let input = crate::get_input_as_string(INPUT_PATH);
    let input = input.trim_end();
    let mut tiles = Vec::with_capacity(100);

    for tile in input.split("\n\n") {
        let id: u32 = tile
            .lines()
            .nth(0)
            .unwrap()
            .trim_start_matches("Tile ")
            .trim_end_matches(":")
            .parse()
            .unwrap();

        let mut image = Array2::default((10, 10));

        for (i, line) in tile.lines().skip(1).enumerate() {
            line.chars().map(Pixel::from).fold(0, |j, pixel| {
                image[[i, j]] = pixel;
                j + 1
            });
        }

        tiles.push(Tile {
            id,
            image,
            ..Default::default()
        });
    }

    rearrange_tiles(&mut tiles);
    multiply_corners(&tiles)
}

pub fn part2() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_part1() {
        assert_eq!(part1(), 27798062994017);
    }

    #[test]
    fn compute_part2() {
        println!("{}", part2());
        // assert_eq!(part2(), 0);
    }

    #[test]
    fn part1_example() {
        use super::Pixel::*;
        let mut tiles = Vec::new();

        tiles.push(Tile {
            id: 2311,
            image: ndarray::arr2(&[
                [Off, Off, On, On, Off, On, Off, Off, On, Off],
                [On, On, Off, Off, On, Off, Off, Off, Off, Off],
                [On, Off, Off, Off, On, On, Off, Off, On, Off],
                [On, On, On, On, Off, On, Off, Off, Off, On],
                [On, On, Off, On, On, Off, On, On, On, Off],
                [On, On, Off, Off, Off, On, Off, On, On, On],
                [Off, On, Off, On, Off, On, Off, Off, On, On],
                [Off, Off, On, Off, Off, Off, Off, On, Off, Off],
                [On, On, On, Off, Off, Off, On, Off, On, Off],
                [Off, Off, On, On, On, Off, Off, On, On, On],
            ]),
            up: None,
            down: None,
            left: None,
            right: None,
        });

        tiles.push(Tile {
            id: 1951,
            image: ndarray::arr2(&[
                [On, Off, On, On, Off, Off, Off, On, On, Off],
                [On, Off, On, On, On, On, Off, Off, Off, On],
                [Off, Off, Off, Off, Off, On, Off, Off, On, On],
                [On, Off, Off, Off, On, On, On, On, On, On],
                [Off, On, On, Off, On, Off, Off, Off, Off, On],
                [Off, On, On, On, Off, On, On, On, On, On],
                [On, On, On, Off, On, On, Off, On, On, Off],
                [Off, On, On, On, Off, Off, Off, Off, On, Off],
                [Off, Off, On, Off, On, Off, Off, On, Off, On],
                [On, Off, Off, Off, On, On, Off, On, Off, Off],
            ]),
            up: None,
            down: None,
            left: None,
            right: None,
        });

        tiles.push(Tile {
            id: 1171,
            image: ndarray::arr2(&[
                [On, On, On, On, Off, Off, Off, On, On, Off],
                [On, Off, Off, On, On, Off, On, Off, Off, On],
                [On, On, Off, On, Off, Off, On, Off, On, Off],
                [Off, On, On, On, Off, On, On, On, On, Off],
                [Off, Off, On, On, On, Off, On, On, On, On],
                [Off, On, On, Off, Off, Off, Off, On, On, Off],
                [Off, On, Off, Off, Off, On, On, On, On, Off],
                [On, Off, On, On, Off, On, On, On, On, Off],
                [On, On, On, On, Off, Off, On, Off, Off, Off],
                [Off, Off, Off, Off, Off, On, On, Off, Off, Off],
            ]),
            up: None,
            down: None,
            left: None,
            right: None,
        });

        tiles.push(Tile {
            id: 1427,
            image: ndarray::arr2(&[
                [On, On, On, Off, On, On, Off, On, Off, Off],
                [Off, On, Off, Off, On, Off, On, On, Off, Off],
                [Off, On, Off, On, On, Off, On, Off, Off, On],
                [On, Off, On, Off, On, Off, On, On, Off, On],
                [Off, Off, Off, Off, On, Off, Off, Off, On, On],
                [Off, Off, Off, On, On, Off, Off, On, On, Off],
                [Off, Off, Off, On, Off, On, On, On, On, On],
                [Off, On, Off, On, On, On, On, Off, On, Off],
                [Off, Off, On, Off, Off, On, On, On, Off, On],
                [Off, Off, On, On, Off, On, Off, Off, On, Off],
            ]),
            up: None,
            down: None,
            left: None,
            right: None,
        });

        tiles.push(Tile {
            id: 1489,
            image: ndarray::arr2(&[
                [On, On, Off, On, Off, On, Off, Off, Off, Off],
                [Off, Off, On, On, Off, Off, Off, On, Off, Off],
                [Off, On, On, Off, Off, On, On, Off, Off, Off],
                [Off, Off, On, Off, Off, Off, On, Off, Off, Off],
                [On, On, On, On, On, Off, Off, Off, On, Off],
                [On, Off, Off, On, Off, On, Off, On, Off, On],
                [Off, Off, Off, On, Off, On, Off, On, Off, Off],
                [On, On, Off, On, Off, Off, Off, On, On, Off],
                [Off, Off, On, On, Off, On, On, Off, On, On],
                [On, On, On, Off, On, On, Off, On, Off, Off],
            ]),
            up: None,
            down: None,
            left: None,
            right: None,
        });

        tiles.push(Tile {
            id: 2473,
            image: ndarray::arr2(&[
                [On, Off, Off, Off, Off, On, On, On, On, Off],
                [On, Off, Off, On, Off, On, On, Off, Off, Off],
                [On, Off, On, On, Off, Off, On, Off, Off, Off],
                [On, On, On, On, On, On, Off, On, Off, On],
                [Off, On, Off, Off, Off, On, Off, On, Off, On],
                [Off, On, On, On, On, On, On, On, On, On],
                [Off, On, On, On, Off, On, Off, Off, On, Off],
                [On, On, On, On, On, On, On, On, Off, On],
                [On, On, Off, Off, Off, On, On, Off, On, Off],
                [Off, Off, On, On, On, Off, On, Off, On, Off],
            ]),
            up: None,
            down: None,
            left: None,
            right: None,
        });

        tiles.push(Tile {
            id: 2971,
            image: ndarray::arr2(&[
                [Off, Off, On, Off, On, Off, Off, Off, Off, On],
                [On, Off, Off, Off, On, On, On, Off, Off, Off],
                [On, Off, On, Off, On, On, On, Off, Off, Off],
                [On, On, Off, On, On, Off, Off, On, Off, Off],
                [Off, On, On, On, On, On, Off, Off, On, On],
                [Off, On, Off, Off, On, On, On, On, Off, On],
                [On, Off, Off, On, Off, On, Off, Off, On, Off],
                [Off, Off, On, On, On, On, Off, On, On, On],
                [Off, Off, On, Off, On, Off, On, On, On, Off],
                [Off, Off, Off, On, Off, On, Off, On, Off, On],
            ]),
            up: None,
            down: None,
            left: None,
            right: None,
        });

        tiles.push(Tile {
            id: 2729,
            image: ndarray::arr2(&[
                [Off, Off, Off, On, Off, On, Off, On, Off, On],
                [On, On, On, On, Off, On, Off, Off, Off, Off],
                [Off, Off, On, Off, On, Off, Off, Off, Off, Off],
                [Off, Off, Off, Off, On, Off, Off, On, Off, On],
                [Off, On, On, Off, Off, On, On, Off, On, Off],
                [Off, On, Off, On, On, On, On, Off, Off, Off],
                [On, On, On, On, Off, On, Off, On, Off, Off],
                [On, On, Off, On, On, On, On, Off, Off, Off],
                [On, On, Off, Off, On, Off, On, On, Off, Off],
                [On, Off, On, On, Off, Off, Off, On, On, Off],
            ]),
            up: None,
            down: None,
            left: None,
            right: None,
        });

        tiles.push(Tile {
            id: 3079,
            image: ndarray::arr2(&[
                [On, Off, On, Off, On, On, On, On, On, Off],
                [Off, On, Off, Off, On, On, On, On, On, On],
                [Off, Off, On, Off, Off, Off, Off, Off, Off, Off],
                [On, On, On, On, On, On, Off, Off, Off, Off],
                [On, On, On, On, Off, On, Off, Off, On, Off],
                [Off, On, Off, Off, Off, On, Off, On, On, Off],
                [On, Off, On, On, On, On, On, Off, On, On],
                [Off, Off, On, Off, On, On, On, Off, Off, Off],
                [Off, Off, On, Off, Off, Off, Off, Off, Off, Off],
                [Off, Off, On, Off, On, On, On, Off, Off, Off],
            ]),
            up: None,
            down: None,
            left: None,
            right: None,
        });

        rearrange_tiles(&mut tiles);

        assert_eq!(multiply_corners(&tiles), 20899048083289);
    }

    #[test]
    fn part2_example() {
        assert_eq!(true, true);
    }
}
