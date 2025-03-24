use std::collections::HashMap;

fn apply_part1_rules(stones: &mut Vec<u64>) {
    let mut i = 0_usize;
    while i < stones.len() {
        let stone = stones[i];
        if stone == 0 {
            stones[i] = 1;
        } else if stone.ilog10() % 2 == 1 {
            let s = stone.to_string();
            let (l, u): (u64, u64) = (
                s[0..(s.len() / 2)].parse().unwrap(),
                s[(s.len() / 2)..].parse().unwrap(),
            );
            stones[i] = u;
            stones.insert(i, l);
            i += 1;
        } else {
            stones[i] *= 2024;
        }
        i += 1;
    }
}

fn part1(mut stones: Vec<u64>) {
    println!("start:\t{:?}", stones);
    for i in 0..25 {
        apply_part1_rules(&mut stones);
        if stones.len() < 25 {
            println!("after:\t{:?}", stones)
        }
        println!("after step {}, len is {}", i + 1, stones.len());
    }
    println!("{}", stones.len());
}

enum OneOrTwo<T> {
    One(T),
    Two(T, T),
}

struct OneOrTwoIter<T> {
    data: OneOrTwo<T>,
    reads: u8,
}

impl<T> OneOrTwoIter<T> {
    fn new(data: OneOrTwo<T>) -> Self {
        Self { data, reads: 0 }
    }
}
impl<T> Iterator for OneOrTwoIter<T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.data {
            OneOrTwo::One(v) => {
                if self.reads > 0 {
                    None
                } else {
                    self.reads += 1;
                    Some(v)
                }
            }
            OneOrTwo::Two(v1, v2) => {
                if self.reads == 0 {
                    self.reads = 1;
                    Some(v1)
                } else if self.reads == 1 {
                    self.reads = 2;
                    Some(v2)
                } else {
                    None
                }
            }
        }
    }
}
impl<T> IntoIterator for OneOrTwo<T>
where
    T: Copy,
{
    type Item = T;

    type IntoIter = OneOrTwoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        OneOrTwoIter::new(self)
    }
}
fn part2(starting_stones: Vec<u64>) {
    use OneOrTwo::*;
    let mut cache: HashMap<u64, OneOrTwo<_>> = Default::default();
    let mut stones: HashMap<u64, u64> = starting_stones.into_iter().map(|v| (v, 1)).collect();
    for step in 0..75 {
        let mut new_stones: HashMap<u64, u64> = Default::default();
        //println!("\n\n{:?}\n", stones);
        stones.into_iter().for_each(|(stone_val, count)| {
            let next = cache.entry(stone_val).or_insert_with_key(|val| {
                if *val == 0 {
                    One(1)
                } else if val.ilog10() % 2 == 1 {
                    let s = val.to_string();
                    let (l, u): (u64, u64) = (
                        s[0..(s.len() / 2)].parse().unwrap(),
                        s[(s.len() / 2)..].parse().unwrap(),
                    );
                    Two(l, u)
                } else {
                    One(val * 2024)
                }
            });
            match next {
                One(v) => {
                    *new_stones.entry(*v).or_default() += count;
                }
                Two(v1, v2) => {
                    *new_stones.entry(*v1).or_default() += count;
                    *new_stones.entry(*v2).or_default() += count;
                }
            }
        });
        stones = new_stones;
        let r: u64 = stones.values().sum();
        println!("at step {}: {}", step + 1, r);
    }
}

fn main() {
    //let input = TEST;
    let input = std::fs::read_to_string("input/d11.txt").unwrap();
    let mut stones: Vec<u64> = input
        .split_ascii_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();
    part2(stones);
}

//static TEST: &str = "0 1 10 99 999";
static TEST: &str = "125 17";

#[cfg(test)]
mod test {

    #[test]
    fn test_log() {
        assert_eq!(10_u64.ilog10(), 1);
        assert_eq!(9_u64.ilog10(), 0);
        assert_eq!(1_u64.ilog10(), 0);
        assert_eq!(100_u64.ilog10(), 2);
        assert_eq!(101_u64.ilog10(), 2);
        assert_eq!(1003_u64.ilog10(), 3);
    }
}
