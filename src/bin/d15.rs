use std::{fmt::Display, ops::Deref};

use utils::{BasicGrid, Coord, Dir};

#[derive(Clone, Copy, Debug)]
enum State {
    Wall,
    Box,
    Empty,
    Robot,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::Wall => "#",
                State::Box => "O",
                State::Empty => ".",
                State::Robot => "@",
            }
        )
    }
}

impl From<u8> for State {
    fn from(value: u8) -> Self {
        if value == b'#' {
            State::Wall
        } else if value == b'.' {
            State::Empty
        } else if value == b'@' {
            State::Robot
        } else if value == b'O' {
            State::Box
        } else {
            panic!();
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Dir15(Dir);

impl From<u8> for Dir15 {
    fn from(value: u8) -> Self {
        if value == b'<' {
            Dir15(Dir::Left)
        } else if value == b'>' {
            Dir15(Dir::Right)
        } else if value == b'^' {
            Dir15(Dir::Up)
        } else if value == b'v' {
            Dir15(Dir::Down)
        } else {
            panic!()
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct MoveState {
    start: Coord,
    end: Coord,
    dir: Dir,
}

fn check_movable(grid: &BasicGrid<State>, start: Coord, dir: Dir) -> Option<MoveState> {
    if let Some(c) = grid.next_pos(start, dir) {
        match *grid.at(c) {
            State::Wall => None,
            State::Empty => Some(MoveState {
                start: start,
                dir: dir,
                end: c,
            }),
            State::Box => {
                let mut probe = c;
                loop {
                    if let Some(next) = grid.next_pos(probe, dir) {
                        match *grid.at(next) {
                            State::Empty => {
                                break Some(MoveState {
                                    start: start,
                                    dir: dir,
                                    end: next,
                                })
                            }
                            State::Box => {
                                probe = next;
                                continue;
                            }
                            State::Wall => break None,
                            State::Robot => panic!(),
                        }
                    } else {
                        break None;
                    }
                }
            }
            State::Robot => panic!(),
        }
    } else {
        None
    }
}

fn move_items(grid: &mut BasicGrid<State>, move_state: MoveState) -> Coord {
    let mut to = move_state.end;
    let back_dir = move_state.dir.turn_right().turn_right();
    let mut from = grid.next_pos(to, back_dir).unwrap();
    loop {
        grid.swap(to, from);
        if from == move_state.start {
            break;
        }

        to = from;
        from = grid.next_pos(from, back_dir).unwrap();
    }
    to
}
fn apply_dir(grid: &mut BasicGrid<State>, mut robot_pos: Coord, dir: Dir15) -> Coord {
    if let Some(move_state) = check_movable(grid, robot_pos, dir.0) {
        robot_pos = move_items(grid, move_state);
    }
    robot_pos
}

fn main() {
    //let input = TEST1;
    let input = std::fs::read_to_string("input/d15.txt").unwrap();
    let p = input.find("\n\n").unwrap();
    let grid_input: Vec<&str> = input[0..p].trim().split("\n").collect();
    let mut grid: BasicGrid<State> = BasicGrid::new(&grid_input);
    let directions: Vec<Dir15> = input[p..]
        .as_bytes()
        .iter()
        .filter_map(|&c| {
            if c.is_ascii_whitespace() {
                None
            } else {
                Some(c.into())
            }
        })
        .collect();
    let mut robot_pos = grid
        .find_with(|s| matches!(*s, State::Robot))
        .pop()
        .unwrap();

    for dir in directions {
        robot_pos = apply_dir(&mut grid, robot_pos, dir);
        //println!("after applying {dir:?}:");
        //grid.display_all();
    }
    let res: usize = grid
        .find_with(|state| matches!(*state, State::Box))
        .iter()
        .map(|c| c.row * 100 + c.col)
        .sum();
    println!("{}", res);
}

static TEST: &str = r#"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;

static TEST1: &str = r#"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
