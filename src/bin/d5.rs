#[derive(Debug)]
struct Rule {
    earlier: u32,
    later: u32,
}

impl Rule {
    fn new(earlier: u32, later: u32) -> Self {
        Self { earlier, later }
    }

    fn is_ordered(&self, pageset: &[u32]) -> bool {
        let (mut earlier_i, mut later_i): (Option<usize>, Option<usize>) = (None, None);
        for i in 0..pageset.len() {
            if pageset[i] == self.earlier {
                earlier_i = Some(i);
            } else if pageset[i] == self.later {
                later_i = Some(i);
            }
        }
        earlier_i.is_none() || later_i.is_none() || earlier_i < later_i
    }
}

struct Part1;

impl Part1 {
    fn run(pages: Vec<Box<[u32]>>, rules: Vec<Rule>) -> u32 {
        pages
            .iter()
            .filter(|l| rules.iter().all(|r| r.is_ordered(l)))
            .map(|l| l[l.len() / 2])
            .sum()
    }
}

struct Part2;

impl Part2 {
    fn is_ordered(pages: &[u32], rules: &[Rule]) -> bool {
        rules.iter().all(|r| r.is_ordered(pages))
    }
    fn order(pages: &mut [u32], rules: &Vec<Rule>) {
        while !Part2::is_ordered(pages, rules) {
            for rule in rules {
                let (mut earlier_i, mut later_i): (Option<usize>, Option<usize>) = (None, None);
                for i in 0..pages.len() {
                    if pages[i] == rule.earlier {
                        earlier_i = Some(i);
                    } else if pages[i] == rule.later {
                        later_i = Some(i);
                    }
                }
                if earlier_i.is_some() && later_i.is_some() && earlier_i > later_i {
                    //println!("reordering on rule {:?}", rule);
                    //println!("before: {:?}", pages);
                    let tmp = pages[earlier_i.unwrap()];
                    pages[earlier_i.unwrap()] = pages[later_i.unwrap()];
                    pages[later_i.unwrap()] = tmp;
                    //println!("after: {:?}", pages);
                }
            }
        }
    }
    fn run(pages: Vec<Box<[u32]>>, rules: Vec<Rule>) -> u32 {
        let mut v: u32 = 0;
        pages
            .into_iter()
            .filter(|l| rules.iter().any(|r| !r.is_ordered(l)))
            .for_each(|mut l| {
                Part2::order(&mut l, &rules);
                println!("reordered: {:?}", l);
                v += l[l.len() / 2];
            });
        v
    }
}
fn main() {
    let mut rules: Vec<Rule> = vec![];
    let mut pages: Vec<Box<[u32]>> = vec![];
    for line in std::fs::read_to_string("input/d5.txt")
        .unwrap()
        .lines()
        .map(str::trim)
    {
        if line.len() > 0 {
            match line.find("|") {
                None => {
                    let v: Vec<u32> = line
                        .split(',')
                        .map(|s| s.trim().parse().expect(line))
                        .collect();
                    pages.push(v.into_boxed_slice());
                }
                Some(i) => rules.push(Rule::new(
                    line[0..i].parse().unwrap(),
                    line[i + 1..].parse().unwrap(),
                )),
            }
        }
    }

    println!("{}", Part2::run(pages, rules));
}

static TEST: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
