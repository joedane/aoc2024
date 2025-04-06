use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
struct Part1Game {
    a: (u64, u64),
    b: (u64, u64),
    total: (u64, u64),
}

impl Part1Game {
    fn new(a_0: u64, a_1: u64, b_0: u64, b_1: u64, prize_0: u64, prize_1: u64) -> Self {
        Part1Game {
            a: (a_0, a_1),
            b: (b_0, b_1),
            total: (prize_0, prize_1),
        }
    }
}
impl FromStr for Part1Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
        )
        .unwrap();
        let caps = re.captures(s).unwrap();
        Ok(Part1Game::new(
            caps.get(1).unwrap().as_str().parse().unwrap(),
            caps.get(2).unwrap().as_str().parse().unwrap(),
            caps.get(3).unwrap().as_str().parse().unwrap(),
            caps.get(4).unwrap().as_str().parse().unwrap(),
            caps.get(5).unwrap().as_str().parse().unwrap(),
            caps.get(6).unwrap().as_str().parse().unwrap(),
        ))
    }
}

struct MatchIter {
    a: u64,
    b: u64,
    target: u64,
    next_a: Option<u64>,
}

impl MatchIter {
    fn new(a: u64, b: u64, target: u64) -> Self {
        Self {
            a,
            b,
            target,
            next_a: Some(target / a),
        }
    }
}
impl Iterator for MatchIter {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        let ret_a = self.next_a;
        if let Some(next_a) = self.next_a {
            if next_a > self.a {
                self.next_a = Some(next_a - self.a);
            } else {
                self.next_a = None;
            }
        }
        ret.and_then(|a| Some(a))
    }
}

fn part1(part1: Part1Game) {
    let (a_x, a_y) = part1.a;
    let (b_x, b_y) = part1.b;
    let (total_x, total_y) = part1.total;
}

fn main() {
    let s: Part1Game = TEST.parse().unwrap();
    part1(s);
}

static TEST: &str = r#"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
"#;
