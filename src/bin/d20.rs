use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use itertools::Itertools;
use utils::{BasicGrid, Coord, Dir};

#[derive(Debug, Clone, Copy)]
enum GridPoint {
    Wall,
    Open,
    Start,
    End,
}

impl From<u8> for GridPoint {
    fn from(value: u8) -> Self {
        match value {
            b'#' => Self::Wall,
            b'.' => Self::Open,
            b'S' => Self::Start,
            b'E' => Self::End,
            _ => panic!(),
        }
    }
}

fn trace_path(grid: &BasicGrid<GridPoint>) -> (Vec<Coord>, HashMap<Coord, usize>) {
    let mut steps: HashMap<Coord, usize> = Default::default();

    let mut c = grid.find_with(|point| matches!(point, GridPoint::Start));

    let start = c.pop().unwrap();

    let mut c = grid.find_with(|point| matches!(point, GridPoint::End));
    let end = c.pop().unwrap();
    let mut at = start;

    let mut dir: Option<Dir> = None;
    for d in [Dir::Up, Dir::Left, Dir::Down, Dir::Right] {
        if let Some(next_pos) = grid.next_pos(start, d) {
            match grid.at(next_pos) {
                GridPoint::Wall => continue,
                GridPoint::Open => {
                    if dir.is_some() {
                        panic!();
                    } else {
                        dir = Some(d);
                    }
                }
                _ => panic!(),
            }
        }
    }

    let mut dir = dir.unwrap();
    let mut step: usize = 0;
    let mut r = vec![];
    r.push(at);
    while at != end {
        //        println!("at: {:?}", at);
        if let Some(next_pos) = grid.next_pos(at, dir) {
            if matches!(grid.at(next_pos), GridPoint::Open | GridPoint::End) {
                /*
                 *  check for forks
                 */
                if grid
                    .next_pos(at, dir.turn_left())
                    .is_some_and(|c| !r.contains(&c) && matches!(grid.at(c), GridPoint::Open))
                    || grid
                        .next_pos(at, dir.turn_right())
                        .is_some_and(|c| !r.contains(&c) && matches!(grid.at(c), GridPoint::Open))
                {
                    panic!();
                }
                r.push(next_pos);
                steps.insert(at, step);
                at = next_pos;
                step += 1;
                continue;
            }
        }

        let mut new_dir: Option<Dir> = None;
        for d in [dir.turn_left(), dir.turn_right()] {
            if let Some(next_pos) = grid.next_pos(at, d) {
                match grid.at(next_pos) {
                    GridPoint::Wall => continue,
                    GridPoint::Open | GridPoint::End => {
                        if new_dir.is_none() {
                            new_dir = Some(d);
                        } else {
                            panic!();
                        }
                    }
                    GridPoint::Start => panic!(),
                }
            }
        }
        if new_dir.is_none() {
            panic!();
        } else {
            dir = new_dir.unwrap();
        }
    }
    steps.insert(at, step);
    (r, steps)
}

fn possible_jumps<T>(grid: &BasicGrid<T>, from: Coord, dist: usize) -> Vec<Coord> {
    let mut targets: HashSet<Coord> = Default::default();
    let mut visited: HashSet<Coord> = Default::default();
    let mut to_visit: Vec<(Coord, usize)> = Default::default();
    to_visit.push((from, dist));
    while let Some((c, dist)) = to_visit.pop() {
        targets.insert(c);

        if dist > 0 {
            for d in [Dir::Up, Dir::Left, Dir::Down, Dir::Right] {
                if let Some(pos) = grid.next_pos(c, d) {
                    if !targets.contains(&pos) {
                        to_visit.push((pos, dist - 1));
                    }
                }
            }
        }
    }
    targets.retain(|&c| c != from);
    targets.into_iter().collect()
}

fn process_part1(data: String, dist: usize, min_savings: usize) {
    let mut input: Vec<&str> = vec![];
    for line in data.lines().map(str::trim) {
        input.push(line);
    }
    let grid: BasicGrid<GridPoint> = BasicGrid::new(&input);

    let (path, steps) = trace_path(&grid);
    let path_len = path.len();
    /*
    for p in &path {
        println!("{:?} => {}", p, steps.get(p).unwrap());
    }
    */
    let mut save_hash: HashMap<usize, usize> = Default::default();

    for p in &path {
        let this_step = *steps.get(p).unwrap();
        for pos in possible_jumps(&grid, *p, dist) {
            if let Some(&step) = steps.get(&pos) {
                if step > this_step + 2 {
                    let savings = step - this_step - 2;
                    if savings >= min_savings {
                        save_hash
                            .entry(savings)
                            .and_modify(|c| *c += 1)
                            .or_insert(1);
                        /*
                        println!(
                            "jumping from pos {:?} to {:?} will save {} steps",
                            p, pos, savings
                        );
                        */
                    }
                }
            }
        }
    }

    println!("{:?}", save_hash.len());
    println!("{:?}", save_hash);
    println!("{}", save_hash.iter().map(|(k, v)| v).sum::<usize>());
}

fn manhattan_len(c1: Coord, c2: Coord) -> usize {
    c1.col.abs_diff(c2.col) + c1.row.abs_diff(c2.row)
}

fn process_part2(data: String, dist: usize, min_savings: usize) {
    let mut input: Vec<&str> = vec![];
    for line in data.lines().map(str::trim) {
        input.push(line);
    }
    let grid: BasicGrid<GridPoint> = BasicGrid::new(&input);

    let (path, steps) = trace_path(&grid);
    let path_len = path.len();
    /*
    for p in &path {
        println!("{:?} => {}", p, steps.get(p).unwrap());
    }
    */
    let mut save_hash: HashMap<usize, usize> = Default::default();

    println!("path is {} long", path.len());
    for (i, start_step) in path[..path.len() - 1].iter().enumerate() {
        for j in (i + 1)..(path.len()) {
            let probe = &path[j];
            if manhattan_len(*start_step, *probe) > dist {
                continue;
            }
            let savings = j - i - manhattan_len(*start_step, *probe);
            if savings >= min_savings {
                save_hash
                    .entry(savings)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
                /*
                println!(
                    "jumping from pos {:?} to {:?} will save {} steps",
                    start_step, probe, savings
                );
                */
            }
        }
    }
    println!("{:?}", save_hash.len());
    println!("{:?}", save_hash);
    for s in save_hash.iter().sorted_by(|s1, s2| s1.0.cmp(&s2.0)) {
        println!("There are {:>2} cheats that save {} picoseconds.", s.1, s.0);
    }
    println!("total savings: {}", save_hash.values().sum::<usize>());
}

fn part1(data: String) {
    process_part1(data, 2, 100);
}

fn part2(data: String) {
    process_part2(data, 20, 100);
}

fn main() {
    //let data = TEST;
    let data = std::fs::read_to_string("input/d20.txt").unwrap();
    part2(data.to_string());
}

static TEST: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

#[cfg(test)]

mod test {

    use super::*;

    #[test]
    fn test_jumps() {
        let g: BasicGrid<u16> = BasicGrid::new_default(15, 15);
        let start = Coord::new(1, 1);
        let v = super::possible_jumps(&g, start, 20);
        for r in 0..g.height {
            for c in 0..g.width {
                let this_c = Coord::new(r, c);
                print!(
                    "{}",
                    if this_c == start {
                        "O"
                    } else if v.contains(&this_c) {
                        "x"
                    } else {
                        "."
                    }
                );
            }
            println!();
        }
    }
}
