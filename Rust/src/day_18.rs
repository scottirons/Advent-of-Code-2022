use std::collections::{ HashSet, VecDeque };

pub fn solution() {
    let mut input = include_str!("input.txt")
        .split('\n').collect::<Vec<&str>>();

    let mut input = include_str!("../../inputs/day_18.txt")
        .split('\n').collect::<Vec<&str>>();

    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();

    let mut x_bounds = (100, 0);
    let mut y_bounds = (100, 0);
    let mut z_bounds = (100, 0);

    for line in input {
        let mut line_iter = line.split(',');
        let cube = (line_iter.next().unwrap().parse::<i32>().unwrap(),
                    line_iter.next().unwrap().parse::<i32>().unwrap(),
                    line_iter.next().unwrap().parse::<i32>().unwrap());
        cubes.insert(cube);
        x_bounds.0 = x_bounds.0.min(cube.0);
        x_bounds.1 = x_bounds.1.max(cube.0);
        y_bounds.0 = y_bounds.0.min(cube.1);
        y_bounds.1 = y_bounds.1.max(cube.1);
        z_bounds.0 = z_bounds.0.min(cube.2);
        z_bounds.1 = z_bounds.1.max(cube.2);
    }
    println!("{:?}\n{:?}\n{:?}", x_bounds, y_bounds, z_bounds);

    let mut total = 0;
    for cube in &cubes {
        // x dirs
        if !cubes.contains(&(cube.0 + 1, cube.1, cube.2)) {
            total += 1;
        }
        if !cubes.contains(&(cube.0 - 1, cube.1, cube.2)) {
            total += 1;
        }
        if !cubes.contains(&(cube.0, cube.1 + 1, cube.2)) {
            total += 1;
        }
        if !cubes.contains(&(cube.0, cube.1 - 1, cube.2)) {
            total += 1;
        }
        if !cubes.contains(&(cube.0, cube.1, cube.2 + 1)) {
            total += 1;
        }
        if !cubes.contains(&(cube.0, cube.1, cube.2 - 1)) {
            total += 1;
        }
    }
    println!("{}", total);
    println!("{}", search(&cubes, &(x_bounds.0 - 1, y_bounds.0 - 1, z_bounds.0 - 1), &(x_bounds.1 + 1, y_bounds.1 + 1, z_bounds.1 + 1)));
}

fn search(cubes: &HashSet<(i32, i32, i32)>, start: &(i32, i32, i32), end: &(i32, i32, i32)) -> i32 {
    let mut total = 0;
    let mut visited: HashSet<(i32, i32, i32)> = HashSet::from([*start]);
    let mut deque: VecDeque<(i32, i32, i32)> = VecDeque::from([*start]);

    while !deque.is_empty() {
        let (x, y, z) = deque.pop_front().unwrap();

        if !cubes.contains(&(x, y, z)) {
            if cubes.contains(&(x + 1, y, z)) {
                total += 1;
            } else if !visited.contains(&(x + 1, y, z)) && in_boundary(&(x + 1, y, z), start, end) {
                deque.push_back((x + 1, y, z));
                visited.insert((x + 1, y, z));
            }
            if cubes.contains(&(x - 1, y, z)) {
                total += 1;
            } else if !visited.contains(&(x - 1, y, z)) && in_boundary(&(x - 1, y, z), start, end) {
                deque.push_back((x - 1, y, z));
                visited.insert((x - 1, y, z));
            }
            if cubes.contains(&(x, y + 1, z)) {
                total += 1;
            } else if !visited.contains(&(x, y + 1, z)) && in_boundary(&(x, y + 1, z), start, end) {
                deque.push_back((x, y + 1, z));
                visited.insert((x, y + 1, z));
            }
            if cubes.contains(&(x, y - 1, z)) {
                total += 1;
            } else if !visited.contains(&(x, y - 1, z)) && in_boundary(&(x, y - 1, z), start, end) {
                deque.push_back((x, y - 1, z));
                visited.insert((x, y - 1, z));
            }
            if cubes.contains(&(x, y, z + 1)) {
                total += 1;
            } else if !visited.contains(&(x, y, z + 1)) && in_boundary(&(x, y, z + 1), start, end) {
                deque.push_back((x, y, z + 1));
                visited.insert((x, y, z + 1));
            }
            if cubes.contains(&(x, y, z - 1)) {
                total += 1;
            } else if !visited.contains(&(x, y, z - 1)) && in_boundary(&(x, y, z - 1), start, end) {
                deque.push_back((x, y, z - 1));
                visited.insert((x, y, z - 1));
            }
        }
    }
total
}

fn in_boundary(point: &(i32, i32, i32), start: &(i32, i32, i32), end: &(i32, i32, i32)) -> bool {
    match (point.0 < start.0, point.1 < start.1, point.2 < start.2) {
        (true, _, _) => { return false; },
        (_, true, _) => { return false; },
        (_, _, true) => { return false; },
        _ => ()
    }
    match (point.0 > end.0, point.1 > end.1, point.2 > end.2) {
        (true, _, _) => { return false; },
        (_, true, _) => { return false; },
        (_, _, true) => { return false; },
        _ => true
    }
}