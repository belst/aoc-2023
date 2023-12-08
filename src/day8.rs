use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

pub struct Map<'a> {
    directions: Vec<Direction>,
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

impl Map<'_> {
    fn next(&self, direction: Direction, current: &str) -> &str {
        let (l, r) = self.nodes.get(current).unwrap();

        match direction {
            Direction::Left => l,
            Direction::Right => r,
        }
    }
}

pub fn generate(input: &str) -> Map {
    let mut it = input.lines();
    let directions = it
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => unreachable!(),
        })
        .collect();

    it.next().unwrap(); // empty line;
    let nodes = it
        .map(|s| {
            let (key, tuple) = s.split_once(" = ").unwrap();
            let tuple = tuple
                .trim_matches(&['(', ')'][..])
                .split_once(", ")
                .unwrap();
            (key, tuple)
        })
        .collect();

    Map { directions, nodes }
}

pub fn part1(input: &Map) -> usize {
    input
        .directions
        .iter()
        .cycle()
        .scan("AAA", |state, &d| {
            *state = input.next(d, state);

            if *state == "ZZZ" {
                None
            } else {
                Some(1)
            }
        })
        .count()
        + 1
}

// it would be smart to store the state somewhere, so we dont have to recalculate it every
// iteration
// stolen from num crate
fn gcd(a: usize, b: usize) -> usize {
    // Use Stein's algorithm
    let mut m = a;
    let mut n = b;
    if m == 0 || n == 0 {
        return m | n;
    }

    // find common factors of 2
    let shift = (m | n).trailing_zeros();

    // divide n and m by 2 until odd
    m >>= m.trailing_zeros();
    n >>= n.trailing_zeros();

    while m != n {
        if m > n {
            m -= n;
            m >>= m.trailing_zeros();
        } else {
            n -= m;
            n >>= n.trailing_zeros();
        }
    }
    m << shift
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 && b == 0 {
        0
    } else {
        a * (b / gcd(a, b))
    }
}
pub fn part2(input: &Map) -> usize {
    let mut starting: Vec<_> = input
        .nodes
        .keys()
        .filter_map(|&k| if k.ends_with('A') { Some((k, 0)) } else { None })
        .collect();

    for &d in input.directions.iter().cycle() {
        for (n, c) in starting.iter_mut().filter(|(k, _)| !k.ends_with('Z')) {
            *n = input.next(d, n);
            *c += 1;
        }
        if starting.iter().all(|(n, _)| n.ends_with('Z')) {
            break;
        }
    }
    starting.into_iter().fold(1, |a, (_, b)| lcm(a, b))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testp1_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(2, part1(&generate(input)));
    }
    #[test]
    fn testp1_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(6, part1(&generate(input)));
    }

    #[test]
    fn test_p2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(6, part2(&generate(input)));
    }
}
