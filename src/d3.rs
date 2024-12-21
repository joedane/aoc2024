#[derive(Debug)]
struct Mul {
    op1: u32,
    op2: u32,
}

impl Mul {
    fn new(op1: u32, op2: u32) -> Self {
        Self { op1, op2 }
    }
}
fn parse_args(s: &[u8]) -> Option<(u32, u32, usize)> {
    let mut i = 0usize;
    let bytes = s;
    while i < s.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if let Ok(op1) = std::str::from_utf8(&s[0..i]).unwrap().parse::<u32>() {
        if i + 1 < s.len() && s[i] == b',' {
            i += 1;
            let mut j = i;
            while j < s.len() && bytes[j].is_ascii_digit() {
                j += 1;
            }
            if j < s.len() && bytes[j] == b')' {
                if let Ok(op2) = std::str::from_utf8(&s[i..j]).unwrap().parse::<u32>() {
                    return Some((op1, op2, j + 1));
                }
            }
        }
    }
    None
}
fn parse_line(line: &str) -> Vec<Mul> {
    let mut v: Vec<Mul> = vec![];
    let mut start = 0;
    let bytes = line.as_bytes();
    while let Some(i) = line[start..].find("mul(") {
        if let Some((op1, op2, next)) = parse_args(&bytes[start + i + 4..]) {
            v.push(Mul::new(op1, op2));
            start = start + i + 4 + next;
        } else {
            start = start + i + 4;
        }
    }
    v
}

pub fn main() {
    //let v = parse_line(TEST);
    let v = parse_line(&std::fs::read_to_string("input/d3.txt").unwrap());
    let r: u32 = v.iter().map(|m| m.op1 * m.op2).sum();
    println!("{}", r);
}

static TEST: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
