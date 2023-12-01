use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map_res,
    error::Error,
    multi::separated_list1,
    sequence::separated_pair,
    Err,
};
use std::collections::HashMap;

type Str = &'static str;

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: Str) {
    let mut map = parse_input(input).unwrap();
    let maxy = map.iter().map(|(k, _)| k.1).max().unwrap();
    let mut count = 0;

    'outer: loop {
        let mut x = 500;
        let mut y = 0;

        loop {
            if y > maxy {
                break 'outer;
            } else if let None = map.get(&(x, y + 1)) {
                y += 1;
            } else if let None = map.get(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else if let None = map.get(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else {
                break;
            }
        }

        map.insert((x, y), Particle::Sand);
        count += 1;
    }

    _print_map(&map);
    dbg!(count);
}

fn part2(input: Str) {
    let mut map = parse_input(input).unwrap();
    let floor = map.iter().map(|(k, _)| k.1).max().unwrap() + 1;
    let mut count = 0;

    'outer: loop {
        let mut x = 500;
        let mut y = 0;
        count += 1;

        loop {
            if y == floor {
                break;
            } else if let None = map.get(&(x, y + 1)) {
                y += 1;
            } else if let None = map.get(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else if let None = map.get(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else {
                if x == 500 && y == 0 {
                    break 'outer;
                }
                break;
            }
        }

        map.insert((x, y), Particle::Sand);
    }

    _print_map(&map);
    dbg!(count);
}

type ParticleMap = HashMap<(isize, isize), Particle>;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Particle {
    Rock,
    Sand,
}

fn _print_map(map: &ParticleMap) {
    let minx = map.iter().map(|(k, _)| k.0).min().unwrap() - 3;
    let maxx = map.iter().map(|(k, _)| k.0).max().unwrap() + 3;
    let miny = map.iter().map(|(k, _)| k.1).min().unwrap() - 3;
    let maxy = map.iter().map(|(k, _)| k.1).max().unwrap() + 3;

    for j in 0..maxy - miny {
        for i in 0..maxx - minx {
            match map.get(&(minx + i, miny + j)) {
                Some(Particle::Rock) => print!("#"),
                Some(Particle::Sand) => print!("o"),
                None => print!("."),
            }
        }
        println!();
    }
}

fn parse_input(input: Str) -> Result<ParticleMap, Err<Error<Str>>> {
    let (_, list) = separated_list1(
        char('\n'),
        separated_list1(
            tag(" -> "),
            separated_pair(
                map_res(digit1, str::parse::<isize>),
                char(','),
                map_res(digit1, str::parse::<isize>),
            ),
        ),
    )(input)?;

    let mut map = HashMap::new();
    for line in list {
        for i in 0..line.len() - 1 {
            let delx = line[i + 1].0 - line[i].0;
            let dely = line[i + 1].1 - line[i].1;
            for j in 0..=(delx + dely).abs() {
                map.insert(
                    (line[i].0 + j * delx.signum(), line[i].1 + j * dely.signum()),
                    Particle::Rock,
                );
            }
        }
    }

    Ok(map)
}
