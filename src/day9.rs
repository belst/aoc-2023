pub fn generate(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|l| l.split_whitespace().flat_map(|n| n.parse().ok()).collect())
        .collect()
}

fn differences(input: &[isize]) -> impl Iterator<Item = isize> + '_ {
    input.windows(2).map(|s| s[1] - s[0])
}

fn next(input: &[isize]) -> isize {
    let d: Vec<_> = differences(input).collect();
    if d.iter().all(|&n| n == 0) {
        return input[0];
    }
    input.last().unwrap() + next(&d)
}

pub fn part1(input: &[Vec<isize>]) -> isize {
    input.iter().map(|seq| next(seq)).sum()
}
fn prev(input: &[isize]) -> isize {
    let d: Vec<_> = differences(input).collect();
    if d.iter().all(|&n| n == 0) {
        return input[0];
    }
    input.first().unwrap() - prev(&d)
}
pub fn part2(input: &[Vec<isize>]) -> isize {
    input.iter().map(|seq| prev(seq)).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!(114, part1(&generate(input)));
    }
    #[test]
    fn test_p2() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!(2, part2(&generate(input)));
    }
}
