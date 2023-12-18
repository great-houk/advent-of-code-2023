use nom::{
    bytes::complete::take_till,
    character::complete::{char, newline},
    combinator::{all_consuming, map_res},
    error::VerboseError,
    multi::separated_list1,
    Err,
};

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let lines = parse(input).unwrap();
    let mut sum = 0;

    for line in lines {
        let mut level = 0;
        let len = line.len();
        let mut diffs = vec![line];

        loop {
            let mut zeros = true;
            level += 1;
            diffs.push(vec![0; len - level]);

            for i in 0..(diffs[level - 1].len() - 1) {
                diffs[level][i] = diffs[level - 1][i + 1] - diffs[level - 1][i];
                zeros &= diffs[level][i] == 0;
            }

            if zeros {
                diffs[level].push(0);
                break;
            }
        }

        for i in (0..level).rev() {
            let val = diffs[i][len - i - 1] + diffs[i + 1][len - i - 1];
            diffs[i].push(val);
        }

        sum += diffs[0][len];
    }

    println!("Sum: {sum}");
}

fn part2(input: &str) {
    let lines = parse(input).unwrap();
    let mut sum = 0;

    for line in lines {
        let mut level = 0;
        let len = line.len();
        let mut diffs = vec![line];

        loop {
            let mut zeros = true;
            level += 1;
            diffs.push(vec![0; len - level]);

            for i in 0..(diffs[level - 1].len() - 1) {
                diffs[level][i] = diffs[level - 1][i + 1] - diffs[level - 1][i];
                zeros &= diffs[level][i] == 0;
            }

            if zeros {
                diffs[level].push(0);
                break;
            }
        }

        for i in (0..level).rev() {
            let val = diffs[i][0] - diffs[i + 1][len - i - 1];
            diffs[i].push(val);
        }

        sum += diffs[0][len];
    }

    println!("Sum: {sum}");
}

fn parse(input: &str) -> Result<Vec<Vec<i32>>, Err<VerboseError<&str>>> {
    all_consuming(separated_list1(
        newline,
        separated_list1(
            char(' '),
            map_res(take_till(|c: char| c.is_whitespace()), str::parse),
        ),
    ))(input)
    .map(|r| r.1)
}
