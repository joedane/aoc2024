#![allow(dead_code)]

use itertools::Itertools;
use std::collections::HashMap;

use utils::{AsciiByte, BasicGrid, Coord};

fn main() {
    let mut input: Vec<&str> = vec![];
    for line in TEST.lines().map(|s| s.trim()) {
        input.push(line);
    }
    let grid: BasicGrid<AsciiByte> = BasicGrid::new(&input);
    let mut node_map: HashMap<AsciiByte, Vec<Coord>> = Default::default();
    for c in grid.row_major_iter() {
        if grid[c] != b'.'.into() {
            node_map.entry(grid[c]).or_insert(vec![]).push(c);
        }
    }
    let mut ans: HashMap<AsciiByte, usize> = Default::default();
    for (code, coords) in node_map.iter() {
        coords.iter().combinations(2).map(|pair| {});
    }
}
static TEST: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
