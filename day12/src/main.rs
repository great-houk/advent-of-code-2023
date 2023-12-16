use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{anychar, digit1, newline},
    combinator::{all_consuming, map_res},
    error::VerboseError,
    multi::{many1, separated_list1},
    sequence::{preceded, tuple},
    Err,
};

fn main() {
    let input = include_str!("sample.txt");

    // part1(input);
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

        let mut hash = HashMap::new();
        let mut inds = vec![0; groups.len()];
        let mut ind = 0;
        let mut count = 0;

        'outer: loop {
            if inds[ind] > line.len() - groups[ind] {
                if ind == 0 {
                    break;
                } else {
                    if let Some((Some(c), None)) = hash.get(&(ind, inds[ind - 1])) {
                        hash.insert((ind, inds[ind - 1]), (Some(*c), Some(count)));
                    } else {
                        hash.insert((ind, inds[ind - 1]), (Some(count), None));
                    }

                    ind -= 1;
                    inds[ind] += 1;
                    continue;
                }
            }

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
                let valid = if i >= inds[ind] && i < inds[ind] + groups[ind] {
                    line[i] != 0
                } else {
                    i == line.len() || line[i] != 1
                };

                if !valid {
                    inds[ind] += 1;
                    continue 'outer;
                }
            }

            if ind == inds.len() - 1 {
                // let mut rep = vec!['.'; line.len()];
                // for i in 0..inds.len() {
                //     for j in 0..groups[i] {
                //         rep[inds[i] + j] = '#';
                //     }
                // }
                // for c in rep {
                //     print!("{c}");
                // }
                // println!();

                inds[ind] += 1;
                count += 1;

                if count % (1 << 20) == 0 {
                    // print!("Count: {count}, Inds: [");
                    // for ind in &inds {
                    //     print!("{ind}, ");
                    // }
                    // println!("]");
                }
            } else if ind != 0 {
                if let Some((Some(first), Some(second))) = hash.get(&(ind, inds[ind - 1])) {
                    let c = second - first;
                    count += c;

                    ind -= 1;
                    inds[ind] += 1;
                    // println!("Here!");
                } else {
                    inds[ind + 1] = inds[ind] + groups[ind] + 1;
                    ind += 1;
                }
            } else {
                inds[ind + 1] = inds[ind] + groups[ind] + 1;
                ind += 1;
            }
        }

        println!("{count}");
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

/*
for c in &line {
                    print!(
                        "{}",
                        match c {
                            0 => '.',
                            1 => '#',
                            _ => panic!("Ah!"),
                        }
                    );
                }
                println!(); */
