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

fn main() {
    //let input = TEST;
    let input = std::fs::read_to_string("input/d11.txt").unwrap();
    let mut stones: Vec<u64> = input
        .split_ascii_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();

    println!("start:\t{:?}", stones);
    for i in 0..75 {
        apply_part1_rules(&mut stones);
        if stones.len() < 25 {
            println!("after:\t{:?}", stones)
        }
        println!("after step {}, len is {}", i + 1, stones.len());
    }
    println!("{}", stones.len());
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
