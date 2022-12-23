use std::collections::{ HashSet, HashMap, VecDeque };

const DIRS: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1)];

pub fn solution() {

    let mut occupied: HashSet<(i32, i32)> = HashSet::new();
    let mut destinations: HashMap<(i32, i32), i32> = HashMap::new();
    let mut curr_order = VecDeque::from([3, 1, 2, 0]);
    let mut nswe_maxes = [i32::MAX, i32::MIN, i32::MAX, i32::MIN];
    let mut points: Vec<(i32, i32)> = Vec::new();
    let mut potential_moves: Vec<(i32, i32)> = Vec::new();

    // parse input
    //let mut input = include_str!("input.txt").split("\n").collect::<Vec<&str>>();
    let mut input = include_str!("../../inputs/day_23.txt").split("\n").collect::<Vec<&str>>();

    for (i, line) in input.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '#' {
                occupied.insert((i as i32, j as i32));
                points.push((i as i32, j as i32));
                potential_moves.push((i as i32, j as i32));
            }
        }
    }
    let mut moves = 1;
    let mut b_done = false;
    let mut extra_steps = 0;
    for step in 0..10 {
        // potential points
        for (i, point) in points.iter().enumerate() {
            potential_moves[i] = check_dirs(&curr_order, &point, &occupied, &mut destinations);
        }

        // go through to move if necessary
        moves = move_elves(&curr_order, &mut points, &potential_moves, &mut occupied, &mut destinations, &mut nswe_maxes);

        if moves == 0 {
            b_done = true;
            extra_steps = 10 - step + 1;
        }

        // update order
        let next = curr_order.pop_front().unwrap();
        curr_order.push_back(next);
    }

    for point in &points {
        nswe_maxes[0] = nswe_maxes[0].min(point.1);
        nswe_maxes[1] = nswe_maxes[1].max(point.1);
        nswe_maxes[2] = nswe_maxes[2].min(point.0);
        nswe_maxes[3] = nswe_maxes[3].max(point.0);
    }

    println!("Part A total: {}", (nswe_maxes[1] - nswe_maxes[0] + 1) * (nswe_maxes[3] + 1 - nswe_maxes[2]) - points.len() as i32);

    while (moves > 0) && (b_done == false) {
        extra_steps += 1;
        for (i, point) in points.iter().enumerate() {
            potential_moves[i] = check_dirs(&curr_order, &point, &occupied, &mut destinations);
        }

        // go through to move if necessary
        moves = move_elves(&curr_order, &mut points, &potential_moves, &mut occupied, &mut destinations, &mut nswe_maxes);

        // update order
        let next = curr_order.pop_front().unwrap();
        curr_order.push_back(next);
    }

    println!("Part B steps: {}", extra_steps + 10);
}

fn check_dirs(curr_order: &VecDeque<i32>, point: &(i32, i32), occupied: &HashSet<(i32, i32)>,
              mut destinations: &mut HashMap<(i32, i32), i32>) -> (i32, i32) {

    let mut result = [false; 8];
    let mut potential: (i32, i32);
    let mut old_val;
    for (i, dir) in DIRS.iter().enumerate() {
        potential = (point.0 + dir.0, point.1 + dir.1);
        if !occupied.contains(&potential) {
            result[i] = true;
        }
    }
    if result == [true, true, true, true, true, true, true, true] {
        return *point;
    }

    // const DIRS: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1)];

    for dir in curr_order {

        match *dir {
            0 => {
                if (result[2] == true) && (result[3] == true) && (result[4] == true) {
                    potential = (point.0, point.1 + 1);
                    if destinations.contains_key(&potential) {
                        old_val = destinations[&potential];
                        destinations.insert(potential, old_val + 1);
                    } else {
                        destinations.insert(potential, 1);
                    }
                    return potential;
                }
            }
            1 => {
                if (result[4] == true) && (result[5] == true) && (result[6] == true) {
                    potential = (point.0 + 1, point.1);
                    if destinations.contains_key(&potential) {
                        old_val = destinations[&potential];
                        destinations.insert(potential, old_val + 1);
                    } else {
                        destinations.insert(potential, 1);
                    }
                    return potential;
                }
            }
            2 => {
                if (result[0] == true) && (result[6] == true) && (result[7] == true) {
                    potential = (point.0, point.1 - 1);
                    if destinations.contains_key(&potential) {
                        old_val = destinations[&potential];
                        destinations.insert(potential, old_val + 1);
                    } else {
                        destinations.insert(potential, 1);
                    }
                    return potential;
                }
            }
            3 => {
                if (result[0] == true) && (result[1] == true) && (result[2] == true) {
                    potential = (point.0 - 1, point.1);
                    if destinations.contains_key(&potential) {
                        old_val = destinations[&potential];
                        destinations.insert(potential, old_val + 1);
                    } else {
                        destinations.insert(potential, 1);
                    }
                    return potential;
                }
            }
            _ => panic!("We shouldn't have gotten here! Only can go 4 possible directions")
        }
    }
    *point
}

fn move_elves(curr_order: &VecDeque<i32>, mut points: &mut Vec<(i32, i32)>, potential_moves: &Vec<(i32, i32)>, occupied: &mut HashSet<(i32, i32)>,
              mut destinations: &mut HashMap<(i32, i32), i32>, mut nsew_maxes: &mut [i32; 4]) -> i32{

    let mut moves= 0;
    for i in 0..points.len() {
        if points[i] == potential_moves[i] {
            continue;
        }
        else if destinations[&potential_moves[i]] == 1 {
            moves += 1;
            occupied.remove(&points[i]);
            occupied.insert(potential_moves[i]);
            points[i] = potential_moves[i].clone();
            destinations.insert(potential_moves[i], 0);
        } else {
            destinations.insert(potential_moves[i], 0);
        }
    }
    moves
}