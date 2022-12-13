use serde::Deserialize;
use std::cmp::Ordering;
use serde_json;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum Line {
    Value(i32),
    Nest(Vec<Line>),
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Line {}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Line {
    fn cmp(&self, other: &Self) -> Ordering {
        use Line::*;
        match (self, other) {
            (Value(l), Value(r)) => l.cmp(r),
            (Nest(l), Nest(r)) => l.cmp(r),
            (Value(l), Nest(r)) => [Value(*l)][..].cmp(r),
            (Nest(l), Value(r)) => l.as_slice().cmp(&[Value(*r)]),
        }
    }
}

#[derive(Debug)]
struct Pair {
    a: Line,
    b: Line,
}

impl Pair {
    fn new(a: Line, b: Line) -> Pair {
        Pair { a, b }
    }
}

#[derive(Debug)]
struct Lines {
    pairs: Vec<Pair>,
}

impl Lines {
    fn new() -> Lines {
        Lines {
            pairs: Vec::new(),
        }
    }
    fn add(&mut self, pair: Pair) {
        self.pairs.push(pair);
    }
}

pub fn solution() {
    //let input = include_str!("input.txt").split("\n\n").collect::<Vec<&str>>();
    let input = include_str!("../../inputs/day_13.txt").split("\n\n").collect::<Vec<&str>>();
    let mut pairs = Lines::new();
    let mut extra_b_1 = serde_json::from_str::<Line>("[[2]]").unwrap();
    let mut extra_b_2 = serde_json::from_str::<Line>("[[6]]").unwrap();
    let mut part_b: Vec<Line> = Vec::from([extra_b_2.clone(), extra_b_1.clone()]);

    for line in input {
        let mut temp = line.split("\n").collect::<Vec<&str>>();
        let a = serde_json::from_str::<Line>(temp[0]).unwrap();
        let b = serde_json::from_str::<Line>(temp[1]).unwrap();
        part_b.push(a.clone());
        part_b.push(b.clone());
        let pair = Pair::new(a, b);

        pairs.add(pair);
    }

    let mut total = 0;
    for (i, pair) in pairs.pairs.iter().enumerate() {
        if pair.a < pair.b {
            total += i + 1;
        }
    }
    println!("Part A: {}", total);

    part_b.sort_unstable();
    let mut total_b = 1;

    for (i, val) in part_b.iter().enumerate() {
        if (*val == extra_b_1) || (*val == extra_b_2) {
            total_b *= (i + 1);
        }
    }

    println!("Part B: {}", total_b);
}

