use std::io::{BufRead, BufReader, Lines, Read};

struct Report {
    levels: Box<[u32]>,
}

trait Safety {
    fn is_safe(r: &[u32]) -> bool;
}

struct Part1 {
    data: D2,
}

impl Part1 {
    fn run(self) -> String {
        let v: u32 = self
            .data
            .reports
            .iter()
            .filter(|r| Part1::is_safe(&r.levels))
            .map(|_| 1u32)
            .sum();

        format!("{}", v)
    }
}
impl Safety for Part1 {
    fn is_safe(levels: &[u32]) -> bool {
        if levels.len() < 2 {
            panic!()
        }
        if levels[0].abs_diff(levels[1]) == 0 || levels[0].abs_diff(levels[1]) > 3 {
            false
        } else if levels[0] < levels[1] {
            // increasing
            return levels[1..].windows(2).all(|w| {
                let (l, u): (i32, i32) = (w[0].try_into().unwrap(), w[1].try_into().unwrap());
                u - l > 0 && u - l <= 3
            });
        } else {
            // decreasing
            return levels[1..].windows(2).all(|w| {
                let (l, u): (i32, i32) = (w[0].try_into().unwrap(), w[1].try_into().unwrap());
                l - u > 0 && l - u <= 3
            });
        }
    }
}

struct Part2 {
    data: D2,
}

impl Safety for Part2 {
    fn is_safe(levels: &[u32]) -> bool {
        if Part1::is_safe(levels) {
            return true;
        }
        let mut v: Vec<u32> = vec![0; levels.len() - 1];
        for i in 0..levels.len() {
            v[0..i].copy_from_slice(&levels[0..i]);
            v[i..].copy_from_slice(&levels[i + 1..]);
            if Part1::is_safe(&v) {
                return true;
            }
        }

        false
    }
}

impl Part2 {
    fn run(self) -> String {
        let v: u32 = self
            .data
            .reports
            .iter()
            .filter(|r| Part2::is_safe(&r.levels))
            .map(|_| 1u32)
            .sum();

        format!("{}", v)
    }
}

struct D2 {
    reports: Box<[Report]>,
}

impl D2 {}

impl<T: std::io::BufRead> From<Lines<T>> for D2 {
    fn from(lines: Lines<T>) -> Self {
        let mut recs: Vec<Report> = vec![];
        for line in lines.map(|s| s.unwrap()) {
            let mut v: Vec<u32> = vec![];
            for tok in line.splitn(usize::MAX, " ") {
                v.push(tok.parse().expect(tok));
            }
            recs.push(Report {
                levels: v.into_boxed_slice(),
            });
        }
        D2 {
            reports: recs.into_boxed_slice(),
        }
    }
}

fn d2_input<T: Read>(r: T) -> std::io::BufReader<T> {
    BufReader::new(r)
}

pub fn main() {
    let d2: D2 = d2_input(std::fs::File::open("input/d2.txt").unwrap())
        .lines()
        .into();

    //let d2: D2 = d2_input(TEST.as_bytes()).lines().into();
    println!("{}", Part2 { data: d2 }.run());
}

static TEST: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
