pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut digits = l.chars().filter(|c| c.is_digit(10));
            let first = digits.next().expect("there to be at least 1 digit");
            let last = digits.next_back().unwrap_or(first);

            return 10 * first.to_digit(10).unwrap() + last.to_digit(10).unwrap();
        })
        .sum()
}

fn replace_number(input: &str) -> String {
    // weird replacement to make sure overlapping numbers dont get destroyed
    input
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nice")
}

pub fn part2(input: &str) -> u32 {
    part1(&replace_number(input))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        assert_eq!(part1(input), 142);
    }
    #[test]
    fn test_p2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        assert_eq!(part2(input), 281);
    }
}
