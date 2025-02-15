#![allow(dead_code)]

use regex::Regex;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Lines, Read};

struct D1 {
    l1: Vec<u32>,
    l2: Vec<u32>,
}

impl D1 {
    fn _part1(mut self) -> String {
        self.l1.sort();
        self.l2.sort();

        let ans = self
            .l1
            .iter()
            .zip(self.l2.iter())
            .map(|(e1, e2)| e1.abs_diff(*e2))
            .reduce(|acc, diff| acc + diff)
            .unwrap();

        format!("{}", ans)
    }

    fn part2(self) -> String {
        let mut counts: HashMap<u32, usize> = Default::default();
        for s in self.l2.iter() {
            *counts.entry(*s).or_insert(0) += 1;
        }
        format!(
            "{}",
            self.l1
                .iter()
                .map(|v| *v as usize * counts.get(v).map_or(0, |v| *v))
                .sum::<usize>()
        )
    }
}
impl<T: std::io::BufRead> From<Lines<T>> for D1 {
    fn from(lines: Lines<T>) -> Self {
        let mut l1 = vec![];
        let mut l2 = vec![];
        let re = Regex::new(r"^(\d+)\D+(\d+)$").unwrap();
        for line in lines {
            let line = line.unwrap();
            let Some(caps) = re.captures(&line) else {
                panic!("no match: {}", line)
            };
            l1.push(caps[1].parse().expect(&line));
            l2.push(caps[2].parse().expect(&line));
        }
        D1 { l1, l2 }
    }
}

fn d1_input<T: Read>(r: T) -> std::io::BufReader<T> {
    BufReader::new(r)
}

fn main() {
    let d1: D1 = d1_input(std::fs::File::open("input/d1.txt").unwrap())
        .lines()
        .into();

    //let d1: D1 = d1_input(TEST.as_bytes()).lines().into();
    println!("{}", d1.part2());
}

static TEST: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
