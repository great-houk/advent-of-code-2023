use std::{collections::HashMap, ops::Div};

use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::all_consuming,
    error::VerboseError,
    multi::{many1, separated_list1},
    Err,
};

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut platform = parse(input).unwrap();
    let mut inds = vec![0; platform[0].len()];
    let mut sum = 0;

    for row in 0..platform.len() {
        for col in 0..platform[row].len() {
            match platform[row][col] {
                '#' => inds[col] = row + 1,
                'O' => {
                    platform[row][col] = '.';
                    platform[inds[col]][col] = 'O';
                    sum += platform.len() - inds[col];
                    inds[col] += 1;
                }
                _ => (),
            }
        }
    }

    // for row in platform {
    //     for col in row {
    //         print!("{col}");
    //     }
    //     println!();
    // }

    println!("Sum: {sum}");
}

fn part2(input: &str) {
    let platform = parse(input).unwrap();
    let num_rocks: usize = platform
        .iter()
        .map(|c| c.iter().filter(|r| **r == 'O').count())
        .sum();
    let mut rocks = vec![(0, 0); num_rocks];
    let mut platforms = [platform.clone(), platform];
    let mut ind = 0;
    let mut hash = HashMap::new();
    // let target = 6;
    let target = 1_000_000_000;

    let mut cycle = 0;
    while cycle < target {
        cycle += 1;

        for _ in 0..4 {
            let mut inds = vec![0; platforms[ind][0].len()];
            let [platform, platform2] = if ind == 0 {
                let [one, two] = &mut platforms;
                [one, two]
            } else {
                let [one, two] = &mut platforms;
                [two, one]
            };
            ind += 1;
            ind %= 2;

            let mut rock = 0;
            for row in 0..platform.len() {
                for col in 0..platform[row].len() {
                    match platform[row][col] {
                        '#' => inds[col] = row + 1,
                        'O' => {
                            platform[row][col] = '.';
                            platform[inds[col]][col] = 'O';
                            platform2[col][platform.len() - row - 1] = '.';
                            platform2[col][platform.len() - inds[col] - 1] = 'O';

                            rocks[rock] = (col, platform.len() - inds[col] - 1);
                            rock += 1;

                            inds[col] += 1;
                        }
                        _ => (),
                    };
                    platform2[col][platform.len() - row - 1] = platform[row][col];
                }
            }
        }

        if let Some(c) = hash.get(&rocks) {
            // println!("{c} {cycle}");
            cycle += (cycle - c) * (target - cycle - 1).div(cycle - c);
        }
        hash.insert(rocks.clone(), cycle);
    }

    let mut sum = 0;
    for row in 0..platforms[ind].len() {
        for col in 0..platforms[ind].len() {
            match platforms[ind][row][col] {
                'O' => {
                    sum += platforms[ind].len() - row;
                }
                _ => (),
            }
        }
    }

    // for row in &platforms[ind] {
    //     for col in row {
    //         print!("{col}");
    //     }
    //     println!();
    // }
    println!("Sum: {sum}");
}

fn parse(input: &str) -> Result<Vec<Vec<char>>, Err<VerboseError<&str>>> {
    all_consuming(separated_list1(
        newline,
        many1(alt((char('.'), char('#'), char('O')))),
    ))(input)
    .map(|r| r.1)
}
