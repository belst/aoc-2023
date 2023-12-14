use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
    str::FromStr,
};

pub fn part1(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut last_pos = vec![0; width];
    let mut total = 0;
    for (i, l) in input.lines().enumerate() {
        let l = l.as_bytes();
        for x in 0..width {
            match l[x] {
                b'O' => {
                    total += height - last_pos[x];
                    last_pos[x] += 1;
                }
                b'#' => {
                    last_pos[x] = i + 1;
                }
                _ => {}
            }
        }
    }
    total
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Rock {
    Cube,
    Rounded,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    West,
    South,
    East,
}

// Another one
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Grid {
    width: usize,
    height: usize,
    data: Vec<Rock>,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();
        let data = s
            .lines()
            .flat_map(|l| {
                l.as_bytes().iter().map(|b| match b {
                    b'#' => Rock::Cube,
                    b'O' => Rock::Rounded,
                    _ => Rock::None,
                })
            })
            .collect();

        Ok(Grid {
            width,
            height,
            data,
        })
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Rock;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        assert!(x < self.width && y < self.height);
        &self.data[x + y * self.width]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        assert!(x < self.width && y < self.height);
        &mut self.data[x + y * self.width]
    }
}

impl Grid {
    fn tilt_vertical(mut self, direction: Direction) -> Grid {
        assert!(matches!(direction, Direction::North | Direction::South));
        if direction == Direction::North {
            let mut last_pos = vec![0; self.width];
            for y in 0..self.height {
                for x in 0..self.width {
                    match self[(x, y)] {
                        Rock::Cube => last_pos[x] = y + 1,
                        Rock::Rounded => {
                            if y != last_pos[x] {
                                self[(x, last_pos[x])] = Rock::Rounded;
                                self[(x, y)] = Rock::None;
                            }
                            last_pos[x] += 1;
                        }
                        Rock::None => {}
                    }
                }
            }
        } else {
            let mut last_pos = vec![self.height - 1; self.width];
            for y in (0..self.height).rev() {
                for x in 0..self.width {
                    match self[(x, y)] {
                        Rock::Cube => last_pos[x] = y.saturating_sub(1),
                        Rock::Rounded => {
                            if y != last_pos[x] {
                                self[(x, last_pos[x])] = Rock::Rounded;
                                self[(x, y)] = Rock::None;
                            }
                            last_pos[x] -= 1;
                        }
                        Rock::None => {}
                    }
                }
            }
        }
        self
    }
    fn tilt_horizontal(mut self, direction: Direction) -> Grid {
        assert!(matches!(direction, Direction::West | Direction::East));
        if direction == Direction::West {
            let mut last_pos = vec![0; self.height];
            for x in 0..self.width {
                for y in 0..self.height {
                    match self[(x, y)] {
                        Rock::Cube => last_pos[y] = x + 1,
                        Rock::Rounded => {
                            if x != last_pos[y] {
                                self[(last_pos[y], y)] = Rock::Rounded;
                                self[(x, y)] = Rock::None;
                            }
                            last_pos[y] += 1;
                        }
                        Rock::None => {}
                    }
                }
            }
        } else {
            let mut last_pos = vec![self.width - 1; self.height];
            for x in (0..self.width).rev() {
                for y in 0..self.height {
                    match self[(x, y)] {
                        Rock::Cube => last_pos[y] = x.saturating_sub(1),
                        Rock::Rounded => {
                            if x != last_pos[y] {
                                self[(last_pos[y], y)] = Rock::Rounded;
                                self[(x, y)] = Rock::None;
                            }
                            last_pos[y] -= 1;
                        }
                        Rock::None => {}
                    }
                }
            }
        }
        self
    }
    fn tilt(self, direction: Direction) -> Grid {
        match direction {
            Direction::North | Direction::South => self.tilt_vertical(direction),
            _ => self.tilt_horizontal(direction),
        }
    }

    fn cycle(self) -> Grid {
        [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ]
        .into_iter()
        .fold(self, |a, d| a.tilt(d))
    }

    fn load(&self) -> usize {
        let mut total = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self[(x, y)] == Rock::Rounded {
                    total += self.height - y;
                }
            }
        }
        total
    }
}

pub fn part2(input: &str) -> usize {
    let mut seen = HashMap::new();
    let mut index_map = Vec::new();
    let mut index = 0usize;
    let mut grid: Grid = input.parse().unwrap();
    let iterations = 1000000000;
    while index < iterations {
        if let Some(&i) = seen.get(&grid) {
            let diff = index - i;
            if diff == 1 {
                break;
            }
            let offset = iterations - i;
            grid = index_map.swap_remove(i + offset % diff);
            break;
        }
        seen.insert(grid.clone(), index);
        index_map.push(grid.clone());
        grid = grid.cycle();
        index += 1;
    }
    grid.load()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_p1() {
        assert_eq!(136, part1(INPUT));
    }

    #[test]
    fn test_tilt_north() {
        let grid: Grid = INPUT.parse().unwrap();
        let tilted: Grid = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
            .parse()
            .unwrap();

        assert_eq!(tilted, grid.tilt(Direction::North));
    }

    #[test]
    fn test_grid_cycle() {
        let grid: Grid = INPUT.parse().unwrap();
        let cycled: Grid = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
            .parse()
            .unwrap();
        assert_eq!(cycled, grid.cycle());
    }

    #[test]
    fn test_p2() {
        assert_eq!(64, part2(INPUT));
    }
}
