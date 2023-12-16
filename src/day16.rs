use rayon::prelude::*;
use std::collections::HashSet;

use crate::utils::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Slot {
    Empty,
    MirrorForward,
    MirrorBackwards,
    SplitHorizontal,
    SplitVertical,
}

impl From<u8> for Slot {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Slot::Empty,
            b'/' => Slot::MirrorForward,
            b'\\' => Slot::MirrorBackwards,
            b'|' => Slot::SplitVertical,
            b'-' => Slot::SplitHorizontal,
            _ => panic!("Invalid Byte"),
        }
    }
}

type Coord = (isize, isize);

fn beam(
    grid: &Grid<Slot>,
    (mut x, mut y): Coord,
    mut direction: Coord,
    energized: &mut HashSet<Coord>,
    seen: &mut HashSet<(Coord, Coord)>,
) {
    while x >= 0 && x < grid.width() as isize && y >= 0 && y < grid.height() as isize {
        if seen.contains(&((x, y), direction)) {
            return;
        }
        energized.insert((x, y));
        seen.insert(((x, y), direction));
        match grid[(x as usize, y as usize)] {
            Slot::Empty => {}
            Slot::MirrorForward => match direction {
                (x_d, 0) => direction = (0, -x_d),
                (0, y_d) => direction = (-y_d, 0),
                _ => panic!("Invalid direction"),
            },
            Slot::MirrorBackwards => match direction {
                (x_d, 0) => direction = (0, x_d),
                (0, y_d) => direction = (y_d, 0),
                _ => panic!("Invalid direction"),
            },
            Slot::SplitHorizontal => {
                if let (0, _) = direction {
                    beam(grid, (x, y), (-1, 0), energized, seen);
                    direction = (1, 0);
                }
            }
            Slot::SplitVertical => {
                if let (_, 0) = direction {
                    beam(grid, (x, y), (0, -1), energized, seen);
                    direction = (0, 1);
                }
            }
        }
        x += direction.0;
        y += direction.1;
    }
}

pub fn generate(input: &str) -> Grid<Slot> {
    let width = input.lines().next().unwrap().len();
    let data = input
        .lines()
        .flat_map(|l| l.as_bytes().iter().map(|&b| b.into()))
        .collect();

    Grid::new(data, width)
}
pub fn part1(input: &Grid<Slot>) -> usize {
    let mut hs = HashSet::new();
    let mut seen = HashSet::new();
    beam(input, (0, 0), (1, 0), &mut hs, &mut seen);
    hs.len()
}

pub fn part2(input: &Grid<Slot>) -> usize {
    let x_max = input.width() as isize - 1;
    let y_max = input.height() as isize - 1;
    let it: Vec<_> = (1..x_max)
        .flat_map(|x| [((x, 0isize), (0, 1)), ((x, y_max), (0, -1))])
        .chain((1..y_max).flat_map(|y| [((y, 0isize), (0, 1)), ((y, x_max), (0, -1))]))
        .chain([
            ((0, 0), (1, 0)),
            ((0, 0), (0, 1)),
            ((x_max, 0), (1, 0)),
            ((x_max, 0), (0, 1)),
            ((0, y_max), (1, 0)),
            ((0, y_max), (0, -1)),
            ((y_max, y_max), (-1, 0)),
            ((y_max, y_max), (0, -1)),
        ])
        .collect();

    it.into_par_iter()
        .map(|(start, direction)| {
            let mut hs = HashSet::new();
            let mut seen = HashSet::new();
            beam(input, start, direction, &mut hs, &mut seen);
            hs.len()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!(46, part1(&generate(input)));
    }
    #[test]
    fn test_p2() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!(51, part2(&generate(input)));
    }
}
