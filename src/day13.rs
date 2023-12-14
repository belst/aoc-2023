use std::{
    cell::RefCell,
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone)]
struct Matrix<T> {
    values: Vec<T>,
}

impl<T> Matrix<T> {
    fn size(d: usize) -> usize {
        ((d + 1) as f64 * (d as f64 / 2.0)) as usize
    }
}
impl<T: Default + Clone> Matrix<T> {
    fn new(dimension: usize) -> Self {
        Self {
            values: vec![T::default(); Self::size(dimension)],
        }
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let (x, y) = (x.min(y), y.max(x));
        &self.values[Self::size(y - 1) + x]
    }
}
impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let (x, y) = (x.min(y), y.max(x));
        &mut self.values[Self::size(y - 1) + x]
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
    cache_rows: RefCell<Matrix<Option<u8>>>,
    cache_columns: RefCell<Matrix<Option<u8>>>,
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self[(x, y)])?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: PartialEq> Grid<T> {
    fn check_between_rows(&self, y1: usize, y2: usize, max_errors: u8) -> u8 {
        let mut errors = 0;
        for o in 1..(y2 - y1) / 2 + 1 {
            errors += self.rows_equal(y1 + o, y2 - o);
            if errors > max_errors {
                return errors;
            }
        }
        errors
    }
    fn check_between_columns(&self, x1: usize, x2: usize, max_errors: u8) -> u8 {
        let mut errors = 0;
        for o in 1..(x2 - x1) / 2 + 1 {
            errors += self.columns_equal(x1 + o, x2 - o);
            if errors > max_errors {
                return errors;
            }
        }
        errors
    }
    fn rows_equal(&self, y1: usize, y2: usize) -> u8 {
        if y1 < self.height && y1 == y2 {
            return 0;
        }
        if let Some(r) = self.cache_rows.borrow()[(y1, y2)] {
            return r;
        }

        let mut differences = 0;
        for x in 0..self.width {
            if self[(x, y1)] != self[(x, y2)] {
                differences += 1;
                if differences > 1 {
                    break;
                }
            }
        }
        self.cache_rows.borrow_mut()[(y1, y2)] = Some(differences);
        differences
    }
    fn columns_equal(&self, x1: usize, x2: usize) -> u8 {
        if x1 < self.width && x1 == x2 {
            return 0;
        }
        if let Some(r) = self.cache_columns.borrow()[(x1, x2)] {
            return r;
        }

        let mut differences = 0;
        for y in 0..self.height {
            if self[(x1, y)] != self[(x2, y)] {
                differences += 1;
                if differences > 1 {
                    break;
                }
            }
        }
        self.cache_columns.borrow_mut()[(x1, x2)] = Some(differences);
        differences
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        assert!(x < self.width && y < self.height);
        &self.data[x + y * self.width]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        assert!(x < self.width && y < self.height);
        &mut self.data[x + y * self.width]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ground {
    Ash,
    Rock,
}

impl Display for Ground {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ash => write!(f, "."),
            Self::Rock => write!(f, "#"),
        }
    }
}

impl TryFrom<u8> for Ground {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(Self::Ash),
            b'#' => Ok(Self::Rock),
            _ => Err(()),
        }
    }
}

fn parse_grid(input: &str) -> Grid<Ground> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let data = input
        .as_bytes()
        .iter()
        .filter_map(|&b| b.try_into().ok())
        .collect();

    Grid {
        width,
        height,
        data,
        cache_rows: RefCell::new(Matrix::new(height)),
        cache_columns: RefCell::new(Matrix::new(width)),
    }
}

pub fn generate(input: &str) -> Vec<Grid<Ground>> {
    input.split("\n\n").map(parse_grid).collect()
}

fn check_span_row(grid: &Grid<Ground>, y1: usize, y2: usize, max_errors: u8) -> bool {
    let extends = grid.rows_equal(y1, y2);
    if extends > max_errors {
        return false;
    }
    let between_errors = grid.check_between_rows(y1, y2, max_errors.saturating_sub(extends));
    if extends + between_errors > max_errors {
        return false;
    }
    (y1 + y2) % 2 == 1 && extends + between_errors == max_errors
}
fn check_span_column(grid: &Grid<Ground>, x1: usize, x2: usize, max_errors: u8) -> bool {
    let extends = grid.columns_equal(x1, x2);
    if extends > max_errors {
        return false;
    }
    let between_errors = grid.check_between_columns(x1, x2, max_errors.saturating_sub(extends));
    if extends + between_errors > max_errors {
        return false;
    }
    (x1 + x2) % 2 == 1 && extends + between_errors == max_errors
}

fn span_rows(grid: &Grid<Ground>, max_errors: u8) -> Option<(usize, usize)> {
    for y in 1..(grid.height - 1) {
        if check_span_row(grid, 0, y, max_errors) {
            return Some((0, y));
        }
        if check_span_row(grid, y, grid.height - 1, max_errors) {
            return Some((y, grid.height - 1));
        }
    }
    None
}
fn span_colums(grid: &Grid<Ground>, max_errors: u8) -> Option<(usize, usize)> {
    for x in 1..(grid.width - 1) {
        if check_span_column(grid, 0, x, max_errors) {
            return Some((0, x));
        }
        if check_span_column(grid, x, grid.width - 1, max_errors) {
            return Some((x, grid.width - 1));
        }
    }
    None
}

fn solve(input: &[Grid<Ground>], max_errors: u8) -> usize {
    input
        .iter()
        .map(|g| {
            let midpoint_row = span_rows(g, max_errors)
                .map(|(a, b)| 1 + (a + b) / 2)
                .unwrap_or_default();
            let midpoint_column = span_colums(g, max_errors)
                .map(|(a, b)| 1 + (a + b) / 2)
                .unwrap_or_default();

            if midpoint_column != 0 && midpoint_row != 0 {
                eprintln!("{midpoint_column}, {midpoint_row}\n\n{g}");
            }

            100 * midpoint_row + midpoint_column
        })
        .sum()
}

pub fn part1(input: &[Grid<Ground>]) -> usize {
    solve(input, 0)
}
pub fn part2(input: &[Grid<Ground>]) -> usize {
    solve(input, 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
        assert_eq!(405, part1(&generate(input)));
    }

    #[test]
    fn test_p2() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        assert_eq!(400, part2(&generate(input)))
    }
}
