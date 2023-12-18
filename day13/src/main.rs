use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline},
    combinator::{all_consuming, map_res},
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
    let blocks = parse(input).unwrap();
    let mut sum = 0;

    for block in blocks {
        // Check horizontal
        let mut mirror = false;
        let mut row = 0;
        'outer: for i in 1..block.len() {
            for j in 0..i.min(block.len() - i) {
                if block[i + j] != block[i - j - 1] {
                    continue 'outer;
                }
            }
            mirror = true;
            row = i;
            break;
        }
        if mirror {
            sum += 100 * row;
            continue;
        }

        // Check vertical
        mirror = false;
        let mut col = 0;
        let len = block[0].len();
        'outer: for i in 1..len {
            for j in 0..i.min(len - i) {
                for k in 0..block.len() {
                    if block[k][i + j] != block[k][i - j - 1] {
                        continue 'outer;
                    }
                }
            }
            mirror = true;
            col = i;
            break;
        }
        if mirror {
            sum += col;
        }
    }

    println!("Sum: {sum}");
}

fn part2(input: &str) {
    let blocks = parse(input).unwrap();
    let mut sum = 0;

    for block in blocks {
        // Check horizontal
        let mut mirror = false;
        let mut row = 0;
        let len = block.len();
        'outer: for i in 1..len {
            let mut diff = 0;
            for j in 0..i.min(len - i) {
                for k in 0..block[0].len() {
                    if block[i + j][k] != block[i - j - 1][k] {
                        if diff > 1 {
                            continue 'outer;
                        } else {
                            diff += 1;
                        }
                    }
                }
            }
            if diff == 1 {
                mirror = true;
                row = i;
                break;
            }
        }
        if mirror {
            sum += 100 * row;
        }

        // Check vertical
        mirror = false;
        let mut col = 0;
        let len = block[0].len();
        'outer: for i in 1..len {
            let mut diff = 0;
            for j in 0..i.min(len - i) {
                for k in 0..block.len() {
                    if block[k][i + j] != block[k][i - j - 1] {
                        if diff > 1 {
                            continue 'outer;
                        } else {
                            diff += 1;
                        }
                    }
                }
            }
            if diff == 1 {
                mirror = true;
                col = i;
                break;
            }
        }
        if mirror {
            sum += col;
        }
    }

    println!("Sum: {sum}");
}

fn parse(input: &str) -> Result<Vec<Vec<Vec<u8>>>, Err<VerboseError<&str>>> {
    all_consuming(separated_list1(
        tag("\n\n"),
        separated_list1(
            newline,
            many1(map_res(alt((char('.'), char('#'))), |c| {
                Ok(match c {
                    '.' => 0,
                    '#' => 1,
                    _ => return Err(format!("Unknown char {c}")),
                })
            })),
        ),
    ))(input)
    .map(|r| r.1)
}
