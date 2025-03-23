use std::collections::{HashMap, HashSet};

use utils::{BasicGrid, Coord, Dir};

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Level(u8);

impl Level {
    fn inc(&self) -> Self {
        Level(self.0 + 1)
    }
}
impl From<u8> for Level {
    fn from(value: u8) -> Self {
        assert!(value > 47 && value < 58);
        Self(value - 48)
    }
}

struct Part1 {
    grid: BasicGrid<Level>,
}

impl Part1 {
    fn trace_from(&self, start: Coord) -> HashSet<Coord> {
        let this_level = self.grid.at(start);
        let mut nines: HashSet<Coord> = Default::default();
        if this_level.0 == 9 {
            nines.insert(start);
            return nines;
        }
        let target = this_level.inc();
        //println!("tracing from {:?} [{:?}]", start, target);
        for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
            if let Some(l) = self.grid.get(start, dir, 1) {
                if l == target {
                    let from_here = self.trace_from(start.in_dir(dir));
                    //println!("from {:?} in direction {:?} is {:?}", start, dir, from_here);
                    nines.extend(from_here);
                }
            };
        }
        //        println!("{} nines from {:?} at {:?}", nines.len(), this_level, start);
        nines
    }
}

struct Part2 {
    grid: BasicGrid<Level>,
}

impl Part2 {
    fn trace_from(&self, start: Coord) -> u16 {
        let this_level = self.grid.at(start);
        if this_level.0 == 9 {
            return 1;
        }
        let mut cnt = 0;
        let target = this_level.inc();
        //println!("tracing from {:?} [{:?}]", start, target);
        for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
            if let Some(l) = self.grid.get(start, dir, 1) {
                if l == target {
                    let from_here = self.trace_from(start.in_dir(dir));
                    //println!("from {:?} in direction {:?} is {:?}", start, dir, from_here);
                    cnt += from_here;
                }
            };
        }
        //        println!("{} nines from {:?} at {:?}", nines.len(), this_level, start);
        cnt
    }
}
fn main() {
    //let input: Vec<&str> = TEST.lines().collect();
    let input = std::fs::read_to_string("input/d10.txt").unwrap();
    let input: Vec<&str> = input.lines().collect();
    let grid: BasicGrid<Level> = BasicGrid::new(&input);
    let part2 = Part2 { grid };
    let starts = part2.grid.find_with(|v| *v == Level(0));
    let mut stats: HashMap<Coord, u16> = Default::default();

    for start in starts {
        stats.insert(start, part2.trace_from(start));
    }
    println!("{:?}", stats.into_values().sum::<u16>());
}

static TEST: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
