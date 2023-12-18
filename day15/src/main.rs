use nom::{
    bytes::complete::{take, take_till},
    character::complete::{anychar, char},
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
    let hashes = parse(input).unwrap();
    let mut sum = 0;

    for h in hashes {
        sum += hash(&h);
    }

    println!("Sum: {sum}");
}

fn hash(label: &[usize]) -> usize {
    let mut hash = 0;
    for c in label {
        hash += c;
        hash *= 17;
        hash &= 255;
    }
    hash
}

fn part2(input: &str) {
    const EQUALS: usize = '=' as usize;
    let steps = parse(input).unwrap();
    let mut boxes = vec![vec![]; 256];

    for step in steps {
        let (label, action) = match &step[..] {
            [label @ .., EQUALS, focal] => (label.to_owned(), *focal),
            [label @ .., sub] => (label.to_owned(), *sub),
            _ => panic!("Unknown format {step:?}"),
        };
        let hash = hash(&label[..]);

        if action == '-' as usize {
            boxes[hash].retain(|(l, _)| *l != label);
        } else {
            let mut found = false;
            for (l, f) in &mut boxes[hash] {
                if *l == label {
                    *f = action - '0' as usize;
                    found = true;
                }
            }
            if !found {
                boxes[hash].push((label, action - '0' as usize));
            }
        }
    }

    let mut focusing_power = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (j, (_, focal)) in b.iter().enumerate() {
            focusing_power += (1 + i) * (1 + j) * focal;
        }
    }

    println!("Power: {focusing_power}");
}

fn parse(input: &str) -> Result<Vec<Vec<usize>>, Err<VerboseError<&str>>> {
    all_consuming(separated_list1(
        char(','),
        many1(map_res(anychar, |c| {
            if c == ',' {
                Err("Comma")
            } else {
                Ok(c as usize)
            }
        })),
    ))(input)
    .map(|r| r.1)
}
