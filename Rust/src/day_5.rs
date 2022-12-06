

pub fn solution() {
    let mut input = include_str!("../../inputs/day_5.txt")
        .split_once("\n\n")
        .map(|(a, b)|
            (a.split("\n").collect::<Vec<&str>>(),
             b.split("\n")
                 .map(|n|
                     n.split(" ").skip(1).step_by(2)
                         .map(|m| m.parse::<i32>().unwrap())
                         .collect::<Vec<i32>>())
                 .collect::<Vec<Vec<i32>>>()))
        .unwrap(); input.1.pop();

    let mut stacc: Vec<Vec<char>> = vec![];
    for _ in 0..10 {
        stacc.push(vec![]);
    }

    // add values to stacc :]
    for row in input.0.iter().rev().skip(1) {
        for (j, char) in row.chars().enumerate() {
            if ((j as i32 - 1).rem_euclid(4) == 0) & (char != ' ') {
                stacc[j / 4 + 1].push(char);
            }
        }
    }

    let mut stacc_2_electric_boogaloo = stacc.to_vec();

    // do da poppin n pushing n stuff : )
    let mut temp = vec![];
    for instruction in input.1.iter() {
        for i in 0..instruction[0] {
            let moved_box = stacc[instruction[1] as usize].pop().unwrap();
            let moved_box_2 = stacc_2_electric_boogaloo[instruction[1] as usize].pop().unwrap();
            temp.push(moved_box_2);
            stacc[instruction[2] as usize].push(moved_box);
        }
        for _ in 0..temp.len() {
            let moved_box = temp.pop().unwrap();
            stacc_2_electric_boogaloo[instruction[2] as usize].push(moved_box);
        }
    }
    print!("Part 1 solution: ");
    for pile in stacc.iter().skip(1) {
        if let Some(char) = pile.last() {
            print!("{}", char);
        }
    } println!();

    print!("Part 2 solution: ");
    for pile in stacc_2_electric_boogaloo.iter().skip(1) {
        if let Some(char) = pile.last() {
            print!("{}", char);
        }
    }
}