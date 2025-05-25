use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
};

use utils::{BasicGrid, Coord, Dir};

use owo_colors::OwoColorize;
use rand::prelude::*;

enum State {
    Wall,
    Empty,
    Start,
    End,
}

impl From<u8> for State {
    fn from(value: u8) -> Self {
        if value == b'#' {
            State::Wall
        } else if value == b'.' {
            State::Empty
        } else if value == b'S' {
            State::Start
        } else if value == b'E' {
            State::End
        } else {
            panic!("invalid char: {}", value)
        }
    }
}

#[derive(Debug, Clone)]
struct PartialPath {
    steps: Vec<Dir>,
    coords: Vec<Coord>,
    facing: Dir,
    score: usize,
}

impl PartialPath {
    fn move_facing(&self, next_pos: Coord) -> Self {
        let mut steps = self.steps.clone();
        steps.push(self.facing);
        let mut coords = self.coords.clone();
        coords.push(next_pos);
        Self {
            steps,
            coords,
            facing: self.facing,
            score: self.score + 1,
        }
    }

    fn turn_left(&self) -> Self {
        Self {
            steps: self.steps.clone(),
            coords: self.coords.clone(),
            facing: self.facing.turn_left(),
            score: self.score + 1000,
        }
    }

    fn turn_right(&self) -> Self {
        Self {
            steps: self.steps.clone(),
            coords: self.coords.clone(),
            facing: self.facing.turn_right(),
            score: self.score + 1000,
        }
    }
}

fn dump_partials(grid: &BasicGrid<State>, partials: &Vec<PartialPath>) {
    let mut colors: HashMap<Coord, (u8, u8, u8)> = Default::default();
    let mut rng = rand::rng();

    fn sat_add(v1: &(u8, u8, u8), v2: &(u8, u8, u8)) -> (u8, u8, u8) {
        (
            v1.0.saturating_add(v2.0),
            v1.1.saturating_add(v2.1),
            v1.2.saturating_add(v2.2),
        )
    }

    for path in partials.iter() {
        let rgb: (u8, u8, u8) = (rng.random(), rng.random(), rng.random());
        for c in path.coords.iter() {
            colors
                .entry(*c)
                .and_modify(|e| *e = sat_add(e, &rgb))
                .or_insert(rgb);
        }
    }

    for row in 0..grid.height {
        for col in 0..grid.width {
            let c = Coord::new(row, col);
            match grid.at(c) {
                State::Wall => print!("#"),
                State::Start => print!("S"),
                State::End => print!("E"),
                State::Empty => {
                    if let Some(color) = colors.get(&c) {
                        print!("{}", "o".truecolor(color.0, color.1, color.2));
                    } else {
                        print!(".");
                    }
                }
            }
        }
        println!();
    }
}

struct Partials(Vec<PartialPath>);

impl Partials {
    fn pop(&mut self) -> Option<PartialPath> {
        self.0.pop()
    }
    fn push(&mut self, item: PartialPath) {
        self.0.push(item);
    }
}
fn paths_from(grid: &BasicGrid<State>, start: Coord) -> Vec<PartialPath> {
    let mut partials: Partials = Partials(vec![PartialPath {
        steps: vec![],
        coords: vec![start],
        facing: Dir::Right,
        score: 0,
    }]);
    let mut complete_paths: Vec<PartialPath> = Default::default();
    let mut best_paths: HashMap<(Coord, Dir), usize> = Default::default();

    fn try_path(
        grid: &BasicGrid<State>,
        path: &PartialPath,
        partials: &mut Partials,
        complete_paths: &mut Vec<PartialPath>,
        best_paths: &mut HashMap<(Coord, Dir), usize>,
    ) {
        let next_pos = grid
            .next_pos(*path.coords.last().unwrap(), path.facing)
            .unwrap();
        //if !path.coords.contains(&next_pos) {
        match grid.at(next_pos) {
            State::End => {
                complete_paths.push(path.move_facing(next_pos));
            }
            State::Wall => {}
            State::Empty | State::Start => {
                let next = path.move_facing(next_pos);
                let next_score = next.score;
                if best_paths
                    .get(&(next_pos, path.facing))
                    .is_none_or(|v| *v >= next_score)
                {
                    best_paths.insert((next_pos, path.facing), next_score);
                    partials.push(next);
                }
            }
        }
        //}
    }

    while let Some(p) = partials.pop() {
        //println!("processing {:?}", p);
        //dump_partials(grid, &partials.0);
        //println!("{:?}", p.steps);
        //println!("YYY");

        try_path(
            grid,
            &p,
            &mut partials,
            &mut complete_paths,
            &mut best_paths,
        );
        try_path(
            grid,
            &p.turn_left(),
            &mut partials,
            &mut complete_paths,
            &mut best_paths,
        );
        try_path(
            grid,
            &p.turn_right(),
            &mut partials,
            &mut complete_paths,
            &mut best_paths,
        );
    }
    complete_paths
}

fn best_path_from(grid: &BasicGrid<State>, start: Coord) -> Option<usize> {
    let mut paths = paths_from(grid, start);
    if paths.is_empty() {
        return None;
    } else {
        paths.sort_by_key(|p| p.score);
        return Some(paths.first().unwrap().score);
    }
}

fn part1() {
    //let input = TEST;
    let input = std::fs::read_to_string("input/d16.txt").unwrap();
    let lines: Vec<&str> = input.lines().map(str::trim).collect();
    let grid: BasicGrid<State> = BasicGrid::new(&lines);
    let start_pos = grid
        .find_with(|s| matches!(s, State::Start))
        .first()
        .unwrap()
        .clone();
    println!("{:?}", best_path_from(&grid, start_pos));
}

fn dump_tiles(grid: &BasicGrid<State>, tiles: &HashSet<Coord>) {
    for r in 0..grid.height {
        for col in 0..grid.width {
            let c = Coord::new(r, col);
            match grid.at(c) {
                State::Wall => print!("{}", '#'),
                _ if tiles.contains(&c) => print!("{}", 'O'.bold()),
                State::Empty => print!("{}", '.'),
                State::Start => print!("{}", 'S'),
                State::End => print!("{}", 'E'),
            }
        }
        println!();
    }
}
fn part2() {
    //let input = TEST1;
    let input = std::fs::read_to_string("input/d16.txt").unwrap();
    let lines: Vec<&str> = input.lines().map(str::trim).collect();
    let grid: BasicGrid<State> = BasicGrid::new(&lines);
    let start_pos = grid
        .find_with(|s| matches!(s, State::Start))
        .first()
        .unwrap()
        .clone();
    let paths = paths_from(&grid, start_pos);
    let best_score: usize = paths.iter().map(|p| p.score).min().unwrap();
    //dump_partials(&grid, &paths);
    let tiles: HashSet<Coord> = paths
        .iter()
        .filter(|p| p.score == best_score)
        .flat_map(|path| path.coords.clone())
        .collect();
    println!("{}", tiles.len());
    dump_tiles(&grid, &tiles);
}

pub fn main() {
    part2();
}

static TEST: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

static TEST1: &str = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;
