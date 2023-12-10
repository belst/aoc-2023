fn idx(x: usize, y: usize, width: usize) -> usize {
    x + y * width
}

fn xy(idx: usize, width: usize) -> (usize, usize) {
    (idx % width, idx / width)
}

pub fn generate(input: &str) -> (Vec<usize>, usize, usize) {
    let width = input.lines().next().unwrap().len() + 1; // +1 because of \n
    let height = input.lines().count();
    let s_idx = input.find('S').unwrap();
    let input = input.as_bytes();
    let mut ret = vec![s_idx];
    let mut pos = s_idx;
    let (x, y) = xy(s_idx, width);
    // check left
    if x > 0 && [b'-', b'F', b'L'].contains(&input[s_idx - 1]) {
        pos = s_idx - 1;
    // check top
    } else if y > 0 && [b'|', b'7', b'F'].contains(&input[s_idx - width]) {
        pos = s_idx - width;
    // check right
    } else if x < width - 1 && [b'-', b'J', b'7'].contains(&input[s_idx + 1]) {
        pos = s_idx + 1;
    // check bottom
    } else if s_idx + width < input.len() && [b'|', b'L', b'J'].contains(&input[s_idx + width]) {
        pos = s_idx + width
    }

    while input[pos] != b'S' {
        let last_pos = ret[ret.len() - 1];
        ret.push(pos);
        match input[pos] {
            b'|' => {
                if last_pos != pos - width {
                    pos -= width;
                } else {
                    pos += width;
                }
            }
            b'-' => {
                if last_pos != pos - 1 {
                    pos -= 1;
                } else {
                    pos += 1;
                }
            }
            b'L' => {
                if last_pos != pos - width {
                    pos -= width;
                } else {
                    pos += 1;
                }
            }
            b'J' => {
                if last_pos != pos - width {
                    pos -= width;
                } else {
                    pos -= 1;
                }
            }
            b'7' => {
                if last_pos != pos - 1 {
                    pos -= 1;
                } else {
                    pos += width;
                }
            }
            b'F' => {
                if last_pos != pos + 1 {
                    pos += 1;
                } else {
                    pos += width;
                }
            }
            _ => {
                unreachable!()
            }
        }
    }
    (ret, width, height)
}

pub fn part1(input: &(Vec<usize>, usize, usize)) -> usize {
    input.0.len() / 2
}

fn prev_next(idx: usize, loopidx: &[usize]) -> (usize, usize) {
    (
        loopidx[(idx + loopidx.len() - 1) % loopidx.len()],
        loopidx[(idx + 1) % loopidx.len()],
    )
}

pub fn part2(&(ref loopidx, width, height): &(Vec<usize>, usize, usize)) -> usize {
    let mut c = 0;
    for y in 0..height {
        let mut intersections = 0;
        let mut last_intersection_direction = isize::MAX;
        for x in 0..width {
            if let Some(idx) = loopidx.iter().position(|&e| e == idx(x, y, width)) {
                let (prev, next) = prev_next(idx, loopidx);
                let (x_prev, y_prev) = xy(prev, width);
                let (x_next, y_next) = xy(next, width);
                let y_direction = if y_prev < y_next { 1 } else { -1 };

                // only count vertical
                // and only count if direction changes, otherwise we already counted
                if [x_next, x_prev].contains(&x) && y_direction != last_intersection_direction {
                    intersections += 1;
                    last_intersection_direction = y_direction;
                }
            } else if intersections % 2 == 1 {
                c += 1;
            }
        }
    }

    c
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1_1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";

        assert_eq!(4, part1(&generate(input)));
    }
    #[test]
    fn test_p1_2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        assert_eq!(8, part1(&generate(input)));
    }

    #[test]
    fn test_p2_1() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        assert_eq!(4, part2(&generate(input)));
    }

    #[test]
    fn test_p2_2() {
        let input = "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........";
        assert_eq!(4, part2(&generate(input)));
    }

    #[test]
    fn test_p2_3() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        assert_eq!(8, part2(&generate(input)));
    }

    #[test]
    fn test_p3_4() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        assert_eq!(10, part2(&generate(input)));
    }
}
