use std::{
    collections::HashSet,
    num::ParseIntError,
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

use itertools::Itertools;

type Str = &'static str;

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: Str) {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
    struct Coords {
        pub x: i32,
        pub y: i32,
    }

    impl Coords {
        fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }
    }

    impl Add for Coords {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Coords {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl AddAssign for Coords {
        fn add_assign(&mut self, rhs: Self) {
            self.x += rhs.x;
            self.y += rhs.y;
        }
    }

    impl Sub for Coords {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Coords {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }

    impl SubAssign for Coords {
        fn sub_assign(&mut self, rhs: Self) {
            self.x -= rhs.x;
            self.y -= rhs.y;
        }
    }

    impl FromStr for Coords {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (m, dist) = s
                .split_ascii_whitespace()
                .take(2)
                .collect_tuple()
                .ok_or(ParseError::Seperate)?;
            let dist: i32 = dist.parse()?;
            match m {
                "R" => Ok(Self { x: dist, y: 0 }),
                "L" => Ok(Self { x: -dist, y: 0 }),
                "U" => Ok(Self { x: 0, y: dist }),
                "D" => Ok(Self { x: 0, y: -dist }),
                _ => Err(ParseError::ParseDir),
            }
        }
    }

    let moves = input.lines().map(str::parse::<Coords>);
    let mut head = Coords::new(0, 0);
    let mut tail = Coords::new(0, 0);
    let mut visited = HashSet::new();
    let mut insert = |c| {
        visited.insert(c);
    };
    // Simulate
    for m in moves {
        insert(tail);
        head += m.unwrap();
        let diff = head - tail;
        let diffx = diff.x.signum();
        let diffy = diff.y.signum();
        let dist = diff.x.abs() + diff.y.abs();
        if dist > 2 {
            let newx;
            let newy;
            if diff.x.abs() > diff.y.abs() {
                newx = head.x - diffx;
                newy = head.y;
                // Set points
                let mut x = tail.x + diffx;
                while x != newx {
                    insert(Coords::new(x, newy));
                    x += diffx;
                }
            } else {
                newx = head.x;
                newy = head.y - diffy;
                // Set points
                let mut y = tail.y + diffy;
                while y != newy {
                    insert(Coords::new(newx, y));
                    y += diffy;
                }
            }
            // Set tail
            tail.x = newx;
            tail.y = newy;
        } else if dist == 2 {
            if diff.x.abs() > diff.y.abs() {
                tail.x = head.x - diffx;
            } else {
                tail.y = head.y - diffy;
            }
            insert(tail);
        }
    }
    // Done
    dbg!(visited.len());
}

fn part2(input: Str) {
    #[derive(Copy, Clone)]
    enum Move {
        Up(usize),
        Down(usize),
        Left(usize),
        Right(usize),
    }

    impl IntoIterator for Move {
        type IntoIter = MoveIter;
        type Item = (isize, isize);

        fn into_iter(self) -> Self::IntoIter {
            match self {
                Self::Up(dist) => MoveIter { dist, x: 0, y: 1 },
                Self::Down(dist) => MoveIter { dist, x: 0, y: -1 },
                Self::Right(dist) => MoveIter { dist, x: 1, y: 0 },
                Self::Left(dist) => MoveIter { dist, x: -1, y: 0 },
            }
        }
    }
    struct MoveIter {
        dist: usize,
        x: isize,
        y: isize,
    }

    impl Iterator for MoveIter {
        type Item = (isize, isize);

        fn next(&mut self) -> Option<Self::Item> {
            if self.dist == 0 {
                return None;
            }
            self.dist -= 1;
            Some((self.x, self.y))
        }
    }

    impl FromStr for Move {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (m, dist) = s
                .split_ascii_whitespace()
                .take(2)
                .collect_tuple()
                .ok_or(ParseError::Seperate)?;
            let dist: usize = dist.parse()?;
            match m {
                "R" => Ok(Self::Right(dist)),
                "L" => Ok(Self::Left(dist)),
                "U" => Ok(Self::Up(dist)),
                "D" => Ok(Self::Down(dist)),
                _ => Err(ParseError::ParseDir),
            }
        }
    }

    let moves = input
        .lines()
        .map(str::parse::<Move>)
        .map(Result::unwrap)
        .flatten();

    let mut rope = [(0, 0); 10];
    let mut visited = HashSet::new();

    for (x, y) in moves {
        rope[0].0 += x;
        rope[0].1 += y;

        for i in 1..rope.len() {
            let diffx = rope[i - 1].0 - rope[i].0;
            let diffy = rope[i - 1].1 - rope[i].1;
            if diffx.abs() > 1 || diffy.abs() > 1 {
                rope[i].0 += diffx.signum();
                rope[i].1 += diffy.signum();
            } else {
                break;
            }
        }

        visited.insert(rope[rope.len() - 1]);
    }

    // Print
    let xmin = rope
        .iter()
        .interleave(visited.iter())
        .map(|r| r.0)
        .min()
        .unwrap()
        - 3;
    let ymin = rope
        .iter()
        .interleave(visited.iter())
        .map(|r| r.1)
        .min()
        .unwrap()
        - 3;
    let xrange = (rope
        .iter()
        .interleave(visited.iter())
        .map(|r| r.0)
        .max()
        .unwrap()
        + 3
        - xmin) as usize;
    let yrange = (rope
        .iter()
        .interleave(visited.iter())
        .map(|r| r.1)
        .max()
        .unwrap()
        + 3
        - ymin) as usize;
    let mut grid = Vec::new();
    for _ in 0..xrange {
        let g = vec![1000; yrange as usize];
        grid.push(g);
    }
    for i in 0..rope.len() {
        if i < grid[(rope[i].0 - xmin) as usize][(rope[i].1 - ymin) as usize] {
            grid[(rope[i].0 - xmin) as usize][(rope[i].1 - ymin) as usize] = i;
        }
    }
    for (x, y) in &visited {
        if 11 < grid[(x - xmin) as usize][(y - ymin) as usize] {
            grid[(x - xmin) as usize][(y - ymin) as usize] = 999;
        }
    }
    for y in (0..yrange).rev() {
        for x in 0..xrange {
            if grid[x][y] == 1000 {
                print!(".");
            } else if grid[x][y] == 999 {
                print!("#");
            } else {
                print!("{}", grid[x][y]);
            }
        }
        println!();
    }
    // Print answer
    dbg!(visited.len());
}

#[derive(Debug, Clone)]
enum ParseError {
    ParseInt(ParseIntError),
    Seperate,
    ParseDir,
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}
