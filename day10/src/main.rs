use nom::{
    character::complete::{anychar, newline},
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
    let tiles = parse(input).unwrap();
    let mut start = None;
    let mut tiles: Vec<Vec<(Tile, Option<u32>)>> = tiles
        .into_iter()
        .enumerate()
        .map(|(i, t)| {
            t.into_iter()
                .enumerate()
                .map(|(j, t)| {
                    if t == Tile::Start {
                        start = Some((i, j));
                        (t, Some(0))
                    } else {
                        (t, None)
                    }
                })
                .collect()
        })
        .collect();
    let mut positions = vec![start.unwrap()];

    let mut max = 0;
    while let Some(pos) = positions.pop() {
        let (i, j) = pos;
        let steps = tiles[i][j].1.unwrap();
        let len = tiles.len();
        let len2 = tiles[0].len();

        let (left, right, up, down) = match tiles[i][j].0 {
            Tile::Vertical => (false, false, true, true),
            Tile::Horizontal => (true, true, false, false),
            Tile::NorthEast => (false, true, true, false),
            Tile::NorthWest => (true, false, true, false),
            Tile::SouthWest => (true, false, false, true),
            Tile::SouthEast => (false, true, false, true),
            Tile::Ground => (false, false, false, false),
            Tile::Start => (true, true, true, true),
        };

        let conds: &[(_, &dyn Fn(usize, usize) -> bool, _)] = &[
            (up, &|i, _| i > 0, (0, 1)),
            (down, &|i, _| i < len - 1, (2, 1)),
            (left, &|_, j| j > 0, (1, 0)),
            (right, &|_, j| j < len2 - 1, (1, 2)),
        ];

        for cond in conds {
            if cond.0 && cond.1(i, j) {
                let (x, y) = (i + cond.2 .0 - 1, j + cond.2 .1 - 1);

                if tiles[x][y].0 == Tile::Start {
                    max = steps + 1;
                } else if tiles[x][y].1.map(|s| steps - 1 > s).unwrap_or(true) {
                    tiles[x][y].1 = Some(steps + 1);
                    positions.push((x, y));
                }
            }
        }
    }
    max /= 2;

    // for line in tiles {
    //     for t in line {
    //         print!(
    //             "{: <3}",
    //             t.1.map(|i| i.to_string()).unwrap_or(".".to_string())
    //         );
    //     }
    //     println!();
    // }

    println!("Max: {max}");
}

fn part2(input: &str) {
    let tiles = parse(input).unwrap();
    let mut start = None;
    let mut tiles: Vec<Vec<(Tile, Option<u32>)>> = tiles
        .into_iter()
        .enumerate()
        .map(|(i, t)| {
            t.into_iter()
                .enumerate()
                .map(|(j, t)| {
                    if t == Tile::Start {
                        start = Some((i, j));
                        (t, Some(0))
                    } else {
                        (t, None)
                    }
                })
                .collect()
        })
        .collect();
    let mut positions = vec![start.unwrap()];

    let mut end = (0, 0);
    while let Some(pos) = positions.pop() {
        let (i, j) = pos;
        let steps = tiles[i][j].1.unwrap();
        let len = tiles.len();
        let len2 = tiles[0].len();

        let (left, right, up, down) = match tiles[i][j].0 {
            Tile::Vertical => (false, false, true, true),
            Tile::Horizontal => (true, true, false, false),
            Tile::NorthEast => (false, true, true, false),
            Tile::NorthWest => (true, false, true, false),
            Tile::SouthWest => (true, false, false, true),
            Tile::SouthEast => (false, true, false, true),
            Tile::Ground => (false, false, false, false),
            Tile::Start => (true, true, true, true),
        };

        let conds: &[(_, &dyn Fn(usize, usize) -> bool, _)] = &[
            (up, &|i, _| i > 0, (0, 1)),
            (down, &|i, _| i < len - 1, (2, 1)),
            (left, &|_, j| j > 0, (1, 0)),
            (right, &|_, j| j < len2 - 1, (1, 2)),
        ];

        for cond in conds {
            if cond.0 && cond.1(i, j) {
                let (x, y) = (i + cond.2 .0 - 1, j + cond.2 .1 - 1);

                if tiles[x][y].0 == Tile::Start {
                    end = (i, j);
                } else if tiles[x][y].1.map(|s| steps - 1 > s).unwrap_or(true) {
                    tiles[x][y].1 = Some(steps + 1);
                    positions.push((x, y));
                }
            }
        }
    }

    let mut path = vec![vec![Tile::Ground; tiles[0].len()]; tiles.len()];

    let mut curr = end;
    let mut last = start.unwrap();

    while tiles[curr.0][curr.1].0 != Tile::Start {
        path[curr.0][curr.1] = tiles[curr.0][curr.1].0;
        let temp = curr;

        match tiles[curr.0][curr.1].0 {
            Tile::Vertical => curr.0 = curr.0 + curr.0 - last.0,
            Tile::Horizontal => curr.1 = curr.1 + curr.1 - last.1,
            Tile::NorthEast | Tile::SouthWest => {
                curr.1 = curr.1 + temp.0 - last.0;
                curr.0 = curr.0 + temp.1 - last.1;
            }
            Tile::NorthWest | Tile::SouthEast => {
                curr.1 = curr.1 + last.0 - temp.0;
                curr.0 = curr.0 + last.1 - temp.1;
            }
            _ => panic!("What????"),
        }

        last = temp;
    }

    let (y, x) = start.unwrap();
    path[y][x] = match (
        (1 + y - last.0, 1 + y - end.0),
        (1 + x - last.1, 1 + x - end.1),
    ) {
        (_, (1, 1)) => Tile::Vertical,
        ((1, 1), _) => Tile::Horizontal,
        ((2, 1), (1, 2)) | ((1, 2), (2, 1)) => Tile::NorthWest,
        ((2, 1), (1, 0)) | ((1, 2), (0, 1)) => Tile::NorthEast,
        ((0, 1), (1, 2)) | ((1, 0), (2, 1)) => Tile::SouthWest,
        ((0, 1), (1, 0)) | ((1, 0), (0, 1)) => Tile::SouthEast,
        _ => panic!("What????? 2"),
    };

    let mut count = 0;
    for row in &mut path {
        let mut inside = false;
        let mut north: Option<bool> = None;
        for tile in row {
            if *tile == Tile::Ground && inside {
                count += 1;
                *tile = Tile::Start;
            }
            inside = match (*tile, north) {
                (Tile::Vertical, _) => !inside,
                (Tile::NorthWest | Tile::NorthEast, Some(n)) => {
                    north = None;
                    inside ^ !n
                }
                (Tile::NorthWest | Tile::NorthEast, None) => {
                    north = Some(true);
                    inside
                }
                (Tile::SouthWest | Tile::SouthEast, Some(n)) => {
                    north = None;
                    inside ^ n
                }
                (Tile::SouthWest | Tile::SouthEast, None) => {
                    north = Some(false);
                    inside
                }
                _ => inside,
            }
        }
    }

    // for line in path {
    //     for t in line {
    //         print!("{}", t as u8 as char);
    //     }
    //     println!();
    // }

    println!("Count: {count}");
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Vertical = '|' as u8,
    Horizontal = '-' as u8,
    NorthEast = 'L' as u8,
    NorthWest = 'J' as u8,
    SouthWest = '7' as u8,
    SouthEast = 'F' as u8,
    Ground = '.' as u8,
    Start = 'S' as u8,
}

fn parse(input: &str) -> Result<Vec<Vec<Tile>>, Err<VerboseError<&str>>> {
    all_consuming(separated_list1(
        newline,
        many1(map_res(anychar, |c| {
            Ok(match c {
                '|' => Tile::Vertical,
                '-' => Tile::Horizontal,
                'L' => Tile::NorthEast,
                'J' => Tile::NorthWest,
                '7' => Tile::SouthWest,
                'F' => Tile::SouthEast,
                '.' => Tile::Ground,
                'S' => Tile::Start,
                _ => return Err(format!("Unexpected char {c}")),
            })
        })),
    ))(input)
    .map(|r| r.1)
}
