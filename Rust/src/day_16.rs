use std::collections::{ HashSet, HashMap, VecDeque };

fn shortest_paths(valves: &Vec<(String, usize, Vec<String>)>, important: &HashMap<usize, usize>) -> HashMap<usize, HashMap<usize, usize>> {
    let mut paths = HashMap::new();
    let mut fancy_array = [[10000_usize; 60]; 60];     // 55 lines in input CHANGE THIS
    for i in 0..valves.len() {
        for connection in &valves[i].2 {
            let mut connection_i = 0;
            for (j, valve) in valves.iter().enumerate() {
                if valve.0 == *connection {
                    connection_i = j;
                }
            }
            fancy_array[i][connection_i] = 1;
            fancy_array[connection_i][i] = 1;
        }
    }
    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                fancy_array[i][j] = fancy_array[i][j].min(fancy_array[i][k] + fancy_array[k][j]);
            }
        }
    }
    for i in 0..valves.len() {
        let line = fancy_array[i];
        let mut path_map = HashMap::new();
        for (j, dist) in line.iter().enumerate() {
            if (j != i) && important.contains_key(&j) {
                path_map.insert(j, *dist);
            }
        }
        paths.insert(i, path_map);
    }
    //println!("{:?}", paths);
    paths
}

pub fn solution() {
    //let mut input = include_str!("input.txt").split('\n').collect::<Vec<&str>>();
    let mut input = include_str!("../../inputs/day_16.txt").split('\n').collect::<Vec<&str>>();
    //let mut input = include_str!("test.txt").split('\n').collect::<Vec<&str>>();

    let mut valves = Vec::with_capacity(input.len());
    let mut positive_valves = HashMap::new();
    for line in input {
        let split: Vec<&str> = line.split_whitespace().collect();
        let name = String::from(split[1]);
        let flow = split[4][5..split[4].len() - 1].parse::<usize>().unwrap();
        let mut temp_vec: Vec<String> = Vec::new();
        for i in 9..split.len() {
            temp_vec.push(String::from(&split[i][0..2]));
        }
        valves.push((name, flow, temp_vec) );
    }
    let mut start = 0;
    valves.sort_by(|a, b|b.1.cmp(&a.1));
    for (i, valve) in valves.iter().enumerate() {
        if valve.0 == "AA" {
            start = i;
        }
    }

    for (i, valve) in valves.iter().enumerate() {
        if valve.1 > 0 {
            positive_valves.insert(i, valve.1);
        }
    }
    //println!("{:?}", valves);
    let paths = shortest_paths(&valves, &positive_valves);
    let result_a = part_a(&paths, &positive_valves, &start);
    println!("{}", result_a);

}

fn backtrack(paths: &HashMap<usize, HashMap<usize, usize>>, positive: &HashMap<usize, usize>, mut result: usize,
             mut curr_time: i32, mut curr_path: Vec<usize>, curr: usize, mut states: &mut HashMap<Vec<usize>, usize>) -> (usize, Vec<usize>) {

    if (curr_path.len() > positive.len()) || (curr_time <= 0) {
        return (result, curr_path);
    }
    let mut starting_result = result.clone();
    let mut clone_path = curr_path.clone();

    for connection in &paths[&curr] {
        if (curr_path[*connection.0] != 1) && (curr_time - *connection.1 as i32 >= 0) {
            if curr_time - *connection.1 as i32 > 0 {
                let mut time_clone = curr_time.clone();
                time_clone -= *connection.1 as i32;
                time_clone -= 1;
                curr_path[*connection.0] = 1;
                starting_result += time_clone as usize * positive[&*connection.0];

                if ((curr_path.iter().filter(|&n| *n == 1).count()) >= positive.len()) && (time_clone >= 0) {
                    return (starting_result, curr_path);
                }
                let backtrack_result = (backtrack(paths, positive, starting_result, time_clone, curr_path.clone(), *connection.0, states));
                states.insert(backtrack_result.1.clone(), backtrack_result.0.max(if states.contains_key(&*backtrack_result.1) { states[&backtrack_result.1] } else { 0 }));
                states.insert(clone_path.clone(), result.max(if states.contains_key(&*clone_path) { states[&clone_path] } else { 0 }));

                if backtrack_result.0 > result {
                    result = backtrack_result.0;
                    clone_path = backtrack_result.1;
                }
                starting_result -= time_clone as usize * positive[connection.0];
                curr_path[*connection.0] = 0;
            }
        }
    }
    (result.max(starting_result), clone_path)
}

fn part_a(paths: &HashMap<usize, HashMap<usize, usize>>, positive: &HashMap<usize, usize>, start: &usize) -> usize {
    let mut result = 0;
    let mut curr_time = 26;
    let mut curr: usize = *start;
    let mut curr_path = Vec::new();
    for _ in 0..positive.len() {
        curr_path.push(0);
    }
    let mut states = HashMap::new();
    let score = backtrack(paths, positive, result, curr_time, curr_path, curr, &mut states);
    let mut total_b = 0;
    println!("{}", states.len());
    for (i, state) in states.iter().enumerate() {
        for (j, j_state) in states.iter().enumerate() {
            if i != j {
                let mut same_count = 0;
                for n in 0..state.0.len() {
                    if (state.0[n] == 1) && (j_state.0[n] == 1) {
                        same_count += 1;
                    }
                }
                if same_count == 0 {
                    total_b = total_b.max(state.1 + j_state.1);
                }

            }
        }
    }
    println!("{}", total_b);

    score.0
}