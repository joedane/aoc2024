use std::{collections::HashMap, str::FromStr};

use regex::Regex;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: i16,
    y: i16,
}

impl Coord {
    fn new(x: i16, y: i16) -> Self {
        Coord { x, y }
    }
}
#[derive(Debug)]
struct Bot {
    p: Coord,
    v: Coord,
}

impl Bot {
    fn new(p_x: i16, p_y: i16, v_x: i16, v_y: i16) -> Self {
        Self {
            p: Coord::new(p_x, p_y),
            v: Coord::new(v_x, v_y),
        }
    }
}

impl FromStr for Bot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        Ok(Bot::new(
            caps.get(1).unwrap().as_str().parse().unwrap(),
            caps.get(2).unwrap().as_str().parse().unwrap(),
            caps.get(3).unwrap().as_str().parse().unwrap(),
            caps.get(4).unwrap().as_str().parse().unwrap(),
        ))
    }
}
fn main() {
    let input = std::fs::read_to_string("input/d14.txt").unwrap();
    //let input = TEST;
    let (width, height) = (101_i16, 103_i16);
    let mut bots: Vec<Bot> = input
        .lines()
        .map(str::trim)
        .map(|s| s.parse().unwrap())
        .collect();

    for _ in 0..100 {
        bots.iter_mut().for_each(|b| {
            b.p.x = (b.p.x + b.v.x).rem_euclid(width);
            b.p.y = (b.p.y + b.v.y).rem_euclid(height);
        });
    }
    let mut map: HashMap<(bool, bool), usize> = Default::default();
    let (half_w, half_h) = (width / 2, height / 2);
    //println!("half_w: {half_w}\thalf_h: {half_h}");
    for bot in bots.iter() {
        let (x, y) = (bot.p.x, bot.p.y);
        let q: Option<(bool, bool)> = if x < half_w {
            if y < half_h {
                Some((true, true))
            } else if y > half_h {
                Some((true, false))
            } else {
                None
            }
        } else if x > half_w {
            if y < half_h {
                Some((false, true))
            } else if y > half_h {
                Some((false, false))
            } else {
                None
            }
        } else {
            None
        };
        //println!("bot {:?} in quad {:?}", bot, q);
        if let Some(q) = q {
            *map.entry(q).or_default() += 1;
        }
    }
    println!("{}", map.values().fold(1, |acc, v| acc * v));
}

static TEST: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
