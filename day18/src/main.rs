use nom::{
    character::complete::char,
    character::complete::{digit1, newline},
    combinator::{all_consuming, map_res},
    error::Error,
    multi::separated_list1,
    sequence::{terminated, tuple},
    Err,
};
use std::collections::{HashSet, VecDeque};

type Str = &'static str;

fn main() {
    let input = include_str!("sample.txt");

    part1(input);
    part2(input);
}

fn part1(input: Str) {
    let map = parse_input(input).unwrap();
    let mut count = 0;

    for (x, y, z) in &map {
        let (x, y, z) = (*x, *y, *z);
        // Check all sides
        if !map.contains(&(x + 1, y, z)) {
            count += 1;
        }
        if !map.contains(&(x - 1, y, z)) {
            count += 1;
        }
        if !map.contains(&(x, y + 1, z)) {
            count += 1;
        }
        if !map.contains(&(x, y - 1, z)) {
            count += 1;
        }
        if !map.contains(&(x, y, z + 1)) {
            count += 1;
        }
        if !map.contains(&(x, y, z - 1)) {
            count += 1;
        }
    }

    dbg!(count);
}

fn part2(input: Str) {
    let map = parse_input(input).unwrap();
    let minx = map.iter().map(|(x, _, _)| x).min().unwrap() - 1;
    let maxx = map.iter().map(|(x, _, _)| x).max().unwrap() + 1;
    let miny = map.iter().map(|(_, y, _)| y).min().unwrap() - 1;
    let maxy = map.iter().map(|(_, y, _)| y).max().unwrap() + 1;
    let minz = map.iter().map(|(_, _, z)| z).min().unwrap() - 1;
    let maxz = map.iter().map(|(_, _, z)| z).max().unwrap() + 1;
    let mut count = 0;

    // Find all air squares outside the thing
    let mut air = HashSet::new();
    let mut moves = VecDeque::from([(minx, miny, minz)]);
    while let Some((x, y, z)) = moves.pop_front() {
        let diff = [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ];
        for (dx, dy, dz) in diff {
            let (x, y, z) = (x + dx, y + dy, z + dz);
            if x < minx || x > maxx || y < miny || y > maxy || z < minz || z > maxz {
                continue;
            }
            if !map.contains(&(x, y, z)) && !air.contains(&(x, y, z)) {
                air.insert((x, y, z));
                moves.push_back((x, y, z));
            }
        }
    }

    // Find all squares bordering air
    for (x, y, z) in &map {
        let (x, y, z) = (*x, *y, *z);
        // Check all sides
        if air.contains(&(x + 1, y, z)) {
            count += 1;
        }
        if air.contains(&(x - 1, y, z)) {
            count += 1;
        }
        if air.contains(&(x, y + 1, z)) {
            count += 1;
        }
        if air.contains(&(x, y - 1, z)) {
            count += 1;
        }
        if air.contains(&(x, y, z + 1)) {
            count += 1;
        }
        if air.contains(&(x, y, z - 1)) {
            count += 1;
        }
    }

    dbg!(count);
}

fn parse_input(input: Str) -> Result<HashSet<(isize, isize, isize)>, Err<Error<Str>>> {
    let (_, v) = all_consuming(separated_list1(
        newline,
        tuple((
            terminated(map_res(digit1, str::parse), char(',')),
            terminated(map_res(digit1, str::parse), char(',')),
            map_res(digit1, str::parse),
        )),
    ))(input)?;

    Ok(HashSet::from_iter(v.into_iter()))
}
