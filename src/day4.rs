use std::collections::HashSet;


type Deck = Vec<usize>;

pub fn generate(input: &str) -> Deck {
    let mut v = vec![];
    for l in input.lines() {
        let (_, numbers) = l.split_once(':').unwrap();
        let (my, winning) = numbers.trim().split_once(" | ").unwrap();
        let my: HashSet<usize> = my.split_whitespace().flat_map(|n| n.parse().ok()).collect();
        let winning = winning.split_whitespace().flat_map(|n| n.parse().ok()).collect();
        v.push(my.intersection(&winning).count());
    }
    v
}

pub fn part1(input: &Deck) -> usize {
    input.iter().map(|&matching| {
        if matching > 0 {
            1 << (matching - 1)
        } else {
            0
        }
    }).sum()
}

fn process(input: &Deck, current: usize) -> usize {
    let matching = input[current];
    
    if 0 == matching {
        1
    } else {
        1 + (current..(current + matching)).map(|i| process(input, i + 1)).sum::<usize>()
    }

}
pub fn part2(input: &Deck) -> usize {
    (0..input.len()).map(|i| process(input, i)).sum()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(13, part1(&generate(input)))
    }
    #[test]
    fn test_p2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(30, part2(&generate(input)))
    }

}
