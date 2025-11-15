use std::{collections::HashMap, str::FromStr, sync::LazyLock};

struct Code {
    code_chars: [char; 4],
}

#[derive(Clone, Copy, Debug)]
struct PadState {
    x_pos: usize,
    y_pos: usize,
}

struct NumericPad(PadState);

static NUM_PAD_POSITIONS: LazyLock<HashMap<char, PadState>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert('7', PadState { x_pos: 0, y_pos: 0 });
    m.insert('8', PadState { x_pos: 1, y_pos: 0 });
    m.insert('9', PadState { x_pos: 2, y_pos: 0 });
    m.insert('4', PadState { x_pos: 0, y_pos: 1 });
    m.insert('5', PadState { x_pos: 1, y_pos: 1 });
    m.insert('6', PadState { x_pos: 2, y_pos: 1 });
    m.insert('1', PadState { x_pos: 0, y_pos: 2 });
    m.insert('2', PadState { x_pos: 1, y_pos: 2 });
    m.insert('3', PadState { x_pos: 2, y_pos: 2 });
    m.insert('0', PadState { x_pos: 3, y_pos: 1 });
    m.insert('A', PadState { x_pos: 3, y_pos: 3 });
    m
});
struct DirectionalPad(PadState);

enum DirPadControls {
    Up,
    Down,
    Left,
    Right,
    Activate,
}
trait PadMove {
    fn pad_move(&mut self, move_to: PadState) -> Vec<DirPadControls>;
}

impl PadMove for NumericPad {
    fn pad_move(&mut self, move_to: PadState) -> Vec<DirPadControls> {
        vec![]
    }
}

impl PadMove for DirectionalPad {
    fn pad_move(&mut self, move_to: PadState) -> Vec<DirPadControls> {
        vec![]
    }
}

struct ControlChain<T> {
    parent: Option<T>,
}

impl FromStr for Code {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 4 {
            return Err(());
        }
        Ok(Code {
            code_chars: [chars[0], chars[1], chars[2], chars[3]],
        })
    }
}

fn part1(input: &str) -> String {
    let codes: Vec<Code> = input.trim().lines().map(|s| s.parse().unwrap()).collect();
    for code in codes {}

    "".to_owned()
}

fn main() {
    //let input = include_str!("../../input/d21.txt");
    let input = TEST;
    println!("{}", part1(input));
}

static TEST: &str = r#"
029A
980A
179A
456A
379A"#;
