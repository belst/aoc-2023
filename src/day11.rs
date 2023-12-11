fn idx(x: usize, y: usize, width: usize) -> usize {
    x + y * width
}

type Coord = (usize, usize);

pub fn generate(input: &str) -> Vec<[Coord; 2]> {
    let height = input.lines().filter(|l| !l.is_empty()).count();
    let input = input.as_bytes();
    let width = input.iter().position(|&b| b == b'\n').unwrap();
    let mut v = vec![];
    let mut x = 0;
    let mut x2 = 0;
    let mut y = 0;
    let mut y2 = 0;
    let mut empty_columns = vec![];

    for y_in in 0..height {
        let mut all_empty = true;
        for x_in in 0..width {
            // on first loop, add all the empty columns
            if y_in == 0
                && (0..height)
                    .map(|y| input[idx(x_in, y, width + 1)])
                    .all(|b| b == b'.')
            {
                empty_columns.push(x_in);
            }
            if empty_columns.contains(&x_in) {
                x += 1;
                x2 += 1_000_000 - 1;
            }
            if input[idx(x_in, y_in, width + 1)] == b'#' {
                v.push([(x, y), (x2, y2)]);
                all_empty &= false;
            }

            x += 1;
            x2 += 1;
        }
        x = 0;
        x2 = 0;
        if all_empty {
            y += 1;
            y2 += 1_000_000 - 1;
        }
        y += 1;
        y2 += 1;
    }

    v
}

pub fn solve(input: &[[Coord; 2]], part: usize) -> usize {
    let mut sum = 0;
    for a in 0..input.len() {
        for b in (a + 1)..input.len() {
            let (x1, y1) = input[a][part - 1];
            let (x2, y2) = input[b][part - 1];
            sum += x1.abs_diff(x2) + y1.abs_diff(y2);
        }
    }
    sum
}

pub fn part1(input: &[[Coord; 2]]) -> usize {
    solve(input, 1)
}
pub fn part2(input: &[[Coord; 2]]) -> usize {
    solve(input, 2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

        assert_eq!(374, part1(&generate(input)));
    }
}
