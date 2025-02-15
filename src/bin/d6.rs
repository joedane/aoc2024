#![allow(dead_code)]

use std::{collections::HashSet, fmt::Display};
use utils::*;

fn _part1(data: &str) {
    let mut input: Vec<&str> = vec![];
    for line in data.lines().map(str::trim) {
        input.push(line);
    }
    let mut grid: utils::BasicGrid<AsciiByte> = utils::BasicGrid::new(&input);
    let mut pos = grid.find(b'^'.into()).unwrap();
    let mut dir = Dir::Up;
    let mut visited: HashSet<Coord> = Default::default();
    visited.insert(pos);

    while let Some(next_pos) = grid.next_pos(pos, dir) {
        let v = grid[next_pos];
        if v == b'#'.into() {
            dir = dir.turn_right();
            continue;
        }
        pos = next_pos;
        grid[pos] = dir.as_ascii_byte();
        visited.insert(pos);
    }
    println!("visited {} spaces", visited.len());
}

#[derive(Debug, Clone)]
struct VisitState {
    step: usize,
    dir: Dir,
}

impl VisitState {
    fn new(step: usize, dir: Dir) -> Self {
        Self { step, dir }
    }

    fn visited(&self, dir: Dir) -> bool {
        self.dir == dir
    }
}

#[derive(Debug, Clone)]
enum CellState {
    Start(Vec<VisitState>),
    Obstructed,
    Open(Vec<VisitState>),
}

impl CellState {
    fn visit(&mut self, step: usize, dir: Dir) {
        match self {
            CellState::Obstructed => panic!(),
            CellState::Open(ref mut visit_state) | CellState::Start(ref mut visit_state) => {
                visit_state.push(VisitState::new(step, dir))
            }
        }
    }

    fn visited(&self, dir: Dir) -> Option<usize> {
        match self {
            CellState::Obstructed => None,
            CellState::Open(visits) | CellState::Start(visits) => visits
                .iter()
                .filter_map(|vs| if vs.dir == dir { Some(vs.step) } else { None })
                .nth(0),
        }
    }
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellState::Obstructed => write!(f, "#"),
            CellState::Open(_) | CellState::Start(_) => {
                if self.visited(Dir::Up).or(self.visited(Dir::Down)).is_some() {
                    if self
                        .visited(Dir::Left)
                        .or(self.visited(Dir::Right))
                        .is_some()
                    {
                        write!(f, "+")
                    } else {
                        write!(f, "|")
                    }
                } else if self
                    .visited(Dir::Left)
                    .or(self.visited(Dir::Right))
                    .is_some()
                {
                    write!(f, "-")
                } else {
                    write!(f, ".")
                }
            }
        }
    }
}

impl From<u8> for CellState {
    fn from(value: u8) -> Self {
        if value == b'#' {
            CellState::Obstructed
        } else if value == b'^' {
            CellState::Start(vec![])
        } else {
            CellState::Open(vec![])
        }
    }
}

fn would_loop_if_turn(
    grid: &BasicGrid<CellState>,
    mut pos: Coord,
    next_pos: Coord,
    dir: Dir,
    mut step: usize,
) -> bool {
    let mut loop_grid = (*grid).clone();
    loop_grid[next_pos] = CellState::Obstructed;
    let mut seen: HashSet<(Coord, Dir)> = Default::default();
    seen.insert((pos, dir));
    let mut dir = dir.turn_right();
    while let Some(next_pos) = loop_grid.next_pos(pos, dir) {
        if matches!(loop_grid[next_pos], CellState::Obstructed) {
            dir = dir.turn_right();
            continue;
        }
        if seen.contains(&(next_pos, dir)) {
            return true;
        }
        seen.insert((next_pos, dir));
        loop_grid[next_pos].visit(step, dir);
        pos = next_pos;
        step += 1;
    }
    return false;
}

fn part2(data: &str) {
    let mut input: Vec<&str> = vec![];
    for line in data.lines().map(str::trim) {
        input.push(line);
    }
    let mut grid: utils::BasicGrid<CellState> = utils::BasicGrid::new(&input);
    let start_candidates = grid.find_with(|v| match v {
        CellState::Start(_) => true,
        _ => false,
    });
    assert!(start_candidates.len() == 1);
    let mut pos = start_candidates[0];
    let mut dir = Dir::Up;
    let mut steps: usize = 0;
    let mut loop_pos: HashSet<Coord> = Default::default();
    let mut seen: HashSet<Coord> = Default::default();
    seen.insert(pos);
    while let Some(next_pos) = grid.next_pos(pos, dir) {
        if matches!(grid[next_pos], CellState::Obstructed) {
            dir = dir.turn_right();
            continue;
        } else if !seen.contains(&next_pos) && would_loop_if_turn(&grid, pos, next_pos, dir, steps)
        {
            println!("loop at {:?}", next_pos);
            loop_pos.insert(next_pos);
        }
        grid[next_pos].visit(steps, dir);
        seen.insert(next_pos);
        pos = next_pos;
        steps += 1;
    }

    println!("{} loops", loop_pos.len());
}
fn main() {
    let data = std::fs::read_to_string("input/d6.txt").unwrap();
    //let data = TEST;
    part2(&data);
}

static TEST: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
