use std::collections::{ HashSet, HashMap, VecDeque };

#[derive(Debug)]
struct TunnelNetwork<'a> {
    valves: Vec<Valve<'a>>,
    name_map: HashMap<String, usize>,
    _phantom_data: std::marker::PhantomData<&'a ()>,
}

impl<'a> TunnelNetwork<'a> {
    fn new() -> TunnelNetwork<'a> {
        TunnelNetwork {
            valves: Vec::new(),
            name_map: HashMap::new(),
            _phantom_data: std::marker::PhantomData,
        }
    }
    fn add_valve(&mut self, valve: Valve<'a>) {
        self.name_map.insert(valve.name.clone(), self.valves.len());
        self.valves.push(valve);
    }
    fn connect_valves(& mut self, index_a: usize, name_b: String) {
        self.valves[index_a].add_connection(name_b);

    }
}

#[derive(Debug)]
struct Valve<'a> {
    name: String,
    flow: u32,
    connections: Vec<String>,
    paths: Vec<Vec<String>>,
    _phantom_data: std::marker::PhantomData<&'a ()>,
}

impl<'a> Valve<'a> {
    fn create(name: String, flow: u32) -> Valve<'static> {
        Valve {
            name,
            flow,
            connections: Vec::new(),
            paths: Vec::new(),
            _phantom_data: std::marker::PhantomData,
        }
    }
    fn add_connection(& mut self, connection: String) {
        self.connections.push(connection);
    }
}

// #[derive(Debug)]
// struct Path<'a> {
//     start: Valve<'a>,
//     moves: Vec<String>,
//     _phantom_data: std::marker::PhantomData<&'a ()>,
// }

fn part_a(tunnels: &TunnelNetwork) -> u32 {
    let mut result = 0;

    result
}

pub fn solution() {
    let mut input = include_str!("input.txt").split('\n').collect::<Vec<&str>>();
    //let mut input = include_str!("../../inputs/day_16.txt").split('\n').collect::<Vec<&str>>();

    let mut tunnels = TunnelNetwork::new();
    let mut temp_valves: Vec<Vec<String>> = Vec::with_capacity(input.len());
    for line in input {
        let split: Vec<&str> = line.split_whitespace().collect();
        // 1 index = name
        // 4 index has flow
        // 9.. indices have connections
        let name = String::from(split[1]);
        let flow = split[4][5..split[4].len() - 1].parse::<u32>().unwrap();
        let mut temp_vec: Vec<String> = Vec::new();
        for i in 9..split.len() {
            temp_vec.push(String::from(&split[i][0..2]));
        }
        let mut valve = Valve::create(name, flow);
        tunnels.add_valve(valve);
        temp_valves.push(temp_vec);
    }
    // now add connections from temp_valves
    for (i, mut conx) in temp_valves.iter().enumerate() {
        for con in conx {
            tunnels.connect_valves(i, con.clone());
        }
    }

    // find all paths
    for i in 0..tunnels.valves.len() {
        let mut paths: Vec<Vec<String>> = Vec::new();
        let mut visited: HashSet<String> = HashSet::from([tunnels.valves[i].name.clone()]);
        let start = &tunnels.valves[i];
        let mut curr_path: Vec<String> = Vec::new();
        let mut to_visit: VecDeque<Vec<String>> = VecDeque::new();
        for next in &tunnels.valves[i].connections {
            to_visit.push_back(Vec::from([next.clone()]));
        }
        while to_visit.len() > 0 {
            let next_val = to_visit.pop_front().unwrap();
            if !visited.contains(&*next_val[next_val.len() - 1]) {
                curr_path = next_val.clone();
                paths.push(curr_path.clone());
            }
            visited.insert(next_val[next_val.len() - 1].clone());
            let next_i = &tunnels.valves[tunnels.name_map[&next_val[next_val.len() - 1].clone()]];
            for next in &next_i.connections {
                if !visited.contains(next) {
                    curr_path.push(next.clone());
                    to_visit.push_back(curr_path.clone());
                    curr_path.pop();
                }
            }
        }
        for path in &paths {
            tunnels.valves[i].paths.push(path.to_owned());
        }
        //println!("{:?}", paths);
    }
}
