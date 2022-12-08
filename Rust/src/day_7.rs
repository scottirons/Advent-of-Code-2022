use std::collections::{ HashMap, HashSet };
use std::collections::hash_map::Entry;

pub fn solution() {
    let mut input: Vec<&str> = include_str!("../../inputs/day_7.txt").split('\n').collect();
    input.pop();

    //println!("{:?}", input);

    let mut curr_path: Vec<String> = Vec::new();
    let mut all_dirs: HashMap<Vec<String>, i32> = HashMap::new();

    for val in input.iter() {
        let input_as_string = String::from(*val);
        if &input_as_string[0..4] == "$ cd" {
            if &input_as_string[5..] == ".." {
                curr_path.pop();
            } else {
                curr_path.push(String::from(&input_as_string[5..]));
                all_dirs.insert(curr_path.clone(), 0);
            }
        } else if &input_as_string[0..3] != "dir" && &input_as_string[0..1] != "$" {
            let first_string: Vec<&str> = input_as_string.split_whitespace().take(1).collect();
            let add_val = first_string[0].parse::<i32>().unwrap();
            let mut clone_path = curr_path.clone();
            while clone_path.len() > 0 {
                all_dirs.entry(clone_path.clone())
                    .and_modify(|e| {*e += add_val}).or_insert(0);
                clone_path.pop();
            }

        }
    }
    let mut total_a: i32 = 0;
    let mut curr_min = i32::MAX;
    let min_rem = all_dirs.get(&*vec![String::from("/")]).unwrap() - 40000000;

    for (_, val) in all_dirs.iter() {
        if *val < 100000 {
            total_a += *val;
        }
        if (*val <= curr_min) && (*val >= min_rem) {
            curr_min = *val;
        }
    }
    println!("Total A: {}\nTotal B: {}", total_a, curr_min);
}