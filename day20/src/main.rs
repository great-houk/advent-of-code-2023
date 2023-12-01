use iter_tools::Itertools;
use nom::{
    character::complete::char,
    character::complete::{digit1, newline},
    combinator::{all_consuming, map_res, opt, recognize},
    error::Error,
    multi::separated_list1,
    sequence::tuple,
    Err,
};

type Str = &'static str;

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: Str) {
    let arr = parse_input(input).unwrap();
    let mut indices = (0..arr.len()).collect_vec();
    let len = arr.len() as isize;

    // Mix
    for (i, mut off) in arr.iter().cloned().enumerate() {
        // Change current ind
        if off < 0 {
            off = (off % (len - 1)) + len - 1;
        }
        let init = indices[i];
        indices[i] += off as usize;
        if indices[i] >= arr.len() {
            indices[i] %= arr.len() - 1;
        }
        // Shift everything else
        for j in 0..arr.len() {
            if i == j {
                continue;
            }
            if indices[j] > init && indices[j] <= indices[i] {
                indices[j] -= 1;
            }
            if indices[j] < init && indices[j] >= indices[i] {
                indices[j] += 1;
            }
        }
    }
    // Get sum of 3 things
    let mut score = 0;
    let mut zero_ind = 0;
    for (i, v) in arr.iter().enumerate() {
        if *v == 0 {
            zero_ind = indices[i];
        }
    }
    for (i, ind) in indices.iter().enumerate() {
        if *ind == (zero_ind + 1000) % arr.len()
            || *ind == (zero_ind + 2000) % arr.len()
            || *ind == (zero_ind + 3000) % arr.len()
        {
            score += arr[i];
        }
    }
    dbg!(score);
}

fn part2(input: Str) {
    let arr = parse_input(input).unwrap();
    let arr = arr.into_iter().map(|v| v * 811589153).collect_vec();
    let mut indices = (0..arr.len()).collect_vec();
    let len = arr.len() as isize;

    // Mix 10 times
    for _ in 0..10 {
        for (i, mut off) in arr.iter().cloned().enumerate() {
            // Change current ind
            if off < 0 {
                off = (off % (len - 1)) + len - 1;
            }
            let init = indices[i];
            indices[i] += off as usize;
            if indices[i] >= arr.len() {
                indices[i] %= arr.len() - 1;
            }
            // Shift everything else
            for j in 0..arr.len() {
                if i == j {
                    continue;
                }
                if indices[j] > init && indices[j] <= indices[i] {
                    indices[j] -= 1;
                }
                if indices[j] < init && indices[j] >= indices[i] {
                    indices[j] += 1;
                }
            }
        }
        // // Print array
        // let mut p = vec![0; arr.len()];
        // for (i, ind) in indices.iter().enumerate() {
        //     p[*ind] = arr[i];
        // }
        // for i in p {
        //     print!("{}, ", i);
        // }
        // println!();
    }
    // Get sum of 3 things
    let mut score = 0;
    let mut zero_ind = 0;
    for (i, v) in arr.iter().enumerate() {
        if *v == 0 {
            zero_ind = indices[i];
        }
    }
    for (i, ind) in indices.iter().enumerate() {
        if *ind == (zero_ind + 1000) % arr.len()
            || *ind == (zero_ind + 2000) % arr.len()
            || *ind == (zero_ind + 3000) % arr.len()
        {
            score += arr[i];
        }
    }
    dbg!(score);
}

fn parse_input(input: Str) -> Result<Vec<isize>, Err<Error<Str>>> {
    let (_, v) = all_consuming(separated_list1(
        newline,
        map_res(recognize(tuple((opt(char('-')), digit1))), str::parse),
    ))(input)?;

    Ok(v)
}
