use std::collections::HashSet;

fn try_match(word: &str, vocab: &Vec<&str>, memo: &mut HashSet<&str>) -> bool {
    println!("try_match on {}", word);
    let mut i = 0;
    let mut stack: Vec<(usize, usize)> = vec![];
    let mut v = 0;

    'i: while i < word.len() {
        while v < vocab.len() {
            println!("testing {} against {}", &word[i..], vocab[v]);
            if word[i..].starts_with(vocab[v]) {
                stack.push((i, v));
                i += vocab[v].len();
                v = 0;
                continue 'i;
            } else {
                v += 1;
            }
        }
        if let Some((saved_i, saved_v)) = stack.pop() {
            if saved_v < vocab.len() - 1 {
                v = saved_v + 1;
                i = saved_i;
                continue 'i;
            } else if saved_i < word.len() - 1 {
                i = saved_i + 1;
                v = 0;
                continue 'i;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    return true;
}
fn main() {
    //    let input = TEST;
    let input = std::fs::read_to_string("input/d19.txt").unwrap();
    let mut lines = input.lines();
    if let Some(l) = lines.next() {
        let mut v: Vec<&str> = l.split(',').map(str::trim).collect();
        let _ = lines.next();
        let words: Vec<&str> = lines.map(str::trim).collect();
        v.sort_by(|v1, v2| v2.len().cmp(&v1.len()));
        println!("vocab: {:?}", v);
        println!("words: {:?}", words);

        let r: usize = words.into_iter().filter(|&w| try_match(w, &v)).count();

        println!("{r}");
    }
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
