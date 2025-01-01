use std::{
    fmt::Display,
    io::{BufReader, Read},
};

pub fn input<T: Read>(r: T) -> std::io::BufReader<T> {
    BufReader::new(r)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AsciiByte(u8);

impl Display for AsciiByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::ascii::escape_default(self.0))
    }
}

impl From<u8> for AsciiByte {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn turn_right(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }

    pub fn as_ascii_byte(&self) -> AsciiByte {
        match self {
            Dir::Up => AsciiByte(b'^'),
            Dir::Down => AsciiByte(b'v'),
            Dir::Left => AsciiByte(b'<'),
            Dir::Right => AsciiByte(b'>'),
        }
    }
}

pub struct BasicGrid<T> {
    data: Box<[T]>,
    width: usize,
    height: usize,
}

impl<T> BasicGrid<T>
where
    T: From<u8>,
{
    pub fn new(lines: &[&str]) -> Self {
        let width = lines[0].len();
        let height = lines.len();
        let mut data: Vec<T> = Vec::with_capacity(width * height);

        for line in lines {
            let bytes = line.as_bytes();
            for b in bytes {
                data.push((*b).into());
            }
        }

        BasicGrid {
            width,
            height,
            data: data.into_boxed_slice(),
        }
    }
}

impl<T> BasicGrid<T> {
    fn idx_to_pos(&self, i: usize) -> Coord {
        Coord::new(i / self.width, i % self.width)
    }

    fn pos_to_idx(&self, pos: Coord) -> usize {
        pos.row * self.width + pos.col
    }

    pub fn next_pos(&self, pos: Coord, dir: Dir) -> Option<Coord> {
        match dir {
            Dir::Up => (pos.row > 0).then(|| Coord::new(pos.row - 1, pos.col)),
            Dir::Down => (pos.row < self.height - 1).then(|| Coord::new(pos.row + 1, pos.col)),
            Dir::Left => (pos.col > 0).then(|| Coord::new(pos.row, pos.col - 1)),
            Dir::Right => (pos.col < self.width - 1).then(|| Coord::new(pos.row, pos.col + 1)),
        }
    }
    fn ur_idx(&self, i: usize, cnt: usize) -> Option<usize> {
        let (row, col) = (i / self.width, i % self.width);
        (row > cnt - 1 && col < self.width - cnt).then(|| i - (self.width * cnt) + cnt)
    }

    fn ul_idx(&self, i: usize, cnt: usize) -> Option<usize> {
        let (row, col) = (i / self.width, i % self.width);
        (row > cnt - 1 && col >= cnt).then(|| i - (self.width * cnt) - cnt)
    }

    fn lr_idx(&self, i: usize, cnt: usize) -> Option<usize> {
        let (row, col) = (i / self.width, i % self.width);
        (row + cnt < self.height && col < self.width - cnt).then(|| i + (self.width * cnt) + cnt)
    }

    fn ll_idx(&self, i: usize, cnt: usize) -> Option<usize> {
        let (row, col) = (i / self.width, i % self.width);
        (row + cnt < self.height && col >= cnt).then(|| i + (self.width * cnt) - cnt)
    }

    fn idx_for(&self, row: usize, col: usize) -> usize {
        self.width * row + col
    }
}

impl<T> BasicGrid<T>
where
    T: From<u8> + PartialEq,
{
    pub fn find(&self, val: T) -> Option<Coord> {
        self.data
            .iter()
            .position(|v| *v == val)
            .map(|i| self.idx_to_pos(i))
    }
}

impl<T> BasicGrid<T>
where
    T: Copy,
{
    fn get_up(&self, i: usize, cnt: usize) -> Option<T> {
        let (row, col) = (i / self.width, i % self.width);
        (row > (cnt - 1)).then(|| self.data[i - self.width * cnt])
    }
    fn get_down(&self, i: usize, cnt: usize) -> Option<T> {
        let (row, col) = (i / self.width, i % self.width);
        (row + cnt < self.height).then(|| self.data[i + self.width * cnt])
    }

    fn get_ur(&self, i: usize, cnt: usize) -> Option<T> {
        self.ur_idx(i, cnt).map(|idx| self.data[idx])
    }

    fn get_ul(&self, i: usize, cnt: usize) -> Option<T> {
        self.ul_idx(i, cnt).map(|idx| self.data[idx])
    }

    fn get_lr(&self, i: usize, cnt: usize) -> Option<T> {
        self.lr_idx(i, cnt).map(|idx| self.data[idx])
    }
    fn get_ll(&self, i: usize, cnt: usize) -> Option<T> {
        self.ll_idx(i, cnt).map(|idx| self.data[idx])
    }
}
impl<T> BasicGrid<T>
where
    T: Display,
{
    pub fn display_all(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                print!("{}", self.data[self.idx_for(i as usize, j as usize)]);
            }
            print!("\n");
        }
        print!("\n");
    }

    pub fn display_at(&self, i: usize, cnt: usize) {
        let (center_row, center_col): (i32, i32) = (
            (i / self.width).try_into().unwrap(),
            (i % self.width).try_into().unwrap(),
        );
        let row: i32 = center_row - cnt as i32;
        let col: i32 = center_col - cnt as i32;
        let width = cnt as i32 * 2 + 1;
        println!("grid at {}", i);
        for i in row..(row + width) {
            for j in col..(col + width) {
                if i < 0 || j < 0 || i >= self.height as i32 || j >= self.width as i32 {
                    print!(".");
                } else {
                    print!("{}", self.data[self.idx_for(i as usize, j as usize)]);
                }
            }
            print!("\n");
        }
        print!("\n");
    }
}
impl<T> std::ops::Index<Coord> for BasicGrid<T> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.data[self.idx_for(index.row, index.col)]
    }
}

impl<T> std::ops::IndexMut<Coord> for BasicGrid<T> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.data[self.idx_for(index.row, index.col)]
    }
}
