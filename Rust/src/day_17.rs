use std::collections::{ HashSet };

fn can_left(shape: &Vec<(i32, i32)>, occupied: &HashSet<(i32, i32)>) -> bool {
    for val in shape {
        if val.0 == 0 {
            return false;
        }
        if occupied.contains(&(val.0 - 1, val.1)) {
            return false;
        }
    }
    true
}

fn can_right(shape: &Vec<(i32, i32)>, occupied: &HashSet<(i32, i32)>) -> bool {
    for val in shape {
        if val.0 == 6 {
            return false;
        }
        if occupied.contains(&(val.0 + 1, val.1)) {
            return false;
        }
    }
    true
}

fn can_down(shape: &Vec<(i32, i32)>, occupied: &HashSet<(i32, i32)>) -> bool {
    for val in shape {
        if occupied.contains(&(val.0, val.1 - 1)) {
            return false;
        }
    }
    true
}

pub fn solution() {
    let mut input: Vec<i8> = include_str!("input.txt").as_bytes().iter()
        .map(|n| match n {
            &60 => -1,
            &62 => 1,
            _ => panic!("ahh")
        })
        .collect();
    let mut input: Vec<i8> = include_str!("../../inputs/day_17.txt").as_bytes().iter()
        .map(|n| match n {
            &60 => -1,
            &62 => 1,
            _ => panic!("ahh")
        })
        .collect();
    //println!("{:?}", input);

    let mut occupied = HashSet::from([(0, -1), (1, -1), (2, -1), (3, -1), (4, -1), (5, -1), (6, -1)]);
    let mut shapes = vec![];
    let shape_a = vec![(2, 3), (3, 3), (4, 3), (5, 3)];
    shapes.push(shape_a);
    let shape_b = vec![(3, 3), (2, 4), (3, 4), (4, 4), (3, 5)];
    shapes.push(shape_b);
    let shape_c = vec![(2, 3), (3, 3), (4, 3), (4, 4), (4, 5)];
    shapes.push(shape_c);
    let shape_d = vec![(2, 3), (2, 4), (2, 5), (2, 6)];
    shapes.push(shape_d);
    let shape_e = vec![(2, 3), (2, 4), (3, 3), (3, 4)];
    shapes.push(shape_e);

    let mut path_i = 0;
    let mut curr_top = 0;
    let mut lr = 0;
    for rock in 0..2022 {
        // println!("{}", curr_top);
        let mut curr_rock = shapes[rock % 5].clone();
        for mut val in &mut curr_rock {
            val.1 += curr_top;
        }
        loop {
            //print_state(&occupied, &curr_rock);
            lr = input[path_i % input.len()];
            path_i += 1;
            //println!("{}", lr);
            if (lr == -1) && can_left(&curr_rock, &occupied) {
                for mut val in &mut curr_rock {
                    val.0 -= 1;
                }
            } else if (lr == 1) && can_right(&curr_rock, &occupied) {

                for mut val in &mut curr_rock {
                    val.0 += 1;
                }
            }

            if can_down(&curr_rock, &occupied) {
                for mut val in &mut curr_rock {
                    val.1 -= 1;
                }
            } else {
                for val in &curr_rock {
                    if val.1 + 1 > curr_top {
                        curr_top = val.1 + 1;
                    }
                    occupied.insert(*val);
                }
                //print_state(&occupied, &curr_rock);
                break;
            }

        }
    }
    println!("{}", curr_top);
}

fn print_state(occupied: &HashSet<(i32, i32)>, curr_rock: &Vec<(i32, i32)>) {
    let mut print_vec = vec![vec!['.'; 7]; 20];
    for pair in occupied {
        print_vec[(pair.1 + 1) as usize][pair.0 as usize] = '#';
    }
    for piece in curr_rock {
        print_vec[piece.1 as usize + 1][piece.0 as usize] = '#';
    }
    for line in print_vec.iter().rev() {
        println!("{:?}", line);
    }
    println!();
}
