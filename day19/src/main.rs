#![feature(int_roundings)]
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{all_consuming, map_res, opt},
    error::Error,
    multi::many1,
    sequence::{preceded, terminated, tuple},
    Err,
};
use std::{cmp::max, collections::VecDeque};

type Str = &'static str;

fn main() {
    let input = include_str!("sample.txt");

    part1(input);
    part2(input);
}

fn part1(input: Str) {
    let blueprints = parse_input(input).unwrap();
    let mut score = 0;

    for blueprint in blueprints.iter() {
        let mut max_geodes = 0;
        let mut moves = VecDeque::new();
        let maxes = [0, 1, 2, 3].map(|i| blueprint.recipes.iter().map(|r| r[i]).max().unwrap());
        moves.push_back(([1, 0, 0, 0], [0, 0, 0, 0], 0));
        while let Some(m) = moves.pop_front() {
            // Check validity
            if max_geodes > (32 - m.2) * (32 - m.2 + m.0[3]) + m.1[3] {
                continue;
            }
            for (i, recipe) in blueprint.recipes.iter().enumerate() {
                if (i == 3 || m.0[i] < maxes[i])
                    && (recipe[0] == 0 || m.0[0] > 0)
                    && (recipe[1] == 0 || m.0[1] > 0)
                    && (recipe[2] == 0 || m.0[2] > 0)
                    && (recipe[3] == 0 || m.0[3] > 0)
                {
                    // Calc wait
                    let mut max_wait = 0;
                    for i in 0..4 {
                        if recipe[i] != 0 {
                            max_wait = max(
                                max_wait,
                                (recipe[i].saturating_sub(m.1[i])).div_ceil(m.0[i]),
                            );
                        }
                    }
                    max_wait += 1;
                    // Check if we are still in time
                    if m.2 + max_wait < 24 {
                        // Add resources
                        let mut robots = m.0.clone();
                        let mut ores = m.1.clone();
                        ores[0] += robots[0] * max_wait;
                        ores[1] += robots[1] * max_wait;
                        ores[2] += robots[2] * max_wait;
                        ores[3] += robots[3] * max_wait;
                        // Remove recipe resources
                        ores[0] -= recipe[0];
                        ores[1] -= recipe[1];
                        ores[2] -= recipe[2];
                        ores[3] -= recipe[3];
                        // Add robot
                        robots[i] += 1;
                        // Push move
                        moves.push_back((robots, ores, m.2 + max_wait));
                    } else {
                        // We're done, and can just wait.
                        // So count geodes and find max
                        let remaining = 24 - m.2;
                        let geodes = m.0[3] * remaining + m.1[3];
                        max_geodes = max(max_geodes, geodes);
                    }
                }
            }
        }
        // Mult score
        score += blueprint.id * max_geodes;
        // dbg!(blueprint.id, max_geodes);
    }

    dbg!(score);
}

fn part2(input: Str) {
    let blueprints = parse_input(input).unwrap();
    let blueprints = blueprints.into_iter().take(3).collect_vec();
    let mut score = 1;

    for blueprint in blueprints.iter() {
        let mut max_geodes = 0;
        let mut moves = VecDeque::new();
        let maxes = [0, 1, 2, 3].map(|i| blueprint.recipes.iter().map(|r| r[i]).max().unwrap());
        moves.push_back(([1, 0, 0, 0], [0, 0, 0, 0], 0));
        while let Some(m) = moves.pop_front() {
            // Check validity
            if max_geodes > (32 - m.2) * (32 - m.2 + m.0[3]) + m.1[3] {
                continue;
            }
            for (i, recipe) in blueprint.recipes.iter().enumerate() {
                if (i == 3 || m.0[i] < maxes[i])
                    && (recipe[0] == 0 || m.0[0] > 0)
                    && (recipe[1] == 0 || m.0[1] > 0)
                    && (recipe[2] == 0 || m.0[2] > 0)
                    && (recipe[3] == 0 || m.0[3] > 0)
                {
                    // Calc wait
                    let mut max_wait = 0;
                    for i in 0..4 {
                        if recipe[i] != 0 {
                            max_wait = max(
                                max_wait,
                                (recipe[i].saturating_sub(m.1[i])).div_ceil(m.0[i]),
                            );
                        }
                    }
                    max_wait += 1;
                    // Check if we are still in time
                    if m.2 + max_wait < 32 {
                        // Add resources
                        let mut robots = m.0.clone();
                        let mut ores = m.1.clone();
                        ores[0] += robots[0] * max_wait;
                        ores[1] += robots[1] * max_wait;
                        ores[2] += robots[2] * max_wait;
                        ores[3] += robots[3] * max_wait;
                        // Remove recipe resources
                        ores[0] -= recipe[0];
                        ores[1] -= recipe[1];
                        ores[2] -= recipe[2];
                        ores[3] -= recipe[3];
                        // Add robot
                        robots[i] += 1;
                        // Push move
                        moves.push_back((robots, ores, m.2 + max_wait));
                    } else {
                        // We're done, and can just wait.
                        // So count geodes and find max
                        let remaining = 32 - m.2;
                        let geodes = m.0[3] * remaining + m.1[3];
                        max_geodes = max(max_geodes, geodes);
                    }
                }
            }
        }
        // Mult score
        score *= max_geodes;
        // dbg!(blueprint.id, max_geodes);
    }

    dbg!(score);
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Blueprint {
    id: u16,
    recipes: [[u16; 4]; 4],
}

fn parse_input(input: Str) -> Result<Vec<Blueprint>, Err<Error<Str>>> {
    let (_, v) = all_consuming(many1(terminated(
        tuple((
            preceded(tag("Blueprint "), map_res(digit1, str::parse)),
            preceded(tag(": Each ore robot costs "), map_res(digit1, str::parse)),
            preceded(
                tag(" ore. Each clay robot costs "),
                map_res(digit1, str::parse),
            ),
            preceded(
                tag(" ore. Each obsidian robot costs "),
                map_res(digit1, str::parse),
            ),
            preceded(tag(" ore and "), map_res(digit1, str::parse)),
            preceded(
                tag(" clay. Each geode robot costs "),
                map_res(digit1, str::parse),
            ),
            preceded(tag(" ore and "), map_res(digit1, str::parse)),
        )),
        tuple((tag(" obsidian."), opt(newline))),
    )))(input)?;

    let mut prints = Vec::with_capacity(v.len());
    for (id, oco, cco, bco, bcc, gco, gcb) in v {
        prints.push(Blueprint {
            id,
            recipes: [
                [oco, 0, 0, 0],
                [cco, 0, 0, 0],
                [bco, bcc, 0, 0],
                [gco, 0, gcb, 0],
            ],
        });
    }

    Ok(prints)
}
