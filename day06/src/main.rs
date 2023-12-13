use nom::{
    branch::alt,
    bytes::complete::{take, take_till, take_till1},
    character::complete::{char, digit1, newline},
    combinator::{all_consuming, eof, map, map_res},
    error::VerboseError,
    multi::{count, many1},
    sequence::{preceded, terminated, tuple},
    Err,
};

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let input = parse(input).unwrap();
    let mut maxes = vec![];

    for (t, d) in input {
        let min = ((t - (t * t - 4.0 * d).sqrt()) / 2.0) as u64 + 1;
        let max = ((t + (t * t - 4.0 * d).sqrt()) / 2.0).ceil() as u64 - 1;
        maxes.push(max - min + 1);
    }

    println!("Product: {}", maxes.iter().product::<u64>());
}

fn part2(input: &str) {
    let input = parse(input).unwrap();

    let input = input
        .into_iter()
        .fold(("".to_string(), "".to_string()), |(a, b), (c, d)| {
            (a + &(c as u64).to_string(), b + &(d as u64).to_string())
        });
    let (t, d): (f64, f64) = (input.0.parse().unwrap(), input.1.parse().unwrap());

    let min = ((t - (t * t - 4.0 * d).sqrt()) / 2.0) as u64 + 1;
    let max = ((t + (t * t - 4.0 * d).sqrt()) / 2.0).ceil() as u64 - 1;

    println!("Count: {}", max - min + 1);
}

fn parse(input: &str) -> Result<Vec<(f64, f64)>, Err<VerboseError<&str>>> {
    all_consuming(map(
        tuple((
            many1(preceded(
                take_till1(|c: char| c.is_numeric() || c == '\n'),
                map_res(digit1, str::parse),
            )),
            many1(preceded(
                take_till1(|c: char| c.is_numeric()),
                map_res(digit1, str::parse),
            )),
        )),
        |(t, d)| t.into_iter().zip(d).collect(),
    ))(input)
    .map(|r| r.1)
}
