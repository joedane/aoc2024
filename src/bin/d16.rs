use std::collections::HashSet;

use utils::{BasicGrid, Coord, Dir};

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

fn best_path_from(
    grid: &BasicGrid<State>,
    visited: &HashSet<Coord>,
    start: Coord,
    facing: Dir,
    current_score: u64,
) -> Option<u64> {
    if visited.contains(&start) {
        return None;
    }
    if matches!(grid.at(grid.next_pos(start, facing).unwrap()), State::End) {
        return Some(current_score + 1);
    }
    let mut best: Option<u64> = None;
    let mut visited = visited.clone();
    visited.insert(start);
    if matches!(grid.at(grid.next_pos(start, facing).unwrap()), State::Empty) {
        best = best_path_from(
            grid,
            &visited,
            grid.next_pos(start, facing).unwrap(),
            facing,
            current_score + 1,
        );
    }

    if matches!(
        grid.at(grid.next_pos(start, facing.turn_right()).unwrap()),
        State::Empty
    ) {
        if let Some(v) = best_path_from(
            grid,
            &visited,
            grid.next_pos(start, facing.turn_right()).unwrap(),
            facing.turn_right(),
            current_score + 1001,
        ) {
            best = best.map(|old_v| v.max(old_v));
        }
    }

    if matches!(
        grid.at(grid.next_pos(start, facing.turn_left()).unwrap()),
        State::Empty
    ) {
        if let Some(v) = best_path_from(
            grid,
            &visited,
            grid.next_pos(start, facing.turn_left()).unwrap(),
            facing.turn_left(),
            current_score + 1001,
        ) {
            best = best.map(|old_v| v.max(old_v));
        }
    }
    best
}

fn main() {
    let input = TEST;
    let lines: Vec<&str> = input.lines().map(str::trim).collect();
    let grid: BasicGrid<State> = BasicGrid::new(&lines);
    let start_pos = grid
        .find_with(|s| matches!(s, State::Start))
        .first()
        .unwrap()
        .clone();
    let end_pos = grid.find_with(|s| matches!(s, State::End)).first().unwrap();
    println!(
        "{:?}",
        best_path_from(&grid, &HashSet::new(), start_pos, Dir::Left, 0)
    );
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
