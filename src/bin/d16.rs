use std::{
    cell::{Cell, RefCell},
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
};

use slotmap::{new_key_type, SecondaryMap, SlotMap};
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

new_key_type! {
    struct LinksKey;
}
struct PathData {
    score: usize,
    pos: Coord,
    facing: Dir,
}

struct Links {
    straight: Option<LinksKey>,
    left: Option<LinksKey>,
    right: Option<LinksKey>,
    last: Option<LinksKey>,
}

impl Links {
    fn link_from(k: LinksKey) -> Self {
        Self {
            straight: None,
            left: None,
            right: None,
            last: Some(k),
        }
    }
}

fn path_from(
    path_tree: &SlotMap<LinksKey, RefCell<Links>>,
    path_data: &SecondaryMap<LinksKey, PathData>,
    from: LinksKey,
) -> Vec<Coord> {
    let mut v = vec![];
    let mut l = from;
    loop {
        v.push(path_data.get(l).unwrap().pos);
        match path_tree.get(l).unwrap().borrow().last {
            Some(parent) => {
                l = parent;
            }
            None => {
                break;
            }
        };
    }
    v
}
fn paths_from(grid: &BasicGrid<State>, start: Coord) -> (Vec<Vec<Coord>>, usize) {
    let mut path_tree: SlotMap<LinksKey, RefCell<Links>> = SlotMap::with_key();
    let root = path_tree.insert(RefCell::new(Links {
        straight: None,
        left: None,
        right: None,
        last: None,
    }));
    let mut path_data = SecondaryMap::new();
    path_data.insert(
        root,
        PathData {
            score: 0,
            pos: start,
            facing: Dir::Right,
        },
    );

    let mut to_visit: Vec<LinksKey> = vec![root];
    let mut completed: Vec<LinksKey> = Default::default();
    let mut best_scores: HashMap<(Coord, Dir), usize> = Default::default();

    while let Some(k) = to_visit.pop() {
        let data = path_data.get(k).unwrap();
        let current_pos = data.pos;
        'dir: for (dir, score) in [
            (data.facing, data.score + 1),
            (data.facing.turn_left(), data.score + 1001),
            (data.facing.turn_right(), data.score + 1001),
        ] {
            if let Some(c) = grid.next_pos(current_pos, dir) {
                match grid.at(c) {
                    State::Wall => {}
                    state @ State::Empty | state @ State::Start | state @ State::End => {
                        if let Some(best_score) = best_scores.get(&(c, dir)) {
                            if *best_score < score {
                                continue 'dir;
                            }
                        }
                        best_scores.insert((c, dir), score);
                        let new_data = PathData {
                            score: score,
                            pos: c,
                            facing: dir,
                        };
                        let new_link = path_tree.insert(RefCell::new(Links::link_from(k)));
                        path_data.insert(new_link, new_data);
                        let _ = path_tree
                            .get(k)
                            .unwrap()
                            .borrow_mut()
                            .straight
                            .insert(new_link);
                        if matches!(state, State::End) {
                            completed.push(new_link);
                        } else {
                            to_visit.push(new_link);
                        }
                    }
                }
            }
        }
    }
    let end_coord = path_data.get(completed[0]).unwrap().pos;
    let best_score = best_scores
        .iter()
        .filter(|((coord, dir), v)| *coord == end_coord)
        .map(|((coord, dir), v)| *v)
        .min()
        .unwrap();
    let paths: Vec<Vec<Coord>> = completed
        .into_iter()
        .filter(|l| path_data.get(*l).unwrap().score == best_score)
        .map(|l| path_from(&path_tree, &path_data, l))
        .collect();
    (paths, best_score)
}

fn best_score_from(grid: &BasicGrid<State>, start: Coord) -> Option<usize> {
    let paths = paths_from(grid, start);
    Some(paths.1)
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
    println!("{:?}", best_score_from(&grid, start_pos));
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
    let (paths, score) = paths_from(&grid, start_pos);
    let cells: HashSet<Coord> = paths.into_iter().flat_map(|v| v.into_iter()).collect();
    dump_tiles(&grid, &cells);
    println!("{}", cells.len());
    //dump_partials(&grid, &paths);
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
