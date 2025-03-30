#![allow(dead_code)]

use std::{
    fmt::{Debug, Display},
    io::{BufReader, Read},
};

pub fn input<T: Read>(r: T) -> std::io::BufReader<T> {
    BufReader::new(r)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AsciiByte(pub u8);

impl Debug for AsciiByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::ascii::escape_default(self.0))
    }
}
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
    pub row: usize,
    pub col: usize,
}

impl Coord {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
    pub fn in_dir(&self, dir: Dir) -> Option<Self> {
        match dir {
            Dir::Down => Some(Self {
                row: self.row + 1,
                ..*self
            }),
            Dir::Up => {
                if self.row > 0 {
                    Some(Self {
                        row: self.row - 1,
                        ..*self
                    })
                } else {
                    None
                }
            }
            Dir::Left => {
                if self.col > 0 {
                    Some(Self {
                        col: self.col - 1,
                        ..*self
                    })
                } else {
                    None
                }
            }
            Dir::Right => Some(Self {
                col: self.col + 1,
                ..*self
            }),
        }
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
    pub width: usize,
    pub height: usize,
}

impl<T> Clone for BasicGrid<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            width: self.width,
            height: self.height,
        }
    }
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
    pub fn find_with<F>(&self, pred: F) -> Vec<Coord>
    where
        F: Fn(&T) -> bool,
    {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(idx, v)| {
                if pred(v) {
                    Some(self.idx_to_pos(idx))
                } else {
                    None
                }
            })
            .collect()
    }
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

    pub fn at(&self, pos: Coord) -> &T {
        &self.data[self.pos_to_idx(pos)]
    }
}

impl<T> BasicGrid<T>
where
    T: PartialEq + std::fmt::Debug,
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
    pub fn get(&self, from: Coord, dir: Dir, cnt: usize) -> Option<T> {
        let i = self.pos_to_idx(from);
        let (row, col) = (i / self.width, i % self.width);
        match dir {
            Dir::Up => (row > (cnt - 1)).then(|| self.data[i - self.width * cnt]),
            Dir::Down => (row + cnt < self.height).then(|| self.data[i + self.width * cnt]),
            Dir::Left => (col >= cnt).then(|| self.data[i - cnt]),
            Dir::Right => (col + cnt < self.width).then(|| self.data[i + cnt]),
        }
    }
}

impl<T> BasicGrid<T>
where
    T: Display,
{
    pub fn display_all(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                print!("{}", self.data[self.idx_for(i, j)]);
            }
            println!();
        }
        println!();
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
            println!();
        }
        println!();
    }
}
impl<T> std::ops::Index<usize> for BasicGrid<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
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

pub struct GridIterator<'a, T> {
    grid: &'a BasicGrid<T>,
    at_row: usize,
    at_col: usize,
}

impl<T> Iterator for GridIterator<'_, T> {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at_col == self.grid.width {
            if self.at_row == self.grid.height - 1 {
                None
            } else {
                let c = Coord::new(self.at_row, self.at_col);
                self.at_row += 1;
                self.at_col = 0;
                Some(c)
            }
        } else {
            let c = Coord::new(self.at_row, self.at_col);
            self.at_col += 1;
            Some(c)
        }
    }
}
impl<T> BasicGrid<T> {
    pub fn row_major_iter(&self) -> GridIterator<T> {
        GridIterator {
            grid: self,
            at_row: 0,
            at_col: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move() {
        let data = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let input: Vec<&str> = data.lines().collect();
        let grid: BasicGrid<u8> = BasicGrid::new(&input);
        assert_eq!(grid.get(Coord::new(0, 0), Dir::Down, 1), Some(b'7'));
        assert_eq!(grid.get(Coord::new(1, 2), Dir::Left, 1), Some(b'8'));
        assert_eq!(grid.get(Coord::new(3, 0), Dir::Up, 1), Some(b'8'));
        assert_eq!(grid.get(Coord::new(0, 0), Dir::Right, 1), Some(b'9'));
    }
}
