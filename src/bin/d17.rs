use itertools::Itertools;
use std::{num::ParseIntError, str::FromStr};

#[derive(Clone)]
struct MachineState {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    ip: usize,
    source: Vec<usize>,
    program: Vec<Op>,
}

impl MachineState {
    fn init(reg_a: usize, reg_b: usize, reg_c: usize, source: Vec<ThreeBit>) -> Self {
        Self {
            reg_a,
            reg_b,
            reg_c,
            ip: 0,
            source: source.iter().map(|tb| tb.as_usize()).collect(),
            program: parse_instructions(source),
        }
    }
}

impl FromStr for MachineState {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_reg(s: &str, label: &str) -> Result<usize, &'static str> {
            let Some(i) = s.find(label) else {
                return Err("no register");
            };
            let i = i + label.len();
            let Some(endl) = s[i..].find("\n") else {
                return Err("no register");
            };
            let Ok(reg_a) = usize::from_str(&s[i..(i + endl)]) else {
                return Err("no register");
            };
            Ok(reg_a)
        }
        let reg_a = parse_reg(s, "Register A: ")?;
        let reg_b = parse_reg(s, "Register B: ")?;
        let reg_c = parse_reg(s, "Register C: ")?;
        let Some(i) = s.find("Program: ") else {
            return Err("no program");
        };
        let v: Result<Vec<ThreeBit>, &'static str> = s[i + "Program: ".len()..]
            .split(',')
            .map(|s| ThreeBit::from_str(s).map_err(|e| "no program"))
            .collect();
        let v = v?;
        Ok(MachineState::init(reg_a, reg_b, reg_c, v))
    }
}

#[derive(Clone, Copy, Debug)]
enum Combo {
    Literal(u8),
    RegA,
    RegB,
    RegC,
}

#[derive(Clone, Copy, Debug)]
struct ThreeBit(u8);

impl ThreeBit {
    fn as_usize(&self) -> usize {
        self.0 as usize
    }
}
impl From<u32> for ThreeBit {
    fn from(value: u32) -> Self {
        let v: u8 = value.try_into().unwrap();
        ThreeBit(v & 0o7)
    }
}

impl FromStr for ThreeBit {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .nth(0)
            .ok_or("bad character")
            .map(|v| v.to_digit(10).unwrap().into())
    }
}

#[derive(Debug, Clone)]
enum Op {
    ADV(Combo),
    BXL(ThreeBit),
    BST(Combo),
    JNZ(ThreeBit),
    BXC,
    OUT(Combo),
    BDV(Combo),
    CDV(Combo),
}

fn parse_combo(operand: ThreeBit) -> Combo {
    let v = operand.0;
    if v < 4 {
        Combo::Literal(v)
    } else if v == 4 {
        Combo::RegA
    } else if v == 5 {
        Combo::RegB
    } else if v == 6 {
        Combo::RegC
    } else {
        panic!("bad combo: {:?}", operand);
    }
}
fn parse_instruction(opcode: ThreeBit, operand: ThreeBit) -> Op {
    match opcode {
        ThreeBit(0) => Op::ADV(parse_combo(operand)),
        ThreeBit(1) => Op::BXL(operand),
        ThreeBit(2) => Op::BST(parse_combo(operand)),
        ThreeBit(3) => Op::JNZ(operand),
        ThreeBit(4) => Op::BXC,
        ThreeBit(5) => Op::OUT(parse_combo(operand)),
        ThreeBit(6) => Op::BDV(parse_combo(operand)),
        ThreeBit(7) => Op::CDV(parse_combo(operand)),
        _ => panic!("invalid opcode: {:?}", opcode),
    }
}

fn parse_instructions<I: IntoIterator<Item = ThreeBit>>(items: I) -> Vec<Op> {
    let mut r = vec![];
    for mut pair in &items.into_iter().chunks(2) {
        match (pair.next(), pair.next()) {
            (Some(p1), Some(p2)) => r.push(parse_instruction(p1, p2)),
            _ => panic!(),
        }
    }
    r
}

fn decode_operand(ms: &MachineState, c: Combo) -> usize {
    match c {
        Combo::Literal(l) => l as usize,
        Combo::RegA => ms.reg_a,
        Combo::RegB => ms.reg_b,
        Combo::RegC => ms.reg_c,
    }
}

fn run_to_halt(ms: &mut MachineState) -> Vec<usize> {
    let mut r = vec![];
    loop {
        if let Some(op) = ms.program.get(ms.ip) {
            match op {
                Op::ADV(combo) => {
                    let operand: u32 = decode_operand(ms, *combo).try_into().unwrap();
                    let d: usize = ms.reg_a / 2_usize.pow(operand);
                    ms.reg_a = d;
                }
                Op::BXL(three_bit) => {
                    ms.reg_b = ms.reg_b ^ three_bit.as_usize();
                }
                Op::BST(combo) => {
                    let operand = decode_operand(ms, *combo);
                    ms.reg_b = operand % 8;
                }
                Op::JNZ(three_bit) => {
                    if ms.reg_a != 0 {
                        ms.ip = three_bit.as_usize();
                        continue;
                    }
                }
                Op::BXC => {
                    ms.reg_b = ms.reg_b ^ ms.reg_c;
                }
                Op::OUT(combo) => {
                    let operand = decode_operand(ms, *combo) % 8;
                    r.push(operand);
                }
                Op::BDV(combo) => {
                    let operand: u32 = decode_operand(ms, *combo).try_into().unwrap();
                    let d: usize = ms.reg_a / 2_usize.pow(operand);
                    ms.reg_b = d;
                }
                Op::CDV(combo) => {
                    let operand: u32 = decode_operand(ms, *combo).try_into().unwrap();
                    let d: usize = ms.reg_a / 2_usize.pow(operand);
                    ms.reg_c = d;
                }
            }
            ms.ip += 1;
        } else {
            break;
        }
    }
    r
}

fn part1(mut ms: MachineState) {
    let v = run_to_halt(&mut ms);
    println!("{}", v.into_iter().join(","));
}

fn try_this(ms: MachineState, a: usize, idx: usize) -> Option<usize> {
    for try_a in 0..8 {
        let mut try_ms = ms.clone();
        try_ms.reg_a = (a << 3) | try_a;
        let v = run_to_halt(&mut try_ms);
        if v == ms.source[idx..] {
            if idx == 0 {
                return Some((a << 3) | try_a);
            } else {
                if let Some(a) = try_this(ms.clone(), (a << 3) | try_a, idx - 1) {
                    return Some(a);
                }
            }
        }
    }
    return None;
}

fn part2(mut ms: MachineState) {
    let idx = ms.source.len() - 1;
    if let Some(a) = try_this(ms, 0, idx) {
        println!("{}", a);
    } else {
        println!("None");
    }
}

fn main() {
    //let input = TEST1;
    let input = std::fs::read_to_string("input/d17.txt").unwrap();
    let mut ms: MachineState = input.parse().unwrap();
    part2(ms);
}

static TEST: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

static TEST1: &str = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;

/*
adv 3
out A % 8
jnz 0

000 011 100 101 101 000

Program: 2,4,1,2,7,5,4,3,0,3,1,7,5,5,3,0

bst A   B:=A%8      (2, 4)
bxl 2   B:=B^2      (1, 2)
cdv B   C:=A>>B     (7, 5)
bxc     B:=B^C      (4, 3)
adv 3   A:=A>>3     (0, 3)
bxl 7   B:=B^7      (1, 7)
out B               (5, 5)
jnz 0               (3, 0)

*/
