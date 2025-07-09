use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::ops::Add;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Pos {
    x: u8,
    y: u8,
}

impl Pos {
    fn new(x: u8, y: u8) -> Self {
        Pos { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Cost(u32);

impl Default for Cost {
    fn default() -> Self {
        Self(u32::MAX)
    }
}

impl Add<Cost> for Cost {
    type Output = Cost;

    fn add(self, rhs: Cost) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct PosItem {
    pos: Pos,
    cost: Cost,
    h: Cost,
}

impl PosItem {
    fn new(x: u8, y: u8, cost: Cost, h: Cost) -> Self {
        PosItem {
            pos: Pos { x, y },
            cost,
            h,
        }
    }
}
impl PartialEq for PosItem {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.cost == other.cost
    }
}

impl Eq for PosItem {}

impl PartialOrd for PosItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost) // reverse ord on cost
    }
}

impl Ord for PosItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn neighbors(pos: Pos, objs: &HashSet<Pos>) -> Vec<Pos> {
    let mut v = Vec::with_capacity(4);
    if pos.x > 0 {
        let p = Pos::new(pos.x - 1, pos.y);
        if !objs.contains(&p) {
            v.push(p);
        }
    }
    if pos.x < WIDTH - 1 {
        let p = Pos::new(pos.x + 1, pos.y);
        if !objs.contains(&p) {
            v.push(p);
        }
    }
    if pos.y > 0 {
        let p = Pos::new(pos.x, pos.y - 1);
        if !objs.contains(&p) {
            v.push(p);
        }
    }
    if pos.y < HEIGHT - 1 {
        let p = Pos::new(pos.x, pos.y + 1);
        if !objs.contains(&p) {
            v.push(p);
        }
    }
    v
}

static WIDTH: u8 = 71;
static HEIGHT: u8 = 71;
static NUM_OBJS: usize = 1024;

fn dump(scores: &HashMap<Pos, Cost>, objs: &HashSet<Pos>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let p = Pos::new(x, y);
            let c = if objs.contains(&p) {
                '#'
            } else {
                scores
                    .get(&p)
                    .and_then(|c: &Cost| Some(c.0))
                    .or(Some(0))
                    .map(|v| {
                        if v < 10 {
                            (v as u8 + 48) as char
                        } else if v < 15 {
                            (v as u8 + 97) as char
                        } else {
                            '*'
                        }
                    })
                    .unwrap()
            };

            print!("{}", c);
        }
        println!("");
    }
    println!("");
}

// A* algorithm
fn shortest_path(objs: &HashSet<Pos>) -> Option<Cost> {
    let mut openSet: BinaryHeap<PosItem> = Default::default();
    let h = |p: Pos| Cost(((WIDTH - p.x) + (HEIGHT - p.y)) as u32);
    openSet.push(PosItem::new(0, 0, Cost(0), h(Pos::new(0, 0))));
    let mut cameFrom: HashMap<Pos, Pos> = Default::default();
    let mut gScore: HashMap<Pos, Cost> = Default::default();
    gScore.insert(Pos::new(0, 0), Cost(0));
    let GOAL = Pos::new(WIDTH - 1, HEIGHT - 1);
    let mut fScore: HashMap<Pos, Cost> = Default::default();
    fScore.insert(Pos::new(0, 0), h(Pos::new(0, 0)));

    while let Some(current) = openSet.pop() {
        //       dump(&gScore, &objs);
        if current.pos == GOAL {
            return Some(current.cost);
        }
        for n in neighbors(current.pos, &objs) {
            let tentative_g = if let Some(g_score) = gScore.get(&current.pos) {
                Cost(g_score.0 + 1)
            } else {
                Cost(u32::MAX)
            };
            if tentative_g < *gScore.get(&n).unwrap_or(&Cost(u32::MAX)) {
                cameFrom.insert(n, current.pos);
                gScore.insert(n, tentative_g);
                let new_f = tentative_g + h(n);
                fScore.insert(n, new_f);
                if !openSet.iter().any(|e| e.pos == n) {
                    openSet.push(PosItem::new(n.x, n.y, tentative_g, new_f));
                }
            }
        }
    }
    return None;
}

fn part1() {
    //let input = TEST;
    let input = std::fs::read_to_string("input/d18.txt").unwrap();
    let objs: HashSet<Pos> = input
        .lines()
        .take(NUM_OBJS)
        .map(|s| {
            let i = s.find(',').unwrap();
            Pos {
                x: s[0..i].parse().unwrap(),
                y: s[i + 1..].parse().unwrap(),
            }
        })
        .collect();

    match shortest_path(&objs) {
        Some(cost) => println!("Cost: {}", cost.0),
        None => println!("No path"),
    }
}

fn part2() {
    fn parse_pos(s: &str) -> Pos {
        let i = s.find(',').unwrap();
        Pos {
            x: s[0..i].parse().unwrap(),
            y: s[i + 1..].parse().unwrap(),
        }
    }
    //let input = TEST;
    let input = std::fs::read_to_string("input/d18.txt").unwrap();
    let mut input_iter = input.lines();
    let mut objs: HashSet<Pos> = input_iter.by_ref().take(NUM_OBJS).map(parse_pos).collect();
    let mut i = 1024;
    let p = loop {
        let next_obj = input_iter.next().map(parse_pos).unwrap();
        i += 1;
        println!("{i}");
        objs.insert(next_obj);
        match shortest_path(&objs) {
            Some(_) => continue,
            None => {
                println!("No path at pos: {:?}", next_obj);
                break;
            }
        }
    };
}

fn main() {
    part2();
}

static TEST: &str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;
