use std::cmp::Ordering;

pub fn solution() {
    // 'a' = utf8 - 96
    // 'A' = utf8 -
    // lowercase 1-26; uppercase 26-52


    let input = include_str!("../../inputs/day_3.txt")
        .lines()
        .map(|a|
            (a.to_string().as_bytes().to_owned()
                .iter()
                .map(|n| match n.cmp(&91) {
                    // lowercase
                    Ordering::Greater => n - 96,
                    Ordering::Less => n - 64 + 26,
                    Ordering::Equal => *n
                })
            )
                .collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    static mut COUNTS:[u8;53] = [0;53];
    //let mut counts:Vec<u8> = Vec::with_capacity(53);
    let mut coll_len;
    let mut total_a: i32 = 0;
    // println!("{:?}", input);
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
            for item in &mut COUNTS {
                *item = 0;
            }
        }
    }
    // part 2
    let mut total_b:i32 = 0;
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