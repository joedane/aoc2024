use std::collections::HashSet;

use utils::*;

fn part1(data: &str) {
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

fn would_loop_here(grid: &BasicGrid<CellState>, mut pos: Coord, mut dir: Dir) -> bool {
    dir = dir.turn_right();
    let orig_pos = pos;
    let mut turns_left: u8 = 3;
    'leg: while turns_left > 0 {
        while let Some(next_pos) = grid.next_pos(pos, dir) {
            let v = grid[next_pos];
            if v == b'#'.into() {
                dir = dir.turn_right();
                turns_left -= 1;
                continue 'leg;
            }
            pos = next_pos;
        }
        return false;
    }
    pos == orig_pos
}

#[derive(Clone, Copy, PartialEq)]
struct VisitState(u8);

impl VisitState {
    fn new(dir: Dir) -> Self {
        let mut v = VisitState(0);
        v.visit(dir);
        v
    }

    fn empty() -> Self {
        VisitState(0)
    }
    fn visited(&self, dir: Dir) -> bool {
        let flag: u8 = match dir {
            Dir::Up => 1,
            Dir::Down => 1 << 1,
            Dir::Left => 1 << 2,
            Dir::Right => 1 << 3,
        };
        self.0 & flag != 0
    }

    fn visit(&mut self, dir: Dir) {
        let flag: u8 = match dir {
            Dir::Up => 1,
            Dir::Down => 1 << 1,
            Dir::Left => 1 << 2,
            Dir::Right => 1 << 3,
        };
        self.0 = self.0 & flag;
    }
}

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Obstructed,
    Open(VisitState),
}

impl CellState {
    fn visit(&mut self, dir: Dir) {
        match self {
            CellState::Obstructed => panic!(),
            CellState::Open(mut visit_state) => {
                visit_state.visit(dir);
            }
        }
    }
}
impl From<u8> for CellState {
    fn from(value: u8) -> Self {
        if value == b'#' {
            CellState::Obstructed
        } else if value == b'^' {
            CellState::Open(VisitState::new(Dir::Up))
        } else {
            CellState::Open(VisitState::empty())
        }
    }
}
fn part2(data: &str) {
    let mut input: Vec<&str> = vec![];
    for line in data.lines().map(str::trim) {
        input.push(line);
    }
    let grid: utils::BasicGrid<CellState> = utils::BasicGrid::new(&input);
    let mut pos = grid
        .find(CellState::Open(VisitState::new(Dir::Up)))
        .unwrap();
    let orig_pos = pos;
    let mut dir = Dir::Up;
    let mut loops: i32 = 0;

    while let Some(next_pos) = grid.next_pos(pos, dir) {
        let v = grid[next_pos];
        if v == CellState::Obstructed {
            dir = dir.turn_right();
            v.visit(dir);
            continue;
        }
        pos = next_pos;
    }

    pos = orig_pos;
    dir = Dir::Up;
    while let Some(next_pos) = grid.next_pos(pos, dir) {
        let v = grid[next_pos];
        if v == CellState::Obstructed {
            dir = dir.turn_right();
            continue;
        
        if would_loop_here(&grid, pos, dir) {
            loops += 1;
        }
        pos = next_pos;
    }

    println!("{}", loops);
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
