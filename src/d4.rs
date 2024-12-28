struct Grid {
    data: Box<[u8]>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(lines: &[&str]) -> Self {
        let width = lines[0].len();
        let height = lines.len();
        let mut data: Vec<u8> = Vec::with_capacity(width * height);

        for line in lines {
            data.extend_from_slice(line.as_bytes());
        }

        Grid {
            width,
            height,
            data: data.into_boxed_slice(),
        }
    }

    fn up(&self, i: usize, cnt: usize, c: u8) -> bool {
        let (row, col) = (i / self.width, i % self.width);
        row > (cnt - 1) && self.data[i - self.width * cnt] == c
    }

    fn down(&self, i: usize, cnt: usize, c: u8) -> bool {
        let (row, col) = (i / self.width, i % self.width);
        row + cnt < self.height && self.data[i + self.width * cnt] == c
    }

    fn ur(&self, i: usize, cnt: usize, c: u8) -> bool {
        let (row, col) = (i / self.width, i % self.width);
        row > cnt - 1 && col < self.width - cnt && self.data[i - (self.width * cnt) + cnt] == c
    }

    fn ul(&self, i: usize, cnt: usize, c: u8) -> bool {
        let (row, col) = (i / self.width, i % self.width);
        row > cnt - 1 && col >= cnt && self.data[i - (self.width * cnt) - cnt] == c
    }

    fn lr(&self, i: usize, cnt: usize, c: u8) -> bool {
        let (row, col) = (i / self.width, i % self.width);
        row + cnt < self.height
            && col < self.width - cnt
            && self.data[i + (self.width * cnt) + cnt] == c
    }

    fn ll(&self, i: usize, cnt: usize, c: u8) -> bool {
        let (row, col) = (i / self.width, i % self.width);
        row + cnt < self.height && col >= cnt && self.data[i + (self.width * cnt) - cnt] == c
    }

    fn matches_right(&self, i: usize) -> bool {
        let (_, start_col) = (i / self.width, i % self.width);
        if start_col + 3 < self.width
            && self.data[i] == b'X'
            && self.data[i + 1] == b'M'
            && self.data[i + 2] == b'A'
            && self.data[i + 3] == b'S'
        {
            true
        } else {
            false
        }
    }

    fn matches_left(&self, i: usize) -> bool {
        let (_, start_col) = (i / self.width, i % self.width);
        if start_col > 2
            && self.data[i] == b'X'
            && self.data[i - 1] == b'M'
            && self.data[i - 2] == b'A'
            && self.data[i - 3] == b'S'
        {
            return true;
        } else {
            return false;
        }
    }

    fn matches_up(&self, i: usize) -> bool {
        self.data[i] == b'X' && self.up(i, 3, b'S') && self.up(i, 2, b'A') && self.up(i, 1, b'M')
    }

    fn matches_down(&self, i: usize) -> bool {
        self.data[i] == b'X'
            && self.down(i, 3, b'S')
            && self.down(i, 2, b'A')
            && self.down(i, 1, b'M')
    }
    fn matches_ur(&self, i: usize) -> bool {
        self.data[i] == b'X' && self.ur(i, 3, b'S') && self.ur(i, 2, b'A') && self.ur(i, 1, b'M')
    }

    fn matches_ul(&self, i: usize) -> bool {
        self.data[i] == b'X' && self.ul(i, 3, b'S') && self.ul(i, 2, b'A') && self.ul(i, 1, b'M')
    }

    fn matches_lr(&self, i: usize) -> bool {
        self.data[i] == b'X' && self.lr(i, 3, b'S') && self.lr(i, 2, b'A') && self.lr(i, 1, b'M')
    }

    fn matches_ll(&self, i: usize) -> bool {
        self.data[i] == b'X' && self.ll(i, 3, b'S') && self.ll(i, 2, b'A') && self.ll(i, 1, b'M')
    }
}

struct Part1;

impl Part1 {
    fn matches_right(grid: &Grid, i: usize) -> bool {
        grid.matches_right(i)
    }

    fn matches_left(grid: &Grid, i: usize) -> bool {
        grid.matches_left(i)
    }

    fn matches_up(grid: &Grid, i: usize) -> bool {
        grid.matches_up(i)
    }

    fn matches_down(grid: &Grid, i: usize) -> bool {
        grid.matches_down(i)
    }

    fn matches_ur(grid: &Grid, i: usize) -> bool {
        grid.matches_ur(i)
    }

    fn count_matches_at(grid: &Grid, i: usize) -> usize {
        let mut count: usize = 0;
        let c: char = grid.data[i] as char;
        if grid.data[i] == b'X' {
            if self.matches_right(i) {
                count += 1;
            }
            if self.matches_left(i) {
                count += 1;
            }
            if self.matches_up(i) {
                count += 1;
            }
            if self.matches_down(i) {
                count += 1;
            }
            if self.matches_ur(i) {
                count += 1;
            }
            if self.matches_ul(i) {
                count += 1;
            }
            if self.matches_lr(i) {
                count += 1;
            }
            if self.matches_ll(i) {
                count += 1;
            }
        }
        count
    }

    fn count_matches(&self) -> usize {
        (0..self.data.len()).map(|i| self.count_matches_at(i)).sum()
    }
}
pub fn main() {
    let mut lines = vec![];
    let data = std::fs::read_to_string("input/d4.txt").unwrap();
    for l in data.lines() {
        lines.push(l.trim());
    }
    let grid = Grid::new(&lines);

    println!("{}", grid.count_matches());
}

static TEST: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_basic() {
        let data = "....X
        ....M
        ....A
        ....S";
        let lines: Vec<&str> = data.lines().map(|s| s.trim()).collect();
        let grid = Grid::new(&lines);
        assert_eq!(1, grid.count_matches());

        let data = "....S
        ....A
        ....M
        ....X";
        let lines: Vec<&str> = data.lines().map(|s| s.trim()).collect();
        let grid = Grid::new(&lines);
        assert_eq!(1, grid.count_matches());

        let data = "X....
        .M...
        ..A..
        ...S.";
        let lines: Vec<&str> = data.lines().map(|s| s.trim()).collect();
        let grid = Grid::new(&lines);
        assert_eq!(1, grid.count_matches());

        let data = "...S.
        ..A..
        .M...
        X....";
        let lines: Vec<&str> = data.lines().map(|s| s.trim()).collect();
        let grid = Grid::new(&lines);
        assert_eq!(1, grid.count_matches());

        let data = "S....
        .A...
        ..M..
        ...X.";
        let lines: Vec<&str> = data.lines().map(|s| s.trim()).collect();
        let grid = Grid::new(&lines);
        assert_eq!(1, grid.count_matches());

        let data = "...S.
        ..A..
        .M...
        X....";
        let lines: Vec<&str> = data.lines().map(|s| s.trim()).collect();
        let grid = Grid::new(&lines);
        assert_eq!(1, grid.count_matches());

        let data = ".....
        .....
        .XMAS
        .....";
        let lines: Vec<&str> = data.lines().map(|s| s.trim()).collect();
        let grid = Grid::new(&lines);
        assert_eq!(1, grid.count_matches());

        let data = ".....
        .....
        .SAMX
        .....";
        let lines: Vec<&str> = data.lines().map(|s| s.trim()).collect();
        let grid = Grid::new(&lines);
        assert_eq!(1, grid.count_matches());
    }
}
