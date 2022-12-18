use std::collections::{ HashSet, HashMap };

fn can_left(shape: &Vec<(isize, isize)>, occupied: &HashSet<(isize, isize)>) -> bool {
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

fn can_right(shape: &Vec<(isize, isize)>, occupied: &HashSet<(isize, isize)>) -> bool {
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

fn can_down(shape: &Vec<(isize, isize)>, occupied: &HashSet<(isize, isize)>) -> bool {
    for val in shape {
        if occupied.contains(&(val.0, val.1 - 1)) {
            return false;
        }
    }
    true
}

pub fn solution() {
    // let mut input: Vec<i8> = include_str!("input.txt").as_bytes().iter()
    //     .map(|n| match n {
    //         &60 => -1,
    //         &62 => 1,
    //         _ => panic!("ahh")
    //     })
    //     .collect();
    let mut input: Vec<i8> = include_str!("../../inputs/day_17.txt").as_bytes().iter()
        .map(|n| match n {
            &60 => -1,
            &62 => 1,
            _ => panic!("ahh")
        })
        .collect();
    //println!("{:?}", input);

    let mut occupied: HashSet<(isize, isize)> = HashSet::from([(0, -1), (1, -1), (2, -1), (3, -1), (4, -1), (5, -1), (6, -1)]);
    let mut shapes: Vec<Vec<(isize, isize)>> = vec![];
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
    let mut curr_top: isize = 0;
    let mut lr = 0;
    let mut rock_mod = 0;
    let mut path_mod = 0;
    let mut states: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    for rock in 0..(237 + 1203) {
        //println!("{}", curr_top);
        let mut curr_rock = shapes[rock % 5].clone();
        for mut val in &mut curr_rock {
            val.1 += curr_top;
        }
        loop {
            //print_state(&occupied, &curr_rock);
            rock_mod = rock % 5;
            path_mod = path_i % input.len();

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
                // if states.contains_key(&(rock_mod, path_mod)) {
                //     let prev = states[&(rock_mod, path_mod)];
                //     println!("Every {} rocks, this diff: {}", rock - prev.0, curr_top as usize - prev.1);
                //     println!("Previous rock: {}, previous value: {}", prev.0, prev.1);
                // } else {
                //     states.insert((rock_mod, path_mod), (rock, curr_top as usize));
                // }
                break;
            }
        }
    }
    println!("{}", curr_top);

    // 1000000000000
    // 1514285714288
    // every 35 => 53 dif, starting at rock 14
    // test input
    // let mut left_to_go: usize = (1000000000000 - 14) % 35;
    // println!("{}", left_to_go);
    // // + 25
    // let result: usize = (1000000000000 / 35) * 53 + 25;
    // println!("{}", result);

    // starting at rock 237, 1720 rocks means a difference of 2738
    // let mut left_to_go: usize = (1000000000000 - 237) % 1720;
    // println!("{}", left_to_go);
    // 237 + 1203
    let result: usize = (1000000000000 / 1720) * 2738 + 2286;
    println!("{}", result);


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
