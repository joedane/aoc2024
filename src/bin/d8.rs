#![allow(dead_code)]

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use utils::{AsciiByte, BasicGrid, Coord};

fn add_nodes_part1(pair: &[&Coord], width: usize, height: usize, nodes: &mut HashSet<Coord>) {
    assert!(pair.len() == 2);

    let wi = std::convert::TryInto::<i64>::try_into(width).unwrap();
    let hi = std::convert::TryInto::<i64>::try_into(height).unwrap();

    let px0 = std::convert::TryInto::<i64>::try_into(pair[0].col).unwrap();
    let px1 = std::convert::TryInto::<i64>::try_into(pair[1].col).unwrap();
    let py0 = std::convert::TryInto::<i64>::try_into(pair[0].row).unwrap();
    let py1 = std::convert::TryInto::<i64>::try_into(pair[1].row).unwrap();

    let dx = px0 - px1;
    let dy = py0 - py1;

    if px0 + dx >= 0 && px0 + dx < wi && py0 + dy >= 0 && py0 + dy < hi {
        nodes.insert(Coord::new((py0 + dy) as usize, (px0 + dx) as usize));
    }

    let dx = px1 - px0;
    let dy = py1 - py0;

    if px1 + dx >= 0 && px1 + dx < wi && py1 + dy >= 0 && py1 + dy < hi {
        nodes.insert(Coord::new((py1 + dy) as usize, (px1 + dx) as usize));
    }
}
fn divisors(n: usize) -> Vec<usize> {
    let mut ret = vec![1];
    let mut i = 2;
    while i <= n / 2 {
        if n % i == 0 {
            ret.push(i);
        }
        i += 1;
    }
    ret
}

fn add_nodes_part2(pair: &[&Coord], width: usize, height: usize, nodes: &mut HashSet<Coord>) {
    assert!(pair.len() == 2);

    let wi = std::convert::TryInto::<i64>::try_into(width).unwrap();
    let hi = std::convert::TryInto::<i64>::try_into(height).unwrap();

    let mut px0 = std::convert::TryInto::<i64>::try_into(pair[0].col).unwrap();
    let mut px1 = std::convert::TryInto::<i64>::try_into(pair[1].col).unwrap();
    let mut py0 = std::convert::TryInto::<i64>::try_into(pair[0].row).unwrap();
    let mut py1 = std::convert::TryInto::<i64>::try_into(pair[1].row).unwrap();

    let col_diff = pair[0].col.abs_diff(pair[1].col);
    let row_diff = pair[0].row.abs_diff(pair[1].row);
    let col_divs = divisors(col_diff);
    let row_divs = divisors(row_diff);
    if let Some((dx, dy)) = col_divs.iter().copied().filter(|d| {
        let cnt = col_diff / d;
        if let Some(dy) = row_divs.iter().copied().filter(|d| {

        });
        None
    });
    let dx = px0 - px1;
    let dy = py0 - py1;

    while px0 + dx >= 0 && px0 + dx < wi && py0 + dy >= 0 && py0 + dy < hi {
        nodes.insert(Coord::new((py0 + dy) as usize, (px0 + dx) as usize));
        px0 += dx;
        py0 += dy;
    }

    let dx = px1 - px0;
    let dy = py1 - py0;

    while px1 + dx >= 0 && px1 + dx < wi && py1 + dy >= 0 && py1 + dy < hi {
        nodes.insert(Coord::new((py1 + dy) as usize, (px1 + dx) as usize));
        px1 += dx;
        py1 += dy;
    }
}

fn part1(grid: BasicGrid<AsciiByte>, node_map: HashMap<AsciiByte, Vec<Coord>>) {
    let mut ans: HashSet<Coord> = Default::default();
    for (_, coords) in node_map.iter() {
        coords.iter().combinations(2).for_each(|pair| {
            add_nodes_part1(&pair, grid.width, grid.height, &mut ans);
        });
    }
    for r in 0..grid.height {
        for c in 0..grid.width {
            let coord = Coord::new(r, c);
            print!(
                "{}",
                if ans.contains(&coord) {
                    AsciiByte(b'#')
                } else {
                    grid[coord]
                }
            );
        }
        println!();
    }

    println!("{}", ans.len());
}

fn part2(grid: BasicGrid<AsciiByte>, node_map: HashMap<AsciiByte, Vec<Coord>>) {
    let mut ans: HashSet<Coord> = Default::default();
    for (_, coords) in node_map.iter() {
        coords.iter().combinations(2).for_each(|pair| {
            add_nodes_part2(&pair, grid.width, grid.height, &mut ans);
        });
    }
    for r in 0..grid.height {
        for c in 0..grid.width {
            let coord = Coord::new(r, c);
            print!(
                "{}",
                if ans.contains(&coord) {
                    AsciiByte(b'#')
                } else {
                    grid[coord]
                }
            );
        }
        println!();
    }

    println!("{}", ans.len());
}

fn main() {
    let mut input: Vec<&str> = vec![];
    //let data = std::fs::read_to_string("input/d8.txt").unwrap();
    let data = TEST;
    for line in data.lines().map(|s| s.trim()) {
        input.push(line);
    }
    let grid: BasicGrid<AsciiByte> = BasicGrid::new(&input);
    let mut node_map: HashMap<AsciiByte, Vec<Coord>> = Default::default();
    for c in grid.row_major_iter() {
        if grid[c] != b'.'.into() {
            node_map.entry(grid[c]).or_default().push(c);
        }
    }
    part2(grid, node_map);
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

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_divisors() {
        assert_eq!(divisors(6), vec![1, 2, 3]);
        assert_eq!(divisors(12), vec![1, 2, 3, 4, 6]);
        assert_eq!(divisors(13), vec![1]);
    }
}
