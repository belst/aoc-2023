use std::collections::{HashMap, HashSet};

fn surrounding(start: usize, size: usize, width: usize, height: usize) -> Vec<usize> {
    // +1 because of \n
    let x_start = start % width;
    let y = start / width;
    let span = x_start.saturating_sub(1)..=(x_start + size).clamp(0, width);
    let mut idxs = vec![];
    if y > 0 {
        idxs.extend(span.clone().map(|x| (y - 1) * width + x));
    }
    if y < height - 1 {
        idxs.extend(span.map(|x| (y + 1) * width + x));
    }
    if x_start > 0 {
        idxs.push(y * width + x_start - 1);
    }
    if x_start + size < width - 1 {
        idxs.push(y * width + x_start + size);
    }
    idxs
}

pub fn part1(input: &str) -> usize {
    let mut numbers: Vec<usize> = vec![];
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut index = 0;
    while index < input.len() {
        if !input.chars().nth(index).unwrap().is_ascii_digit() {
            index += 1;
            continue;
        }
        let num: String = input
            .chars()
            .skip(index)
            .take_while(|c| c.is_ascii_digit())
            .collect();
        let idxs = surrounding(index, num.len(), width + 1, height);
        if idxs.iter().any(|i| {
            let c = input.chars().nth(*i).unwrap();
            !c.is_ascii_digit() && c != '.' && c != '\n'
        }) {
            numbers.push(num.parse().unwrap());
        }
        index += num.len();
    }

    numbers.into_iter().sum()
}

pub fn part2(input: &str) -> usize {
    let mut numbers = HashMap::new();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut index = 0;

    while index < input.len() {
        if input.chars().nth(index).unwrap().is_ascii_digit() {
            let num: String = input
                .chars()
                .skip(index)
                .take_while(|c| c.is_ascii_digit())
                .collect();
            numbers.insert(index..(index + num.len()), num.parse::<usize>().unwrap());
            index += num.len();
        } else {
            index += 1;
        }
    }

    let positions: Vec<_> = input
        .char_indices()
        .filter_map(|(i, c)| {
            if c == '*' {
                Some(surrounding(i, 1, width + 1, height))
            } else {
                None
            }
        })
        .collect();
    let mut res = 0;
    for s in positions {
        // this would totally break if a '*' was adjacent to the same number twice, eg:
        // 123*123
        // thankfully this doesnt happen in the input
        let s: HashSet<_> = s
            .iter()
            .filter_map(|idx| {
                numbers
                    .iter()
                    .find(|(k, _)| k.contains(idx))
                    .map(|(_, v)| *v)
            })
            .collect();
        if s.len() == 2 {
            res += s.into_iter().product::<usize>();
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(4361, part1(input));
    }
    #[test]
    fn test_p2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(467835, part2(input));
    }
}
