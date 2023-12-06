use std::iter::zip;

#[derive(Debug, Clone, Copy)]
pub struct Race {
    time: usize,
    distance: usize,
}

pub fn generate(input: &str) -> Vec<Race> {
    let lines: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .flat_map(|n| n.parse().ok())
                .collect()
        })
        .collect();
    zip(lines[0].iter(), lines[1].iter())
        .map(|(&time, &distance)| Race { time, distance })
        .collect()
}

// formula final distance:
// t * (max_t - t) > d
// t * max_t - t * t > d
// max_t -t > d / t
pub fn part1(input: &[Race]) -> usize {
    input.iter()
        .map(|r| {
            for n in 0..r.time / 2 {
                if n * (r.time - n) > r.distance {
                    return r.time - n - n + 1;
                }
            }
            1
        }).product()
}

pub fn part2(input: &[Race]) -> usize {
    let (t, d) = input.iter().fold((String::new(), String::new()), |(mut t, mut d), r| {
        t += &r.time.to_string();
        d += &r.distance.to_string();
        (t, d)
    });
    part1(&[Race {
        time: t.parse().unwrap(),
        distance: d.parse().unwrap()
    }])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(288, part1(&generate(input)));
    }
    #[test]
    fn test_p2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(71503, part2(&generate(input)));
    }
}
