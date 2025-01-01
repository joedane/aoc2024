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
fn main() {
    // let data = std::fs::read_to_string("input/d6.txt").unwrap();
    let data = TEST;
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
