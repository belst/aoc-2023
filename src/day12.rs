use itertools::{intersperse, repeat_n, EitherOrBoth, Itertools};
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Spring {
    Ok,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ok,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("Invalid character"),
        }
    }
}

type SpringRow = (Vec<Spring>, Vec<usize>);

fn parse_line(line: &str) -> SpringRow {
    let (springs, chksums) = line.split_once(' ').unwrap();
    let springs = springs.chars().map(Spring::from).collect();
    let chksums = chksums.split(',').map(|n| n.parse().unwrap()).collect();
    (springs, chksums)
}

pub fn generate(input: &str) -> Vec<SpringRow> {
    input.lines().map(parse_line).collect()
}

fn check_configuration(configuration: &[Spring], row: &[Spring], check: &[usize]) -> bool {
    let mut seen = 0;
    let mut c = vec![];
    for &r in row {
        if r == Spring::Unknown {
            c.push(configuration[seen]);
            seen += 1;
        } else {
            c.push(r);
        }
    }
    let groups = c.iter().group_by(|&c| c);
    let broken_count_ranges = groups.into_iter().filter_map(|(&k, group)| {
        if k == Spring::Damaged {
            Some(group.count())
        } else {
            None
        }
    });

    for z in check.iter().zip_longest(broken_count_ranges) {
        if let EitherOrBoth::Both(&c, count) = z {
            if c != count {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

fn count_options((springoptions, springcount): &SpringRow) -> usize {
    // optimize to only generate partitions of unkown
    let unknown_count = springoptions
        .iter()
        .filter(|&&so| so == Spring::Unknown)
        .count();
    repeat_n([Spring::Ok, Spring::Damaged], unknown_count)
        .multi_cartesian_product()
        .filter(|p| check_configuration(p, springoptions, springcount))
        .count()
}

pub fn part1(input: &[SpringRow]) -> usize {
    input.par_iter().map(count_options).sum()
}
pub fn part2(input: &[SpringRow]) -> usize {
    let input: Vec<_> = input
        .iter()
        .map(|(springoptions, springcount)| {
            (
                intersperse(repeat_n(springoptions.iter(), 5), [Spring::Unknown].iter())
                    .flatten()
                    .cloned()
                    .collect(),
                repeat_n(springcount.iter(), 5).flatten().cloned().collect(),
            )
        })
        .collect();
    part1(&input)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p1() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(21, part1(&generate(input)));
    }

    #[test]
    fn test_p1_single() {
        let input = "???.### 1,1,3";

        assert_eq!(1, part1(&generate(input)));
    }
    #[test]
    fn test_p2_single() {
        let input = "???.### 1,1,3";

        // this should only be about 5 times + 4 slower than test_p1_single
        assert_eq!(1, part2(&generate(input)));
    }
    #[test]
    fn test_p2_single_complex() {
        let input = ".??..??...?##. 1,1,3";

        // this will still be slow af
        assert_eq!(16384, part2(&generate(input)));
    }
    #[test]
    fn test_p2() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(525152, part2(&generate(input)));
    }
}
