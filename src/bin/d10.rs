use utils::{BasicGrid, Coord};

struct Part1 {
    grid: BasicGrid<u8>,
}

impl Part1 {
    fn trace_from(&self, start: Coord) -> u16 {
        let this = self.grid.at(start);

        todo!()
    }

    fn trace(&self, start: Coord) -> u16 {
        self.trace_from(start)
    }
}
fn main() {
    let input: Vec<&str> = TEST.lines().collect();
    let grid: BasicGrid<u8> = BasicGrid::new(&input);
    let part1 = Part1 { grid };
    let starts = part1.grid.find_with(|v| *v == 0);
    for start in starts {
        part1.trace(start);
    }
}

static TEST: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
