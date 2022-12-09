use std::collections::HashSet;

pub fn solution() {

    let input: Vec<&str> = include_str!("../../inputs/day_9.txt").split("\n")
        .collect();

    let (part_a, part_b) = follow_path(input);
    println!("Part A: {}\nPart B: {}", part_a, part_b);
}

fn follow_path(input: Vec<&str>) -> (i32, i32) {
    let mut segments = vec![(0, 0); 10];
    let mut second: HashSet<(i32, i32)> = HashSet::new();
    let mut ninth: HashSet<(i32, i32)> = HashSet::new();
    second.insert((0, 0));
    ninth.insert((0, 0));

    for line in input.iter() {
        let (direction, amount) = line.trim().split_once(" ").unwrap();

        for _ in 0..amount.parse().unwrap() {
            match direction {
                "U" => segments[0].1 += 1,
                "D" => segments[0].1 -= 1,
                "L" => segments[0].0 -= 1,
                "R" => segments[0].0 += 1,
                _ => panic!("Invalid direction!")
            }
            for i in 1..segments.len() {
                if (((segments[i].0 - segments[i - 1].0) as i32).abs() == 2) &&
                    (((segments[i].1 - segments[i - 1].1) as i32).abs() == 2) {
                    segments[i].0 = (segments[i].0 + segments[i - 1].0) / 2;
                    segments[i].1 = (segments[i].1 + segments[i - 1].1) / 2;
                } else if ((segments[i].0 - segments[i - 1].0) as i32).abs() == 2 {
                    segments[i].1 = segments[i - 1].1;
                    segments[i].0 = (segments[i].0 + segments[i - 1].0) / 2;
                } else if ((segments[i].1 - segments[i - 1].1) as i32).abs() == 2 {
                    segments[i].0 = segments[i - 1].0;
                    segments[i].1 = (segments[i].1 + segments[i - 1].1) / 2;
                }
            }
            second.insert((segments[1].0, segments[1].1));
            ninth.insert((segments[9].0, segments[9].1));
        }
    }


    (second.len() as i32, ninth.len() as i32)
}