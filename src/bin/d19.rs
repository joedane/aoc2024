use std::collections::{HashMap, HashSet};

fn try_match<'a>(word: &'a str, vocab: &Vec<&str>, memo: &mut HashMap<&'a str, bool>) -> bool {
    println!("try match {word}");
    if word.len() == 0 {
        return true;
    } else if let Some(s) = memo.get(word) {
        println!("found {word} in cache ({s})");
        return *s;
    } else {
        if vocab.iter().filter(|v| word.starts_with(*v)).any(|v| {
            println!("trying prefix {v}");
            try_match(&word[v.len()..], vocab, memo)
        }) {
            memo.insert(word, true);
            println!("matched {word}");
            return true;
        } else {
            memo.insert(word, false);
            return false;
        }
    }
}

fn match_all<'a>(word: &'a str, vocab: &Vec<&str>, memo: &mut HashMap<&'a str, usize>) -> usize {
    println!("try match {word}");
    if word.len() == 0 {
        return 1;
    } else if let Some(s) = memo.get(word) {
        println!("found {word} in cache ({s})");
        return *s;
    } else {
        let mut matches = 0;
        for v in vocab.iter().filter(|v| word.starts_with(*v)) {
            matches += match_all(&word[v.len()..], vocab, memo);
        }
        memo.insert(word, matches);
        return matches;
    }
}

fn part1() {
    //let input = TEST;
    let input = std::fs::read_to_string("input/d19.txt").unwrap();
    let mut lines = input.lines();
    if let Some(l) = lines.next() {
        let mut v: Vec<&str> = l.split(',').map(str::trim).collect();
        let _ = lines.next();
        let words: Vec<&str> = lines.map(str::trim).collect();
        v.sort_by(|v1, v2| v2.len().cmp(&v1.len()));
        println!("vocab: {:?}", v);
        println!("words: {:?}", words);

        let mut memo: HashMap<&str, bool> = Default::default();
        let r: usize = words
            .into_iter()
            .filter(|&w| try_match(w, &v, &mut memo))
            .count();

        println!("{r}");
    }
}

fn part2() {
    //let input = TEST;
    let input = std::fs::read_to_string("input/d19.txt").unwrap();
    let mut lines = input.lines();
    if let Some(l) = lines.next() {
        let mut v: Vec<&str> = l.split(',').map(str::trim).collect();
        let _ = lines.next();
        let words: Vec<&str> = lines.map(str::trim).collect();
        v.sort_by(|v1, v2| v2.len().cmp(&v1.len()));
        println!("vocab: {:?}", v);
        println!("words: {:?}", words);

        let mut memo: HashMap<&str, usize> = Default::default();
        let r: HashMap<&str, usize> = words
            .into_iter()
            .map(|w| (w, match_all(w, &v, &mut memo)))
            .collect();
        for w in &r {
            println!("word '{}' can be made {} ways", w.0, w.1);
        }
        println!("total: {}", r.values().sum::<usize>());
    }
}

fn main() {
    part2();
}

static TEST: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
