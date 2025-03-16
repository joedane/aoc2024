use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, VecDeque},
    fs::File,
    io::Write,
};

use random_color::RandomColor;

fn part1(input: &str, mut data: Vec<u16>) {
    let mut in_skip = false;
    let mut next_id: u16 = 0;
    for b in input.trim().as_bytes() {
        if in_skip {
            let n = *b - 48;
            for i in 0..n {
                data.push(u16::MAX);
            }
        } else {
            let n = *b - 48;
            for i in 0..n {
                data.push(next_id);
            }
            next_id += 1;
        }
        in_skip = !in_skip;
    }
    let mut first_free = data
        .iter()
        .enumerate()
        .find_map(|(n, b)| if *b == u16::MAX { Some(n) } else { None })
        .unwrap();
    let mut last_used = data.len() - 1;
    while first_free < last_used {
        data.swap(first_free, last_used);
        first_free += 1;
        while data[first_free] != u16::MAX {
            first_free += 1;
        }
        last_used -= 1;
        while data[last_used] == u16::MAX {
            last_used -= 1;
        }
    }
    let v: u64 = data.iter().enumerate().fold(0, |acc, (n, c)| {
        if *c == u16::MAX {
            acc
        } else {
            acc + n as u64 * *c as u64
        }
    });
    println!("{}", v);
}

struct Part2 {
    free_maps: [BinaryHeap<Reverse<usize>>; 9],
    data: Vec<u16>,
}

impl Part2 {
    fn new(input: &str, mut data: Vec<u16>) -> Self {
        let mut in_skip = false;
        let mut next_id: u16 = 0;
        let mut free_maps: [BinaryHeap<Reverse<usize>>; 9] = Default::default();

        for b in input.trim().as_bytes() {
            if in_skip {
                let n = (*b - 48) as usize;
                if n > 0 {
                    let start = data.len();
                    for i in 0..n {
                        data.push(u16::MAX);
                    }
                    free_maps[n - 1].push(Reverse(start));
                }
            } else {
                let n = *b - 48;
                assert!(n > 0);
                for i in 0..n {
                    data.push(next_id);
                }
                next_id += 1;
            }
            in_skip = !in_skip;
        }
        println!("data len: {}", data.len());
        Self { free_maps, data }
    }

    fn first_fit(&mut self, src: usize, len: usize) -> Option<usize> {
        let mut ret = None;
        for i in (len - 1)..self.free_maps.len() {
            if let Some(p) = self.free_maps[i].peek() {
                if p.0 < src && ret.is_none_or(|(idx, old_p)| p.0 < old_p) {
                    ret = Some((i, p.0));
                }
            }
        }
        ret.map(|(idx, p)| idx)
    }

    fn move_data(&mut self, idx: usize, src: usize, l: usize) -> usize {
        let dst = self.free_maps[idx].pop().unwrap();
        self.data.copy_within(src..(src + l), dst.0);
        self.data[src..(src + l)].fill(u16::MAX);
        dst.0
    }

    fn dump_data(&self) {
        for i in 0..self.data.len() {
            if self.data[i] == u16::MAX {
                print!(".");
            } else {
                print!("{}", self.data[i]);
            }
        }
        println!();
    }

    fn dump_data_html<W>(&self, mut w: W) -> std::io::Result<()>
    where
        W: Write,
    {
        let colors: [String; 8] = core::array::from_fn(|i| RandomColor::new().to_hex());
        write!(&mut w, "<!DOCTYPE html>")?;
        writeln!(
            &mut w,
            "<html><head><link rel=\"stylesheet\" href=\"d9.css\"><title>Day 9 testing</title></head><body><table>"
        )?;
        for r in self.data.chunks(100) {
            writeln!(&mut w, "<tr>")?;
            for c in r {
                if *c == u16::MAX {
                    write!(&mut w, "<td class=\"empty\">.</td>")?
                } else {
                    write!(
                        &mut w,
                        "<td class=\"file color-{}\" title=\"{}\">{}</td>",
                        c % 8,
                        c,
                        c % 100
                    )?;
                }
            }
            writeln!(&mut w, "</tr>")?;
        }
        writeln!(&mut w, "</table></body></html>")?;

        Ok(())
    }
}

fn part2(input: &str, mut data: Vec<u16>) {
    let mut map = Part2::new(input, data);
    let mut scan = map.data.len() - 1;
    let mut rep = 100;

    while scan > 0 {
        if rep < 100 {
            let f = File::create(format!("debug/day9-output-{rep}.html")).unwrap();
            map.dump_data_html(f);
            rep += 1;
        }

        let mut l = 1;
        while scan > 0 && map.data[scan] == map.data[scan - 1] {
            l += 1;
            if scan > 0 {
                scan -= 1;
            } else {
                break;
            }
        }
        if let Some(idx) = map.first_fit(scan, l) {
            println!("moving file {} ({l} chars) to idx {idx}", map.data[scan]);
            let dst = map.move_data(idx, scan, l);
            if idx >= l {
                map.free_maps[idx - l].push(Reverse(dst + l));
            }
        }
        while scan > 0 {
            scan -= 1;
            if map.data[scan] != u16::MAX {
                break;
            }
        }
    }

    let v: u64 = map.data.iter().enumerate().fold(0, |acc, (n, c)| {
        if *c == u16::MAX {
            acc
        } else {
            acc + n as u64 * *c as u64
        }
    });
    println!("{}", v);
}

fn main() {
    let input = std::fs::read_to_string("input/d9.txt").unwrap();
    //let input = TEST;
    let mut v: u64 = 0;
    for b in input.trim().as_bytes() {
        v += (*b - 48) as u64;
    }
    let mut data: Vec<u16> = Vec::with_capacity(100000);
    part2(&input, data);
}

//static TEST: &str = "12345";
static TEST: &str = "2333133121414131402";
//static TEST: &str = "35904175134";
