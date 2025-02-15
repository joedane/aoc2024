#![allow(dead_code)]

use std::str::FromStr;

struct Item {
    total: u64,
    components: Vec<u16>,
}

fn check_part1(total: u64, acc: u64, items: &[u16]) -> bool {
    if acc > total {
        return false;
    } else if items.len() == 0 {
        return total == acc;
    } else {
        if check_part1(total, acc + items[0] as u64, &items[1..]) {
            return true;
        } else {
            return check_part1(total, acc * items[0] as u64, &items[1..]);
        }
    }
}

fn check_part2(total: u64, acc: u64, items: &[u16]) -> bool {
    if acc > total {
        return false;
    } else if items.len() == 0 {
        return total == acc;
    } else {
        if check_part2(total, acc + items[0] as u64, &items[1..]) {
            return true;
        } else if check_part2(total, acc * items[0] as u64, &items[1..]) {
            return true;
        } else {
            let n = items[0].ilog10();
            return check_part2(
                total,
                acc * 10_u64.pow(n + 1) + items[0] as u64,
                &items[1..],
            );
        }
    }
}

#[derive(Debug)]
enum DaySevenError {
    Parse(String),
}

impl DaySevenError {
    fn parse_error(msg: String) -> Self {
        Self::Parse(msg)
    }
}
impl FromStr for Item {
    type Err = DaySevenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(i) = s.find(':') {
            let total: u64 = s[0..i]
                .parse()
                .map_err(|_| DaySevenError::parse_error(s.to_string()))
                .unwrap();
            let components = s[i + 1..]
                .split_whitespace()
                .map(|s| {
                    s.parse::<u16>()
                        .map_err(|_| DaySevenError::parse_error(format!("2: {}", s)))
                })
                .collect::<Result<Vec<u16>, DaySevenError>>()
                .unwrap();

            Ok(Item { total, components })
        } else {
            Err(DaySevenError::parse_error(s.to_string()))
        }
    }
}

fn main() {
    let data = std::fs::read_to_string("input/d7.txt").unwrap();
    //let data = TEST;
    let check = check_part2;
    let ans: u64 = data
        .lines()
        .map(|s| s.trim().parse::<Item>().unwrap())
        .filter_map(|i| {
            let v: Option<u64> = if check(i.total, i.components[0] as u64, &i.components[1..]) {
                Some(i.total)
            } else {
                None
            };
            v
        })
        .map(|v| v as u64)
        .sum();
    println!("{}", ans);
}

static TEST: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
