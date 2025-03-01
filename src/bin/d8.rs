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
fn divisors(n: i64) -> Vec<i64> {
    let mut ret = vec![];
    let mut i = 1;
    while i <= n.abs() / 2 {
        if n % i == 0 {
            ret.push(i * n.signum());
        }
        i += 1;
    }
    ret
}

fn fill_part2(
    nodes: &mut HashSet<Coord>,
    width: usize,
    height: usize,
    start: Coord,
    dx: isize,
    dy: isize,
) {
    let mut c = start;
    while c.col < width && c.row < height {
        nodes.insert(c);
        let (v, of) = c.col.overflowing_add_signed(dx);
        if of {
            break;
        } else {
            c.col = v;
        }
        let (v, of) = c.row.overflowing_add_signed(dy);
        if of {
            break;
        } else {
            c.row = v;
        }
    }
    let mut c = start;
    while c.col < width && c.row < height {
        nodes.insert(c);
        let (v, of) = c.col.overflowing_add_signed(-dx);
        if of {
            break;
        } else {
            c.col = v;
        }
        let (v, of) = c.row.overflowing_add_signed(-dy);
        if of {
            break;
        } else {
            c.row = v;
        }
    }
}

fn add_nodes_part2(pair: &[&Coord], width: usize, height: usize, nodes: &mut HashSet<Coord>) {
    assert!(pair.len() == 2);

    let x_diff = pair[1].col as isize - pair[0].col as isize;
    let y_diff = pair[1].row as isize - pair[0].row as isize;

    let (dx, dy) = if x_diff == 0 {
        (0_isize, y_diff / y_diff.abs())
    } else if y_diff == 0 {
        (x_diff / x_diff.abs(), 0_isize)
    } else {
        (1..=x_diff.abs().min(y_diff.abs()))
            .rev()
            //            .inspect(|n| println!("n: {}, x_diff: {}, y_diff: {}", n, x_diff, y_diff))
            .find(|n| x_diff % n == 0 && y_diff % n == 0)
            .map(|n| (x_diff / n, y_diff / n))
            .unwrap()
        /*
            let col_divs = divisors(x_diff);
        let row_divs = divisors(y_diff);

        col_divs
            .iter()
            .copied()
            .cartesian_product(row_divs.iter().copied())
            .map(|(x, y)| (x as i64, y as i64))
            .find(|(dx, dy)| {
                if x_diff % dx == 0 && y_diff % dy == 0 {
                    println!(
                        "testing ({},{}) on pair {:?}\t{:?}",
                        dx, dy, pair[0], pair[1]
                    );
                    println!("xdiff: {}, ydiff: {}", x_diff, y_diff);
                    (1..x_diff.abs()).any(|n| n * dx == x_diff && n * dy == y_diff)
                } else {
                    false
                }
            })
            .unwrap()
        */
    };
    fill_part2(nodes, width, height, *pair[0], dx, dy);
    /*
    println!(
        "for pair ({:?}, {:?}):\t\tdx = {}\tdy = {}",
        pair[0], pair[1], dx, dy
    );
        */
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
    let data = std::fs::read_to_string("input/d8.txt").unwrap();
    //let data = TEST;
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
        assert_eq!(divisors(0), vec![]);

        assert_eq!(divisors(-12), vec![-1, -2, -3, -4, -6]);
    }
}
