use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::vec_deque::VecDeque;

const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn retrace(start: &(usize, usize), end: &(usize, usize), path_map: &HashMap<(usize, usize), (usize, usize)>) -> i32 {
    let mut num_steps = 1;

    let mut next_point = path_map[end];
    while next_point != *start {
        next_point = path_map[&next_point];
        num_steps += 1;
    }
    num_steps
}

pub fn solve() {
    let mut grid: Vec<Vec<&u8>> = include_str!("../../inputs/day_12.txt").split('\n')
        .map(|n| n.as_bytes().iter().collect())
        .collect();

    // is there a better way to find the start?
    let mut start = (0, 0);
    let mut end= (0, 0);
    for (i, vec) in grid.iter().enumerate() {
        for (j, mut val) in vec.iter().enumerate() {
            if **val == 83 {
                start = (i, j);
            }
            if **val == 69 {
                end = (i, j);
            }
        }
    }
    grid[start.0][start.1] = &97;
    grid[end.0][end.1] = &122;

    let result_a = solution_a(&grid, start.clone(), end.clone());
    let result_b = solution_b(&grid, end.clone());

    println!("{}", result_a);
    println!("{}", result_b);
}

fn solution_b(grid: &Vec<Vec<&u8>>, end: (usize, usize)) -> i32 {

    let mut visited: HashSet<(usize, usize)> = HashSet::from([end]);
    let mut path: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut to_explore: VecDeque<(usize, usize)> = VecDeque::from([end]);

    let mut current_square;
    let mut curr_val;
    let mut result= 0;
    while !to_explore.is_empty() {
        current_square = to_explore.pop_front().unwrap();
        curr_val = grid[current_square.0][current_square.1];
        //println!("{:?}, {}", current_square, curr_val + 1);
        for dir in DIRS {
            let next_dir = (current_square.0 as i32 + dir.0, current_square.1 as i32 + dir.1);
            if (next_dir.0 >= 0) && (next_dir.0 < grid.len() as i32)
                && (next_dir.1 >= 0) && (next_dir.1 < grid[0].len() as i32)
                && ((*grid[next_dir.0 as usize][next_dir.1 as usize] >= (*curr_val)) ||
                (*grid[next_dir.0 as usize][next_dir.1 as usize] == (*curr_val - 1)))
                && !visited.contains(&(next_dir.0 as usize, next_dir.1 as usize)) {
                path.insert((next_dir.0 as usize, next_dir.1 as usize), current_square);
                to_explore.push_back((next_dir.0 as usize, next_dir.1 as usize));
                visited.insert((next_dir.0 as usize, next_dir.1 as usize));
                if grid[next_dir.0 as usize][next_dir.1 as usize] == &97 {
                    return retrace(&end, &(next_dir.0 as usize, next_dir.1 as usize), &path);
                }
            }
        }
    }

    //println!("{:?}", visited.contains(&end));
    // something went goofy ;-;
    if to_explore.is_empty() && !visited.contains(&end) {
        panic!("Something went wrong");
    }

    result
}

fn solution_a(grid: &Vec<Vec<&u8>>, start: (usize, usize), end: (usize, usize)) -> i32{

    let mut visited: HashSet<(usize, usize)> = HashSet::from([start]);
    let mut path: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut to_explore: VecDeque<(usize, usize)> = VecDeque::from([start]);

    let mut current_square;
    let mut curr_val;
    let mut result= 0;
    while !to_explore.is_empty() {
        current_square = to_explore.pop_front().unwrap();
        curr_val = grid[current_square.0][current_square.1];
        //println!("{:?}, {}", current_square, curr_val + 1);
        for dir in DIRS {
            let next_dir = (current_square.0 as i32 + dir.0, current_square.1 as i32 + dir.1);
            if (next_dir.0 >= 0) && (next_dir.0 < grid.len() as i32)
                && (next_dir.1 >= 0) && (next_dir.1 < grid[0].len() as i32)
                && *grid[next_dir.0 as usize][next_dir.1 as usize] <= (*curr_val + 1)
                && !visited.contains(&(next_dir.0 as usize, next_dir.1 as usize)) {
                path.insert((next_dir.0 as usize, next_dir.1 as usize), current_square);
                to_explore.push_back((next_dir.0 as usize, next_dir.1 as usize));
                visited.insert((next_dir.0 as usize, next_dir.1 as usize));
                if (next_dir.0 as usize, next_dir.1 as usize) == end {
                    return retrace(&start, &end, &path);

                }

            }
        }
    }

    //println!("{:?}", visited.contains(&end));
    // something went goofy ;-;
    if to_explore.is_empty() && !visited.contains(&end) {
        panic!("Something went wrong");
    }

    result
}