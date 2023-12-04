use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{char, digit1, newline},
    combinator::{all_consuming, map_res},
    error::VerboseError,
    multi::{many1, separated_list1},
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
    let mut sum = 0;

    for (_, winning, nums) in input {
        let mut count = 0;
        for num in nums {
            if winning.contains(&num) {
                count += 1;
            }
        }
        if count != 0 {
            sum += 2u32.pow(count - 1);
        }
    }

    println!("Sum: {sum}");
}

fn part2(input: &str) {
    let input = parse(input).unwrap();
    let mut copies = vec![1; input.len()];

    for (id, winning, nums) in input {
        let mut count = 0;
        for num in nums {
            if winning.contains(&num) {
                count += 1;
            }
        }
        for i in 0..count {
            if count >= copies.len() {
                break;
            }

            copies[i + id as usize] += copies[id as usize - 1];
        }
    }

    let sum: u32 = copies.iter().sum();
    println!("Sum: {sum}");
}

fn parse(input: &str) -> Result<Vec<(u32, Vec<u32>, Vec<u32>)>, Err<VerboseError<&str>>> {
    all_consuming(separated_list1(
        newline,
        tuple((
            preceded(
                take_till(|c: char| c.is_numeric()),
                map_res(digit1, str::parse),
            ),
            preceded(
                take_till(|c: char| c.is_numeric()),
                terminated(
                    separated_list1(many1(char(' ')), map_res(digit1, str::parse)),
                    tag(" |"),
                ),
            ),
            preceded(
                many1(char(' ')),
                separated_list1(many1(char(' ')), map_res(digit1, str::parse)),
            ),
        )),
    ))(input)
    .map(|r| r.1)
}
