use nom::{
    bytes::complete::{tag, take},
    character::complete::{anychar, char, digit1, line_ending},
    combinator::{all_consuming, map, map_res},
    error::VerboseError,
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
    Err,
};
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let lines = parse(input).unwrap();
    let (mut xmax, mut ymax, mut xmin, mut ymin) = (0, 0, 0, 0);
    let _ = lines.iter().fold((0, 0), |(x, y), (dx, dy, _)| {
        xmax = xmax.max(x + dx);
        xmin = xmin.min(x + dx);
        ymax = ymax.max(y + dy);
        ymin = ymin.min(y + dy);
        (x + dx, y + dy)
    });
    let (xlen, ylen) = (xmax - xmin + 1, ymax - ymin + 1);
    let xy = move |x, y| (x + y * xlen) as usize;
    let mut grid = vec![0; (xlen * ylen) as usize];
    let mut coords = (-xmin, -ymin);

    for (dx, dy, _) in lines {
        let (x, y) = coords;
        for dx in 0.min(dx)..=0.max(dx) {
            for dy in 0.min(dy)..=0.max(dy) {
                grid[xy(x + dx, y + dy)] = 1;
            }
        }
        coords.0 += dx;
        coords.1 += dy;
    }

    let mut fill = vec![(-xmin + 1, -ymin + 1)];

    while let Some((x, y)) = fill.pop() {
        if grid[xy(x, y)] == 0 {
            grid[xy(x, y)] = 1;
            fill.push((x + 1, y));
            fill.push((x - 1, y));
            fill.push((x, y + 1));
            fill.push((x, y - 1));
        }
    }

    // for y in 0..ylen {
    //     for x in 0..xlen {
    //         print!("{}", if grid[xy(x, y)] == 1 { '#' } else { '.' });
    //     }
    //     println!();
    // }

    let count: isize = grid.into_iter().sum();
    println!("Count: {count}");
}

fn part2(input: &str) {
    let moves = parse(input).unwrap();
    let mut ends = Vec::with_capacity(moves.len() * 2);
    let mut coords = (0, 0);

    for m in moves {
        let m = match (m.2 & 0xF, m.2 >> 4) {
            (0, d) => (d as isize, 0),
            (1, d) => (0, d as isize),
            (2, d) => (-(d as isize), 0),
            (3, d) => (0, -(d as isize)),
            _ => panic!("Bad hex {}!", m.2),
        };

        if let (0, d) = m {
            ends.push((coords, d.is_positive()));
            ends.push(((coords.0 + d, coords.1), !d.is_positive()));
        }
        coords.0 += m.1;
        coords.1 += m.0;
    }

    ends.sort();

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    enum Dir {
        Down,
        Up,
        Vert,
    }

    let mut row = ends[0].0 .0;
    let mut flips = HashMap::new();
    let mut count = 0;
    let mut row_count = 0;

    for ((y, x), is_down) in ends {
        if y != row {
            let mut fs: Vec<_> = flips.iter().collect();
            fs.sort();

            // Add weird line
            let mut x = 0;
            let mut inside = false;
            let mut i = 0;
            while i < fs.len() {
                // Add empty space before switch
                if inside {
                    count += fs[i].0 - x;
                }
                count += 1;
                // Swap inside if neccesary
                if *fs[i].1 == Dir::Vert {
                    // Switch always
                    inside = !inside;
                    // Move on
                    x = fs[i].0 + 1;
                    i += 1;
                } else {
                    // Add line
                    count += fs[i + 1].0 - fs[i].0;
                    // Switch if they're not equal
                    inside ^= fs[i + 1].1 != fs[i].1;
                    // Move on
                    x = fs[i + 1].0 + 1;
                    i += 2;
                }
            }

            // Add inbetween
            x = 0;
            inside = false;
            row_count = 0;
            for flip in &fs {
                if *flip.1 == Dir::Up {
                    continue;
                }
                // Add line
                if inside {
                    row_count += flip.0 - x;
                }
                row_count += 1;
                x = flip.0 + 1;
                // Flip
                inside = !inside;
            }
            count += row_count * (y - row - 1);

            flips.retain(|_, dir| {
                let ret = *dir != Dir::Up;
                *dir = Dir::Vert;
                ret
            });
            row = y;
        }

        flips.insert(x, if is_down { Dir::Down } else { Dir::Up });
    }
    count += row_count;

    println!("Count: {count}");
}

fn parse(input: &str) -> Result<Vec<(isize, isize, u32)>, Err<VerboseError<&str>>> {
    all_consuming(separated_list1(
        line_ending,
        map(
            tuple((
                map_res(anychar, |c| {
                    Ok(match c {
                        'U' => (0, -1),
                        'D' => (0, 1),
                        'L' => (-1, 0),
                        'R' => (1, 0),
                        _ => return Err(format!("Unexpected char {c}")),
                    })
                }),
                preceded(char(' '), map_res(digit1, str::parse::<isize>)),
                terminated(
                    preceded(
                        tag(" (#"),
                        map_res(take(6usize), |n| u32::from_str_radix(n, 16)),
                    ),
                    char(')'),
                ),
            )),
            |((dx, dy), d, color)| (dx * d, dy * d, color),
        ),
    ))(input)
    .map(|r| r.1)
}
