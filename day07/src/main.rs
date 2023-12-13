use itertools::Itertools;
use nom::{
    bytes::complete::take,
    character::complete::{char, digit1, newline},
    combinator::{all_consuming, map, map_res},
    error::VerboseError,
    multi::separated_list1,
    sequence::{preceded, tuple},
    Err,
};

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut hands = parse(input)
        .unwrap()
        .into_iter()
        .map(|(hand, bid)| Hand::new(hand, bid))
        .collect_vec();
    hands.sort();
    let sum: usize = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum();
    println!("Sum: {sum}");
}

fn part2(input: &str) {
    let mut hands = parse(input)
        .unwrap()
        .into_iter()
        .map(|(hand, bid)| Hand2::new(hand, bid))
        .collect_vec();
    // for (chars, hand) in hands {
    //     println!(
    //         "{:?} {} {:?}",
    //         hand.kind,
    //         chars.iter().collect::<String>(),
    //         hand.cards
    //     );
    // }
    hands.sort();
    let sum: usize = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum();
    println!("Sum: {sum}");
}

fn parse(input: &str) -> Result<Vec<([char; 5], usize)>, Err<VerboseError<&str>>> {
    all_consuming(separated_list1(
        newline,
        map(
            tuple((
                take(5usize),
                preceded(char(' '), map_res(digit1, str::parse)),
            )),
            |(cards, bid)| {
                let cards = cards.chars().collect_vec().try_into().unwrap();
                (cards, bid)
            },
        ),
    ))(input)
    .map(|r| r.1)
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    High,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    kind: Kind,
    cards: [Card; 5],
    bid: usize,
}

impl Hand {
    pub fn new(input: [char; 5], bid: usize) -> Self {
        let cards: [Card; 5] = input
            .map(|c| match c {
                '1'..='9' => c as u8 - '1' as u8,
                'T' => 9,
                'J' => 10,
                'Q' => 11,
                'K' => 12,
                'A' => 13,
                _ => panic!("Invalid Input! {c}"),
            })
            .map(|u| unsafe { std::mem::transmute(u) });
        let kind = Self::get_kind(&cards);
        Self { kind, cards, bid }
    }

    pub fn get_kind(cards: &[Card; 5]) -> Kind {
        let mut type_counts = [0; 14];
        for &card in cards {
            type_counts[card as usize] += 1;
        }
        let mut counts = [0; 6];
        for count in type_counts {
            counts[count] += 1;
        }

        if counts[5] == 1 {
            Kind::Five
        } else if counts[4] == 1 {
            Kind::Four
        } else if counts[3] == 1 && counts[2] == 1 {
            Kind::Full
        } else if counts[3] == 1 {
            Kind::Three
        } else if counts[2] == 2 {
            Kind::Two
        } else if counts[2] == 1 {
            Kind::One
        } else {
            Kind::High
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand2 {
    kind: Kind,
    cards: [Card2; 5],
    bid: usize,
}

impl Hand2 {
    pub fn new(input: [char; 5], bid: usize) -> Self {
        let cards: [Card2; 5] = input
            .map(|c| match c {
                'J' => 0,
                '1'..='9' => c as u8 - '1' as u8 + 1,
                'T' => 10,
                'Q' => 11,
                'K' => 12,
                'A' => 13,
                _ => panic!("Invalid Input! {c}"),
            })
            .map(|u| unsafe { std::mem::transmute(u) });
        let kind = Self::get_kind(&cards);
        Self { kind, cards, bid }
    }

    pub fn get_kind(cards: &[Card2; 5]) -> Kind {
        let mut type_counts = [0; 14];
        let mut j_count = 0;
        for &card in cards {
            if card != Card2::Joker {
                type_counts[card as usize] += 1;
            } else {
                j_count += 1;
            }
        }
        let mut counts = [0; 6];
        for count in type_counts {
            counts[count] += 1;
        }

        if counts[5 - j_count] >= 1 || j_count == 5 {
            Kind::Five
        } else if counts[4 - j_count] >= 1 {
            Kind::Four
        } else if match (j_count, counts[3], counts[2]) {
            (0, a, b) if a == 1 && b == 1 => true,
            (1, _, b) if b >= 2 => true,
            _ => false,
        } {
            Kind::Full
        } else if counts[3 - j_count] >= 1 {
            Kind::Three
        } else if counts[2] >= 2 {
            Kind::Two
        } else if counts[2 - j_count] >= 1 {
            Kind::One
        } else {
            Kind::High
        }
    }
}

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card2 {
    Joker,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}
