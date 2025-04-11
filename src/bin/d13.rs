use num_rational::{Ratio, Rational64};
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

trait CloseEnough {
    fn close_enough(self, other: Self) -> bool;
}

impl CloseEnough for f64 {
    fn close_enough(self, other: Self) -> bool {
        (self - other).abs() < 0.0001
    }
}

impl CloseEnough for Ratio<i64> {
    fn close_enough(self, other: Self) -> bool {
        self == other
    }
}
fn play_game(game: Part1Game) -> Option<u64> {
    let (a_x, a_y) = game.a;
    let (a_x, a_y): (Ratio<i64>, Ratio<i64>) = (
        Ratio::from_integer(a_x.try_into().unwrap()),
        Ratio::from_integer(a_y.try_into().unwrap()),
    );
    let (b_x, b_y) = game.b;
    let (b_x, b_y): (Ratio<i64>, Ratio<i64>) = (
        Ratio::from_integer(b_x.try_into().unwrap()),
        Ratio::from_integer(b_y.try_into().unwrap()),
    );
    let (total_x, total_y) = game.total;
    let (total_x, total_y): (Ratio<i64>, Ratio<i64>) = (
        Ratio::from_integer(total_x.try_into().unwrap()),
        Ratio::from_integer(total_y.try_into().unwrap()),
    );

    let R = a_x / a_y;
    let nb = (total_x - (R * total_y)) / (b_x - (R * b_y));
    let na = (total_y - nb * b_y) / a_y;
    if na.close_enough(na.round()) && nb.close_enough(nb.round()) {
        //println!("OK: {:?}", game);
        if !(na.is_integer() && nb.is_integer()) {
            panic!()
        } else {
            let na: u64 = na.to_integer().try_into().unwrap();
            let nb: u64 = nb.to_integer().try_into().unwrap();
            Some(3_u64 * na + nb)
        }
    } else {
        //println!("No (na: {}, nb: {}) {:?}", na, nb, game);
        None
    }
}

fn part1(part1: Part1Game) -> Option<u64> {
    play_game(part1)
}

fn part2(mut part2: Part1Game) -> Option<u64> {
    let factor = 10000000000000;
    part2.total.0 += factor;
    part2.total.1 += factor;
    play_game(part2)
}

fn main() {
    //let input = TEST;
    let input = std::fs::read_to_string("input/d13.txt").unwrap();
    let sum: u64 = input
        .trim()
        .split("\n\n")
        .map(|s| part2(s.parse().unwrap()).unwrap_or(0))
        .sum();
    println!("{sum}");
}

static TEST: &str = r#"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"#;
