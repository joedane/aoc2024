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

#[derive(Debug)]
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

#[derive(Debug)]
enum CellState {
    Obstructed,
    Open(Vec<VisitState>),
}

impl CellState {
    fn visit(&mut self, step: usize, dir: Dir) {
        match self {
            CellState::Obstructed => panic!(),
            CellState::Open(ref mut visit_state) => visit_state.push(VisitState::new(step, dir)),
        }
    }

    fn visited(&self, dir: Dir) -> Option<usize> {
        match self {
            CellState::Obstructed => None,
            CellState::Open(visits) => visits
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
            CellState::Open(visit_state) => {
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
            CellState::Open(vec![VisitState::new(1, Dir::Up)])
        } else {
            CellState::Open(vec![])
        }
    }
}

fn would_loop_if_turn(grid: &BasicGrid<CellState>, step: usize, pos: Coord, dir: Dir) -> bool {
    let turn_dir = dir.turn_right();
    match grid.at(pos) {
        CellState::Obstructed => false,
        CellState::Open(visits) => {
            for v in visits {
                if v.dir == turn_dir {
                    return v.step < step;
                }
            }
            false
        }
    }
}

fn part2(data: &str) {
    let mut input: Vec<&str> = vec![];
    for line in data.lines().map(str::trim) {
        input.push(line);
    }
    let mut grid: utils::BasicGrid<CellState> = utils::BasicGrid::new(&input);
    let mut pos = grid
        .find_with(|v| match v {
            CellState::Obstructed => false,
            CellState::Open(vec) => vec.len() > 0,
        })
        .unwrap();
    let orig_pos = pos;
    let mut dir = Dir::Up;
    let mut loops: i32 = 0;
    let mut step: usize = 1;

    while let Some(next_pos) = grid.next_pos(pos, dir) {
        println!("part 1 (4,4) is {:?}", grid[Coord::new(4, 4)]);
        let v = &mut grid[next_pos];
        if matches!(*v, CellState::Obstructed) {
            grid[pos].visit(step, dir);
            dir = dir.turn_right();
            step += 1;
            grid[pos].visit(step, dir);
            continue;
        }
        grid[pos].visit(step, dir);
        step += 1;
        println!("grid at {:?} is now {:?}", pos, grid[pos]);
        pos = next_pos;
    }

    grid.display_all();

    pos = orig_pos;
    dir = Dir::Up;
    step = 1;
    while let Some(next_pos) = grid.next_pos(pos, dir) {
        let v = &grid[next_pos];
        if matches!(*v, CellState::Obstructed) {
            dir = dir.turn_right();
            step += 1;
            continue;
        } else if would_loop_if_turn(&grid, step, pos, dir) {
            loops += 1;
            println!("turn at {:?}", pos);
        }
        step += 1;
        pos = next_pos;
    }

    println!("{} loops", loops);
}
fn main() {
    // let data = std::fs::read_to_string("input/d6.txt").unwrap();
    let data = TEST;
    part2(data);
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
