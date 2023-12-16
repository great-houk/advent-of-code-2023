use nom::{
    bytes::complete::tag,
    character::complete::{anychar, digit1, newline},
    combinator::{all_consuming, map_res},
    error::VerboseError,
    multi::{many1, separated_list1},
    sequence::{preceded, tuple},
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

    let mut sum = 0;
    for (mut line, groups) in lines {
        let mut count = 0;

        // Find ?s
        let unknowns: Vec<_> = line
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, s)| *s == 2)
            .map(|(i, _)| i)
            .collect();

        // Iterate through all possible things
        for num in 0..(1 << unknowns.len()) {
            for (i, ind) in unknowns.iter().enumerate() {
                line[*ind] = (num & (1 << i)) >> i;
            }

            if is_valid(&line, &groups) {
                count += 1;
            }
        }

        println!("{count}");
        sum += count;
    }

    println!("Sum: {sum}");
}

fn is_valid(line: &Vec<usize>, groups: &Vec<usize>) -> bool {
    let mut ind = 0;
    let mut count = 0;

    for spot in line {
        if *spot == 1 {
            count += 1;
        } else if count != 0 && ind < groups.len() && count == groups[ind] {
            count = 0;
            ind += 1;
        } else if count != 0 {
            return false;
        }
    }

    return (ind == groups.len() && count == 0)
        || (ind == groups.len() - 1 && count == groups[ind]);
}

fn part2(input: &str) {
    let lines = parse(input).unwrap();
    let mut sum: usize = 0;

    for (mut line, mut groups) in lines {
        line.push(2);
        let copy = line.clone();
        for _ in 0..4 {
            line.extend_from_slice(&copy);
        }
        line.pop();

        let copy = groups.clone();
        for _ in 0..4 {
            groups.extend(&copy);
        }

        let mut inds = vec![0; groups.len()];
        let mut ind = 0;
        let mut counts: Vec<Vec<((usize, usize), usize)>> = vec![vec![]; inds.len()];
        let mut hash = HashMap::new();

        let check_valid = |ind, inds: &Vec<usize>| {
            let start = if ind == 0 {
                0
            } else {
                inds[ind - 1] + groups[ind - 1] + 1
            };
            let end = if ind == inds.len() - 1 {
                line.len()
            } else {
                inds[ind] + groups[ind] + 1
            };
            for i in start..end {
                if i >= inds[ind] && i < inds[ind] + groups[ind] {
                    if line[i] == 0 {
                        return false;
                    }
                } else if i < line.len() && line[i] == 1 {
                    return false;
                }
            }
            true
        };

        loop {
            if inds[ind] + groups[ind] > line.len() {
                if ind == 0 {
                    break;
                } else {
                    hash.extend(counts[ind].drain(..));
                    ind -= 1;
                    inds[ind] += 1;
                    continue;
                }
            }
            if !check_valid(ind, &inds) {
                inds[ind] += 1;
            } else {
                // Check hash
                if let Some(c) = hash.get(&(ind, inds[ind])) {
                    for level in &mut counts {
                        for ind in level {
                            ind.1 += c;
                        }
                    }

                    hash.extend(counts[ind].drain(..));
                    ind -= 1;
                    inds[ind] += 1;
                } else {
                    counts[ind].push(((ind, inds[ind]), 0));

                    if ind == inds.len() - 1 {
                        for level in &mut counts {
                            for ind in level {
                                ind.1 += 1;
                            }
                        }
                        inds[ind] += 1;
                    } else {
                        inds[ind + 1] = inds[ind] + groups[ind] + 1;
                        ind += 1;
                    }
                }
            }
        }

        let count = counts[0][0].1;
        // println!("{count}");
        sum += count;
    }

    println!("Sum: {sum}");
}

fn parse(input: &str) -> Result<Vec<(Vec<usize>, Vec<usize>)>, Err<VerboseError<&str>>> {
    all_consuming(separated_list1(
        newline,
        tuple((
            many1(map_res(anychar, |c| match c {
                '.' => Ok(0),
                '#' => Ok(1),
                '?' => Ok(2),
                _ => Err(format!("Unknown Char {c}")),
            })),
            preceded(
                tag(" "),
                separated_list1(tag(","), map_res(digit1, str::parse)),
            ),
        )),
    ))(input)
    .map(|r| r.1)
}
