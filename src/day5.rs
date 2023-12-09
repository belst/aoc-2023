use rayon::prelude::*;
use std::ops::Range;

type Map = (usize, usize, usize);

#[derive(Debug, Clone)]
struct Mapping {
    map: Vec<Map>,
    next: Option<Box<Mapping>>,
}

impl Mapping {
    fn match_item(&self, input: usize) -> (usize, Option<&Mapping>) {
        let dest = self
            .map
            .iter()
            .find(|(_, source, count)| input >= *source && input < *source + *count)
            .map(|(dest, source, _)| dest + (input - source))
            .unwrap_or(input);
        (dest, self.next.as_deref())
    }
}

#[derive(Debug, Clone)]
pub struct Almanac {
    seeds: Vec<usize>,
    maps: Mapping,
}
pub fn part1(input: &Almanac) -> usize {
    input
        .seeds
        .iter()
        .map(|s| {
            let mut next = Some(&input.maps);
            let mut current = *s;
            loop {
                let (c, n) = next.unwrap().match_item(current);
                if n.is_none() {
                    return c;
                }
                next = n;
                current = c;
            }
        })
        .min()
        .unwrap()
}

struct RangeSet {
    inner: Vec<Range<usize>>,
}

impl RangeSet {
    fn new() -> Self {
        Self { inner: vec![] }
    }

    fn insert(&mut self, item: Range<usize>) {
        let mut overlapping = vec![];
        for i in 0..self.inner.len() {
            if item.start < self.inner[i].end && item.end > self.inner[i].start {
                overlapping.push(i);
            }
        }
        let min = item.start.min(
            overlapping
                .iter()
                .map(|&i| self.inner[i].start)
                .min()
                .unwrap_or(item.start),
        );
        let max = item.end.max(
            overlapping
                .iter()
                .map(|&i| self.inner[i].end)
                .min()
                .unwrap_or(item.end),
        );

        overlapping.iter().for_each(|&i| {
            self.inner.swap_remove(i);
        });

        self.inner.push(min..max);
    }

    fn iter_values(&self) -> impl ParallelIterator<Item = usize> + '_ {
        self.inner.par_iter().flat_map(|r| r.clone())
    }
}

impl FromIterator<Range<usize>> for RangeSet {
    fn from_iter<T: IntoIterator<Item = Range<usize>>>(iter: T) -> Self {
        let mut rs = Self::new();
        for i in iter {
            rs.insert(i);
        }
        rs
    }
}

pub fn part2(input: &Almanac) -> usize {
    let ranges: RangeSet = input
        .seeds
        .chunks_exact(2)
        .map(|s| (s[0]..s[0] + s[1]))
        .collect();

    ranges
        .iter_values()
        .filter_map(|s| {
            let mut next = Some(&input.maps);
            let mut current = s;
            loop {
                let (c, n) = next.unwrap().match_item(current);
                if n.is_none() {
                    return Some(c);
                }
                next = n;
                current = c;
            }
        })
        .min()
        .unwrap()
}

fn parse_map<'a>(it: &mut impl Iterator<Item = &'a str>) -> Vec<Map> {
    it.take_while(|l| !l.is_empty())
        .map(|l| {
            let mut numbers = l.split_whitespace().flat_map(|n| n.parse::<usize>().ok());
            let destination = numbers.next().unwrap();
            let source = numbers.next().unwrap();
            let count = numbers.next().unwrap();
            (destination, source, count)
        })
        .collect()
}

pub fn generate(input: &str) -> Almanac {
    let mut it = input.lines();
    let seeds: Vec<_> = it
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .flat_map(|s| s.parse::<usize>().ok())
        .collect();
    it.next(); // skip empty line
    let mapname = it.next().unwrap();
    assert_eq!(mapname, "seed-to-soil map:");
    let seed_to_soil_map: Vec<_> = parse_map(&mut it);

    //it.next(); // skip empty line
    let mapname = it.next().unwrap();
    assert_eq!(mapname, "soil-to-fertilizer map:");
    let soil_to_fertilizer: Vec<_> = parse_map(&mut it);

    //it.next(); // skip empty line
    let mapname = it.next().unwrap();
    assert_eq!(mapname, "fertilizer-to-water map:");
    let fertilizer_to_water: Vec<_> = parse_map(&mut it);

    //it.next(); // skip empty line
    let mapname = it.next().unwrap();
    assert_eq!(mapname, "water-to-light map:");
    let water_to_light: Vec<_> = parse_map(&mut it);

    //it.next(); // skip empty line
    let mapname = it.next().unwrap();
    assert_eq!(mapname, "light-to-temperature map:");
    let light_to_temperature: Vec<_> = parse_map(&mut it);

    //it.next(); // skip empty line
    let mapname = it.next().unwrap();
    assert_eq!(mapname, "temperature-to-humidity map:");
    let temperature_to_humidity: Vec<_> = parse_map(&mut it);

    //it.next(); // skip empty line
    let mapname = it.next().unwrap();
    assert_eq!(mapname, "humidity-to-location map:");
    let humidity_to_location: Vec<_> = parse_map(&mut it);

    Almanac {
        seeds,
        maps: Mapping {
            map: seed_to_soil_map,
            next: Some(Box::new(Mapping {
                map: soil_to_fertilizer,
                next: Some(Box::new(Mapping {
                    map: fertilizer_to_water,
                    next: Some(Box::new(Mapping {
                        map: water_to_light,
                        next: Some(Box::new(Mapping {
                            map: light_to_temperature,
                            next: Some(Box::new(Mapping {
                                map: temperature_to_humidity,
                                next: Some(Box::new(Mapping {
                                    map: humidity_to_location,
                                    next: None,
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(35, part1(&generate(input)))
    }
    #[test]
    fn test_p2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(46, part2(&generate(input)))
    }
}
