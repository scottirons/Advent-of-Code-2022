use std::collections:: { VecDeque, HashSet };

const DIRS: [(i32, i32); 5] = [(0, 1), (0, -1), (1, 0), (-1, 0), (0, 0)];

#[derive(Debug)]
enum Direction {
    PlusRow,
    MinusRow,
    PlusCol,
    MinusCol
}

#[derive(Debug)]
struct Storm {
    dir: Direction,
    offset: i32
}

pub fn solution() {

    let mut q: VecDeque<((i32, i32), i32)> = VecDeque::from([((-1, 0), 0)]);
    let mut visited: HashSet<((i32, i32), i32)> = HashSet::new();

    let mut cols: Vec<Vec<Storm>> = Vec::new();
    let mut rows: Vec<Vec<Storm>> = Vec::new();

    //let mut input = include_str!("input.txt").split('\n').collect::<Vec<&str>>();
    let mut input = include_str!("../../inputs/day_24.txt").split('\n').collect::<Vec<&str>>();
    for row in 0..(input.len() - 2) {
        rows.push(Vec::new());
    }
    for col in 0..(input[0].len() - 2) {
        cols.push(Vec::new());
    }

    for (row, line) in input.iter().enumerate().skip(1).take(input.len() - 2) {
        for (col, wind) in line.chars().enumerate().skip(1).take(line.len() - 2) {
            match wind {
                '>' =>  {
                    rows[row - 1].push(Storm {
                        dir: Direction::PlusRow,
                        offset: col as i32 - 1
                    });
                }
                '<' => {
                    rows[row - 1].push(Storm {
                        dir: Direction::MinusRow,
                        offset: col as i32 - 1
                    });
                }
                '^' => {
                    cols[col - 1].push(Storm {
                        dir: Direction::MinusCol,
                        offset: row as i32 - 1
                    });
                }
                'v' => {
                    cols[col - 1].push(Storm {
                        dir: Direction::PlusCol,
                        offset: row as i32 - 1
                    });
                }
                _ => ()
            }
        }
    }

    let part_a_res = part_a(&mut q, &cols, &rows, &mut visited, &(rows.len() as i32 - 1, cols.len() as i32 - 1));
    println!("First part: {}", part_a_res);
    //visited = HashSet::new();
    q = VecDeque::from([((rows.len() as i32, cols.len() as i32 - 1), part_a_res)]);
    let back_to_start = part_a(&mut q, &cols, &rows, &mut visited, &(0, 0));
    println!("Back to the start: {}", back_to_start);
    //visited = HashSet::new();
    q = VecDeque::from([((-1, 0), back_to_start)]);
    let final_result = part_a(&mut q, &cols, &rows, &mut visited, &(rows.len() as i32 - 1, cols.len() as i32 - 1));
    println!("We back, baby: {}", final_result);

}

fn part_a(mut queue: &mut VecDeque<((i32, i32), i32)>,
          cols: &Vec<Vec<Storm>>, rows: &Vec<Vec<Storm>>,
          mut visited: &mut HashSet<((i32, i32), i32)>, target: &(i32, i32)) -> i32 {

    while queue.len() > 0 {
        let (coord, time) = queue.pop_front().unwrap();
        for dir in DIRS {
            let next = (coord.0 + dir.0, coord.1 + dir.1);
            if !visited.contains(&(next, time + 1)) && valid_location(time + 1, &next, &rows, &cols) {
                if next == *target {
                    return time + 2;
                }
                queue.push_back((next, time + 1));
                visited.insert((next, time + 1));
            }
        }
    }
    panic!("Something went wonky!")
}

fn valid_location(minute: i32, coords: &(i32, i32), rows: &Vec<Vec<Storm>>, cols: &Vec<Vec<Storm>>) -> bool {

    if (coords.0 == -1) && (coords.1 == 0) {
        return true;
    }

    if (coords.0 == rows.len() as i32) && (coords.1 == cols.len() as i32 - 1) {
        return true;
    }

    if (coords.0 < 0) || (coords.1 < 0) || (coords.0 > rows.len() as i32 - 1) || (coords.1 > cols.len() as i32 - 1) {
        return false;
    }

    for storm in &rows[coords.0 as usize] {
        match storm.dir {
            Direction::PlusRow => {
                if (storm.offset + minute).rem_euclid(cols.len() as i32) == coords.1 {
                    return false;
                }
            }
            _ => {
                if (storm.offset - minute).rem_euclid(cols.len() as i32) == coords.1 {
                    return false;
                }
            }
        }
    }

    for storm in &cols[coords.1 as usize] {
        match storm.dir {
            Direction::PlusCol => {
                if (storm.offset + minute).rem_euclid(rows.len() as i32) == coords.0 {
                    return false;
                }
            }
            _ => {
                if (storm.offset - minute).rem_euclid(rows.len() as i32) == coords.0 {
                    return false;
                }
            }
        }
    }

    true
}