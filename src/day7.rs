use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        use Card as C;
        match value {
            '2' => C::Two,
            '3' => C::Three,
            '4' => C::Four,
            '5' => C::Five,
            '6' => C::Six,
            '7' => C::Seven,
            '8' => C::Eight,
            '9' => C::Nine,
            'T' => C::T,
            'J' => C::J,
            'Q' => C::Q,
            'K' => C::K,
            'A' => C::A,
            _ => panic!("Invalid Input"),
        }
    }
}

type Hand = [Card; 5];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HandType<T> {
    HighCard(T),
    OnePair(T),
    TwoPair(T),
    ThreeOfAKind(T),
    FullHouse(T),
    FourOfAKind(T),
    FiveOfAKind(T),
}

impl From<Hand> for HandType<Hand> {
    fn from(value: Hand) -> Self {
        use HandType as HT;
        let mut hm = HashMap::new();
        for v in value {
            hm.entry(v).and_modify(|c| *c += 1).or_insert(1);
        }
        match hm.len() {
            5 => HT::HighCard(value),
            4 => HT::OnePair(value),
            3 if hm.values().max() == Some(&2) => HT::TwoPair(value),
            3 if hm.values().max() == Some(&3) => HT::ThreeOfAKind(value),
            2 if hm.values().max() == Some(&3) => HT::FullHouse(value),
            2 if hm.values().max() == Some(&4) => HT::FourOfAKind(value),
            1 => HT::FiveOfAKind(value),
            _ => panic!("Cannot have more than 5 unique values in a hand"),
        }
    }
}

pub fn generate(input: &str) -> Vec<(Hand, usize)> {
    let mut v = vec![];
    for l in input.lines().filter(|l| !l.is_empty()) {
        let (hs, bids) = l.split_once(' ').unwrap();
        v.push((
            hs.chars()
                .map(From::from)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            bids.parse().unwrap(),
        ));
    }
    v
}

pub fn part1(input: &[(Hand, usize)]) -> usize {
    let mut input: Vec<(HandType<Hand>, usize)> =
        input.iter().map(|&(h, b)| (h.into(), b)).collect();

    input.sort_by_key(|v| v.0);

    input
        .into_iter()
        .enumerate()
        .map(|(i, (_, b))| b * (i + 1))
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct P2Card(Card);

impl Ord for P2Card {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 == other.0 {
            Ordering::Equal
        } else if self.0 == Card::J {
            Ordering::Less
        } else if other.0 == Card::J {
            Ordering::Greater
        } else {
            self.0.cmp(&other.0)
        }
    }
}

impl PartialOrd for P2Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
type P2Hand = [P2Card; 5];

fn convert_to_p2(value: Hand) -> P2Hand {
    [
        P2Card(value[0]),
        P2Card(value[1]),
        P2Card(value[2]),
        P2Card(value[3]),
        P2Card(value[4]),
    ]
}

impl From<P2Hand> for HandType<P2Hand> {
    fn from(value: P2Hand) -> Self {
        use HandType as HT;
        let mut hm = HashMap::new();
        for v in value {
            hm.entry(v).and_modify(|c| *c += 1).or_insert(1);
        }
        let len = if hm.contains_key(&P2Card(Card::J)) {
            hm.len() - 1
        } else {
            hm.len()
        };
        match len {
            5 => HT::HighCard(value),        // No joker here
            4 => HT::OnePair(value),         // maybe one joker, or maybe none, still only 1 pair
            0 | 1 => HT::FiveOfAKind(value), // if 0, all are jokers, if 1 it might be 5 of a
            // different kind, OR one kind of a different card and the rest jokers
            3 if hm.contains_key(&P2Card(Card::J)) => HT::ThreeOfAKind(value), // eihter one or 2
            // jokers, makes this a 3 of a kind regardless (3 +1 joker means at least on other
            // card has to double up)
            3 if hm.values().max() == Some(&2) => HT::TwoPair(value), // No joker here and the next
            // one
            3 if hm.values().max() == Some(&3) => HT::ThreeOfAKind(value),
            // if I have 2 uniques and at least 1 joker, the only way to get a full house is if
            // both of the other uniques double up, eg: J AA KK
            // otherwise this will always be a FourOfAKind
            // eg: J A KKK, JJ A KK, JJJ A K (Pretend J is K)
            2 if hm.contains_key(&P2Card(Card::J)) => {
                let lowest_non_joker_count = *hm
                    .iter()
                    .filter_map(|(k, v)| if k == &P2Card(Card::J) { None } else { Some(v) })
                    .min()
                    .unwrap();
                if lowest_non_joker_count == 1 {
                    HT::FourOfAKind(value)
                } else {
                    HT::FullHouse(value)
                }
            }
            // No Jokers here, solve as in Part1
            2 if hm.values().max() == Some(&3) => HT::FullHouse(value),
            2 if hm.values().max() == Some(&4) => HT::FourOfAKind(value),
            _ => panic!("Cannot have more than 5 unique values in a hand: {hm:?}"),
        }
    }
}

pub fn part2(input: &[(Hand, usize)]) -> usize {
    let mut input: Vec<(HandType<P2Hand>, usize)> = input
        .iter()
        .map(|&(h, b)| (convert_to_p2(h).into(), b))
        .collect();

    input.sort_by_key(|v| v.0);

    input
        .into_iter()
        .enumerate()
        .map(|(i, (_, b))| b * (i + 1))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(6440, part1(&generate(input)));
    }
    #[test]
    fn test_p2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(5905, part2(&generate(input)));
    }
}
