use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till1},
    character::complete::{char, digit1},
    combinator::{all_consuming, eof, map, map_res},
    error::VerboseError,
    multi::{count, many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    Err,
};
use std::ops::Range;

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let (mut seeds, steps) = parse(input).unwrap();

    for step in steps {
        for seed in &mut seeds {
            for map in &step {
                if map.0.contains(seed) {
                    *seed = *seed + map.1.start - map.0.start;
                    break;
                }
            }
        }
    }

    println!("Min: {}", seeds.iter().min().unwrap());
}

#[derive(Debug, Clone)]
struct Ranges {
    pub ranges: Vec<(u64, u64)>,
}

impl Ranges {
    pub fn new(ranges: Vec<Range<u64>>) -> Self {
        let mut ret = Self {
            ranges: Vec::with_capacity(ranges.len()),
        };
        for range in ranges {
            ret.add_range(range);
        }
        ret
    }

    pub fn add_range(&mut self, mut range: Range<u64>) {
        if self.ranges.len() == 0 {
            self.ranges.push((range.start, range.end));
            return;
        }

        for i in 0..self.ranges.len() {
            // Add at position i
            if range.end < self.ranges[i].0 {
                self.ranges.insert(i, (range.start, range.end));
                self.ranges.retain(|r| *r != (0, 0));
                return;
            }
            let left_inside = range.start >= self.ranges[i].0 && range.start <= self.ranges[i].1;
            let right_inside = range.end >= self.ranges[i].0 && range.end <= self.ranges[i].1;
            let totally_inside = self.ranges[i].0 >= range.start && self.ranges[i].0 <= range.end;
            match (left_inside, right_inside, totally_inside) {
                (true, true, _) => {
                    // Totally inside, so do nothing and return
                    self.ranges.retain(|r| *r != (0, 0));
                    return;
                }
                (true, false, _) => {
                    // Left edge intersecting, so extend existing range
                    range.start = self.ranges[i].0;
                    self.ranges[i] = (0, 0);
                }
                (false, true, _) => {
                    // Right edge intersecting, so extend existing range
                    range.end = self.ranges[i].1;
                    self.ranges[i] = (0, 0);
                }
                (false, false, true) => {
                    // Totally inside, so delete range
                    self.ranges[i] = (0, 0);
                }
                (false, false, _) => (), // Nothing, continue looking
            }
        }

        // Doesn't intersect anything, so add to end
        self.ranges.push((range.start, range.end));
        self.ranges.retain(|r| *r != (0, 0));
    }

    pub fn transform(&mut self, transformations: Vec<(Range<u64>, Range<u64>)>) {
        let mut additions = vec![];

        for (source, destination) in transformations {
            assert!(source.end - source.start == destination.end - destination.start);
            for i in 0..self.ranges.len() {
                let range = self.ranges[i];
                let left_inside = range.0 >= source.start && range.0 <= source.end;
                let right_inside = range.1 >= source.start && range.1 <= source.end;
                let source_inside = source.start >= range.0 && source.start <= range.1;
                match (left_inside, right_inside, source_inside) {
                    (true, true, _) => {
                        // Totally inside, so remove and add new range
                        let start = range.0 - source.start + destination.start;
                        let end = range.1 - source.start + destination.start;
                        self.ranges[i] = (0, 0);
                        additions.push(start..end);
                    }
                    (true, false, _) => {
                        // Left edge intersecting, so change and add new range
                        let start = range.0 - source.start + destination.start;
                        let end = destination.end;
                        self.ranges[i] = (source.end, range.1);
                        additions.push(start..end);
                    }
                    (false, true, _) => {
                        // Right edge intersecting, so change and add new range
                        let start1 = destination.start;
                        let end1 = range.1 - source.start + destination.start;
                        self.ranges[i] = (range.0, source.start);
                        additions.push(start1..end1);
                    }
                    (false, false, true) => {
                        // Source is entirely inside range, weird stuff
                        self.ranges[i] = (range.0, source.start);
                        additions.push(destination.start..destination.end);
                        self.ranges.insert(i + 1, (source.end, range.1));
                    }
                    (false, false, false) => (), // Nothing, continue looking
                }
            }
        }

        self.ranges.retain(|r| *r != (0, 0));

        for new in additions {
            self.add_range(new);
        }
    }
}

fn part2(input: &str) {
    let (seeds, steps) = parse(input).unwrap();
    let mut seeds = Ranges::new(
        seeds
            .into_iter()
            .tuples()
            .map(|(start, length)| start..(start + length))
            .collect(),
    );

    for step in steps {
        seeds.transform(step);
    }

    println!("Seed: {}", seeds.ranges[0].0);
}

fn parse(
    input: &str,
) -> Result<(Vec<u64>, Vec<Vec<(Range<u64>, Range<u64>)>>), Err<VerboseError<&str>>> {
    all_consuming(tuple((
        preceded(
            tag("seeds: "),
            separated_list1(char(' '), map_res(digit1, str::parse)),
        ),
        many1(preceded(
            take_till1(|c: char| c.is_numeric()),
            many1(map(
                count(
                    terminated(map_res(digit1, str::parse), alt((take(1usize), eof))),
                    3,
                ),
                |c| (c[1]..(c[1] + c[2]), c[0]..(c[0] + c[2])),
            )),
        )),
    )))(input)
    .map(|r| r.1)
}
