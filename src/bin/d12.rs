use std::collections::{HashMap, HashSet};

use utils::{AsciiByte, BasicGrid, Coord, Dir};

fn remove_some(roots: &mut HashSet<Coord>) -> Option<Coord> {
    if let Some(e) = roots.iter().next().cloned() {
        roots.remove(&e);
        Some(e)
    } else {
        None
    }
}

#[derive(Debug)]
struct Region {
    code: AsciiByte,
    plots: HashSet<Coord>,
}

impl Region {
    fn new(c: Coord, code: AsciiByte) -> Self {
        let mut hs: HashSet<Coord> = Default::default();
        hs.insert(c);
        Self { code, plots: hs }
    }

    fn add_to(&mut self, c: Coord) {
        self.plots.insert(c);
    }

    fn contains(&self, c: Coord) -> bool {
        self.plots.contains(&c)
    }

    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perieter(&self) -> usize {
        let mut sz = 0_usize;
        for p in self.plots.iter() {
            for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                if let Some(c) = p.in_dir(dir) {
                    if !self.plots.contains(&c) {
                        sz += 1;
                    }
                } else {
                    sz += 1;
                }
            }
        }
        sz
    }

    fn price(&self) -> usize {
        self.area() * self.perieter()
    }

    fn sides(&self, grid: &BasicGrid<AsciiByte>) -> usize {
        self.plots.iter().map(|c| corners(*c, grid)).sum()
    }
}

fn corners(c: Coord, grid: &BasicGrid<AsciiByte>) -> usize {
    let mut cnt = 0_usize;
    for d in [Dir::Up, Dir::Right, Dir::Down, Dir::Left] {
        let (d1, d2) = (d, d.turn_right());
        if grid.get(c, d1, 1) != Some(*grid.at(c)) && grid.get(c, d2, 1) != Some(*grid.at(c)) {
            // convex corner
            cnt += 1;
        } else if grid.get(c, d1, 1) == Some(*grid.at(c)) && grid.get(c, d2, 1) == Some(*grid.at(c))
        {
            let x = grid.next_pos(c, d1).and_then(|x| grid.next_pos(x, d2));
            if x != Some(c) {
                cnt += 1;
            }
        }
    }
    cnt
}
fn build_regions(grid: &BasicGrid<AsciiByte>) -> Vec<Region> {
    let mut roots: HashSet<Coord> = (0..grid.height)
        .flat_map(|row| (0..grid.width).map(move |col| Coord::new(row, col)))
        .collect();
    let mut regions: Vec<Region> = Default::default();
    while let Some(e) = remove_some(&mut roots) {
        let mut stack: Vec<Coord> = vec![e];
        let mut region = Region::new(e, *grid.at(e));
        while let Some(e) = stack.pop() {
            let this_e = grid.at(e);
            for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                if let Some(up_e) = grid.get(e, dir, 1) {
                    if up_e == *this_e {
                        let c = e.in_dir(dir).unwrap();
                        if !region.contains(c) {
                            roots.remove(&c);
                            region.add_to(c);
                            stack.push(c);
                        }
                    }
                }
            }
        }
        regions.push(region);
    }
    regions
}

fn part1() {
    //let input = TEST2;
    let input = std::fs::read_to_string("input/d12.txt").unwrap();
    let data: Vec<&str> = input.lines().collect();
    let grid: BasicGrid<AsciiByte> = BasicGrid::new(&data);
    let regions = build_regions(&grid);
    /*
    for r in regions.iter() {
        println!(
            "Region {} has area {} and perimeter {} and price {}",
            r.code,
            r.area(),
            r.perieter(),
            r.price()
        );
    }
    */
    println!(
        "total price: {}",
        regions.iter().map(|r| r.price()).sum::<usize>()
    );
}

fn part2() {
    let input = TEST;
    //let input = std::fs::read_to_string("input/d12.txt").unwrap();
    let data: Vec<&str> = input.lines().collect();
    let grid: BasicGrid<AsciiByte> = BasicGrid::new(&data);
    let regions = build_regions(&grid);
    for r in regions.iter() {
        println!(
            "Region {} has area {} and perimeter {} and sides {}",
            r.code,
            r.area(),
            r.perieter(),
            r.sides(&grid),
        );
    }
    println!(
        "total corners: {}",
        regions.iter().map(|r| r.sides(&grid)).sum::<usize>()
    );
}
fn main() {
    part2();
}

static TEST: &str = "AAAA
BBCD
BBCC
EEEC";

static TEST1: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

static TEST2: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
