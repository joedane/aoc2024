use std::io::{BufReader, Read};

pub mod d1;
pub mod d2;
pub mod d3;

fn input<T: Read>(r: T) -> std::io::BufReader<T> {
    BufReader::new(r)
}

fn main() {}
