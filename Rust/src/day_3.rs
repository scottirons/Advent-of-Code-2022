use std::cmp::Ordering;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn solution() {

    let input = include_str!("../../inputs/day_3.txt")
        //let input = include_str!("input.txt")
        .lines()
        .map(|a|
            (a.to_string().as_bytes().to_owned()
                .iter()
                .map(|n| match n.cmp(&91) {
                    // turn into fancy order
                    Ordering::Greater => n - 96,
                    Ordering::Less => n - 38,
                    Ordering::Equal => *n
                })).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let mut total_a: i32 = 0;           // results
    let mut total_b:i32 = 0;
    let mut half: usize;                // partway indices for part a
    let mut both_halves;
    let mut iter_sum;              // sum of intersection part a

    // part b stuff
    let mut remainder;
    let mut sets: Vec<HashSet<&u8>> = Vec::with_capacity(3);
    sets.resize(3, Default::default());

    for (i, collection) in input.iter().enumerate() {
        // first half
        half = collection.len() / 2;
        let first_half: HashSet<u8> = collection.into_iter().take(half).cloned().collect();
        let second_half: HashSet<u8> = collection.into_iter().skip(half).cloned().collect();
        both_halves = first_half.intersection(&second_half);
        iter_sum = both_halves.fold(0, |acc, x| acc + *x as i32);
        total_a += iter_sum as i32;

        // second half
        remainder = (i).rem_euclid(3);
        let who_hash: HashSet<&u8> = collection.into_iter().collect();
        sets[remainder] = who_hash;
        // end of a triplet
        if remainder == 2 {
            let id_value = sets[0].intersection(&sets[1]).find(|it| sets[2].contains(**it));
            total_b += **id_value.unwrap() as i32;
        }
    }
    println!("Part a total: {}\nPart b total: {}", total_a, total_b);

    total_a = 0;
    total_b = 0;
    // OLD VERSION
    static mut COUNTS:[u8;53] = [0;53];
    let mut coll_len;
    // part 1
    unsafe {
        for collection in input.iter() {
            coll_len = collection.len();
            for i in 0..coll_len / 2 {
                COUNTS[collection[i] as usize] += 1;
            }
            for i in coll_len / 2..coll_len {
                if COUNTS[collection[i] as usize] > 0 {
                    total_a += collection[i] as i32;
                    COUNTS[collection[i] as usize] = 0;
                }
            }
            // clear
            for item in &mut COUNTS {
                *item = 0;
            }
        }
    }
    // part 2
    unsafe {
        let mut curr_count = 0;
        for collection in input.iter() {
            curr_count += 1;
            for ele in collection.iter() {
                if curr_count == 1 {
                    COUNTS[*ele as usize] = 1;
                } else if curr_count == 2 {
                    if COUNTS[*ele as usize] == 1 {
                        COUNTS[*ele as usize] = 2;
                    }
                } else if COUNTS[*ele as usize] == 2 {
                    total_b += *ele as i32;
                    COUNTS[*ele as usize] = 0;
                }
            }
            if curr_count == 3 {
                for item in &mut COUNTS {
                    *item = 0;
                }
                curr_count = 0;
            }
        }
    }
    println!("Part a total: {}\nPart b total: {}", total_a, total_b);

}