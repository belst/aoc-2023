#[derive(Debug, Clone, Default)]
pub struct Draw {
    red: usize,
    blue: usize,
    green: usize,
}

type Game = Vec<Draw>;

pub fn generate(input: &str) -> Vec<Game> {
    let mut games = vec![];
    for l in input.lines() {
        let game = l.split_once(':').expect("line to contain :").1;
        let mut draws = vec![];
        for draw in game.split(';') {
            let mut d = Draw::default();
            for cube in draw.split(",") {
                let (amount, color) = cube.trim().split_once(" ").expect("A cube and a number");
                match color.trim() {
                    "red" => d.red = amount.trim().parse().expect("to parse a number"),
                    "blue" => d.blue = amount.trim().parse().expect("to parse a number"),
                    "green" => d.green = amount.trim().parse().expect("to parse a number"),
                    x => {
                        dbg!(x);
                        unreachable!()
                    }
                }
            }
            draws.push(d);
        }
        games.push(draws)
    }
    games
}

pub fn part1(input: &Vec<Game>) -> usize {
    input
        .into_iter()
        .enumerate()
        .filter(|(_, g)| {
            g.iter()
                .all(|d| d.red <= 12 && d.blue <= 14 && d.green <= 13)
        })
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part2(input: &Vec<Game>) -> usize {
    input
        .iter()
        .map(|g| {
            g.iter().fold(Draw::default(), |a, c| Draw {
                red: a.red.max(c.red),
                blue: a.blue.max(c.blue),
                green: a.green.max(c.green),
            })
        })
        .map(|d| d.red * d.green * d.blue)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        assert_eq!(8, part1(&generate(input)));
    }

    #[test]
    fn test_p2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        assert_eq!(2286, part2(&generate(input)));
    }
}
