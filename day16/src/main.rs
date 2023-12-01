use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{digit1, newline},
    combinator::{all_consuming, map, map_res},
    error::Error,
    multi::separated_list1,
    sequence::{preceded, tuple},
    Err,
};
use std::{
    cmp::max,
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
};

type Str = &'static str;

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: Str) {
    let map = parse_input(input).unwrap();

    let mut moves = VecDeque::from([(u16::from_ne_bytes([b'A', b'A']), HashSet::new(), 0, 0)]);
    let mut visited = HashMap::new();
    let mut max_score = 0;

    while let Some((valve, mut opened, mut score, mut turn)) = moves.pop_front() {
        // Add turn
        turn += 1;
        // Add score
        for valve in &opened {
            score += map.get(valve).as_ref().unwrap().rate;
        }
        // Only go for 30 min
        if turn >= 30 {
            max_score = max(score, max_score);
            continue;
        }
        // Check if we've been here before
        if let Some(value) = visited.get(&valve) {
            if *value > score + 20 {
                max_score = max(score, max_score);
                continue;
            } else {
                visited.insert(valve.clone(), score);
            }
        } else {
            visited.insert(valve.clone(), score);
        }
        // Move to new one
        for path in &map.get(&valve).unwrap().leads_to {
            moves.push_back((path.clone(), opened.clone(), score, turn));
        }
        // Open Valve
        if !opened.contains(&valve) {
            opened.insert(valve.clone());
            moves.push_back((valve.clone(), opened.clone(), score, turn));
        }
    }

    dbg!(max_score);
}

fn part2(input: Str) {
    let map = parse_input(input).unwrap();
    let mut max_score = 0;
    const NUM_TURNS: u8 = 26;
    #[derive(Clone, PartialEq, Eq)]
    struct State {
        opened: BTreeSet<u16>,
        score: u16,
        you: u16,
        elephant: u16,
        turn: u8,
    }

    #[derive(Copy, Clone, PartialEq, Eq)]
    enum Options {
        Open(u16),
        Move(u16),
    }

    let init = State {
        opened: BTreeSet::new(),
        score: 0,
        you: u16::from_ne_bytes([b'A', b'A']),
        elephant: u16::from_ne_bytes([b'A', b'A']),
        turn: 1,
    };
    let mut prev = HashMap::new();
    let mut moves = VecDeque::from([init]);
    let mut iters = 0;

    while let Some(state) = moves.pop_front() {
        iters += 1;
        // Change max score
        if state.score > max_score {
            max_score = state.score;
            // println!("{max_score}");
        }
        // Only go for NUM_TURNS turns
        if state.turn > NUM_TURNS {
            continue;
        }
        // Cull obvious bad ones
        let (len, max_rate): (_, u16) = {
            let i = map
                .iter()
                .filter(|(id, v)| v.rate != 0 && !state.opened.contains(id))
                .map(|(_, v)| v.rate)
                .collect_vec();
            (i.len() as u16, i.iter().sum())
        };
        if state.score + max_rate * ((NUM_TURNS - state.turn) as u16 - len + 2) < max_score {
            continue;
        }
        // If we've been here before, don't go again
        if let Some(turn) = prev.get(&(state.you, state.elephant, state.opened.clone())) {
            if *turn < state.turn {
                continue;
            }
        }
        // Add to prev
        prev.insert(
            (state.you, state.elephant, state.opened.clone()),
            state.turn,
        );
        // Make array of all you moves and elephant moves
        let mut yposs = Vec::with_capacity(map[&state.you].leads_to.len() + 1);
        // Don't open valves we've opened before, or ones with 0 rate
        if !state.opened.contains(&state.you) && map[&state.you].rate != 0 {
            yposs.push(Options::Open(state.you));
        }
        for n in &map[&state.you].leads_to {
            yposs.push(Options::Move(*n));
        }
        let mut eposs = Vec::with_capacity(map[&state.elephant].leads_to.len() + 1);
        if !state.opened.contains(&state.elephant) && map[&state.elephant].rate != 0 {
            eposs.push(Options::Open(state.elephant));
        }
        for n in &map[&state.elephant].leads_to {
            eposs.push(Options::Move(*n));
        }
        // Add through all possible combos
        for ymove in &yposs {
            for emove in &eposs {
                match (ymove, emove) {
                    (Options::Open(y), Options::Open(e)) => {
                        if *y == *e {
                            continue;
                        }
                        let mut s = state.clone();
                        s.turn += 1;
                        // Open y
                        s.score += map[y].rate * (NUM_TURNS as u16 - state.turn as u16);
                        s.opened.insert(*y);
                        // Open e
                        s.score += map[e].rate * (NUM_TURNS as u16 - state.turn as u16);
                        s.opened.insert(*e);
                        // Add move
                        moves.push_back(s);
                    }
                    (Options::Open(y), Options::Move(e)) => {
                        let mut s = state.clone();
                        s.turn += 1;
                        // Open y
                        s.score += map[y].rate * (NUM_TURNS as u16 - state.turn as u16);
                        s.opened.insert(*y);
                        // Move e
                        s.elephant = *e;
                        // Add move
                        moves.push_back(s);
                    }
                    (Options::Move(y), Options::Open(e)) => {
                        let mut s = state.clone();
                        s.turn += 1;
                        // Move y
                        s.you = *y;
                        // Open e
                        s.score += map[e].rate * (NUM_TURNS as u16 - state.turn as u16);
                        s.opened.insert(*e);
                        // Add move
                        moves.push_back(s);
                    }
                    (Options::Move(y), Options::Move(e)) => {
                        let mut s = state.clone();
                        s.turn += 1;
                        // Move y
                        s.you = *y;
                        // Move e
                        s.elephant = *e;
                        // Add move
                        moves.push_back(s);
                    }
                }
            }
        }
    }

    dbg!(max_score, iters);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Valve {
    rate: u16,
    leads_to: HashSet<u16>,
}

fn parse_input(input: Str) -> Result<HashMap<u16, Valve>, Err<Error<Str>>> {
    let (_, v) = all_consuming(separated_list1(
        newline,
        tuple((
            preceded(
                tag("Valve "),
                map(take(2usize), |s: Str| {
                    u16::from_ne_bytes(s.as_bytes().try_into().unwrap())
                }),
            ),
            preceded(tag(" has flow rate="), map_res(digit1, str::parse)),
            preceded(
                alt((
                    tag("; tunnels lead to valves "),
                    tag("; tunnel leads to valve "),
                )),
                separated_list1(
                    tag(", "),
                    map(take(2usize), |s: Str| {
                        u16::from_ne_bytes(s.as_bytes().try_into().unwrap())
                    }),
                ),
            ),
        )),
    ))(input)?;

    let mut map = HashMap::with_capacity(v.len());
    for (key, rate, leads_to) in v {
        map.insert(
            key,
            Valve {
                rate,
                leads_to: leads_to.into_iter().collect(),
            },
        );
    }

    Ok(map)
}
