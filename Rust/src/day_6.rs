use itertools::Itertools;

pub fn solution() {

    let n = 4;
    let answer = include_str!("../../inputs/day_6.txt")
        .trim()
        .chars()
        .collect::<Vec<char>>()
        .windows(n)
        .enumerate()
        .filter(|(_, window)| window.into_iter().unique().count() == n)
        .map(|(i, _)| i + n)
        .next().unwrap();

    println!("{:?}", answer);

}