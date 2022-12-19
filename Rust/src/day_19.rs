#![allow(dead_code)]
#![allow(unused_variables, warnings)]
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct OreBot {
    ore_cost: i32
}

#[derive(Debug)]
struct ClayBot {
    ore_cost: i32
}

#[derive(Debug)]
struct ObsidianBot {
    ore_cost: i32,
    clay_cost: i32
}

#[derive(Debug)]
struct GeodeBot {
    ore_cost: i32,
    obsidian_cost: i32
}

#[derive(Debug)]
struct Blueprint {
    ore: OreBot,
    clay: ClayBot,
    obsidian: ObsidianBot,
    geode: GeodeBot
}


pub fn solution() {
    let mut input = include_str!("../../inputs/day_19.txt").split('\n').collect::<Vec<&str>>();
    //let mut input = include_str!("input.txt").split('\n').collect::<Vec<&str>>();
    let mut blueprints: Vec<Blueprint> = Vec::new();

    for blueprint in input {
        let mut split = blueprint.split_whitespace();
        let mut blueprint_vec = Vec::new();
        loop {
            let val = split.next();
            if val != None {
                blueprint_vec.push(val.unwrap());
            } else {
                break;
            }
        }
        let ore = OreBot { ore_cost: blueprint_vec[6].parse::<i32>().unwrap() };
        let clay = ClayBot { ore_cost: blueprint_vec[12].parse::<i32>().unwrap() };
        let obsidian = ObsidianBot {
            ore_cost: blueprint_vec[18].parse::<i32>().unwrap(),
            clay_cost: blueprint_vec[21].parse::<i32>().unwrap() };
        let geode = GeodeBot {
            ore_cost: blueprint_vec[27].parse::<i32>().unwrap(),
            obsidian_cost: blueprint_vec[30].parse::<i32>().unwrap()
        };
        blueprints.push(Blueprint { ore, clay, obsidian, geode });
    }
    let mut result = 0;
    // part A
    for (i, blueprint) in blueprints.iter().enumerate() {
        let val = calculate_ore(blueprint, 24);
        result += val * (i as i32 + 1);
    }
    // part B
    let mut result_b = 1;
    for (i, blueprint) in blueprints.iter().enumerate().take(3) {
        let val = calculate_ore(blueprint, 32);
        println!("{}", val);
        result_b *= val;
    }

    println!("{}", result);
    println!("Part B: {}", result_b);
}

fn calculate_ore(blueprint: &Blueprint, begin: i32) -> i32 {

    let mut cracked = 0;
    let max_ore = blueprint.ore.ore_cost.max(blueprint.geode.ore_cost.max(blueprint.obsidian.ore_cost.max(blueprint.clay.ore_cost)));
    let max_clay = blueprint.obsidian.clay_cost;
    let max_obs = blueprint.geode.obsidian_cost;

    // time, geode, ore, clay, obsidian, obot, cbot, obsbot, gbot
    let start = (begin, 0, 0, 0, 0, 1, 0, 0, 0);
    let mut states: HashSet<(i32, i32, i32, i32, i32, i32, i32, i32, i32)> = HashSet::new(); // do i need this?
    let mut queue: VecDeque<(i32, i32, i32, i32, i32, i32, i32, i32, i32)> = VecDeque::from([start]);
    let mut stack: Vec<(i32, i32, i32, i32, i32, i32, i32, i32, i32)> = Vec::from([start]);

    while stack.len() > 0 {
        //let mut state = queue.pop_front().unwrap();

        let state = *stack.last().unwrap();
        stack.pop();

        // 0: time, 1: geode, 2: ore, 3: clay, 4: obsidian, 5: obot, 6: cbot, 7: obsbot, 8: gbot
        // update amounts of stuff
        let new_ore = state.2 + state.5;
        let new_clay = state.3 + state.6;
        let new_obs = state.4 + state.7;
        let new_geodes = state.1 + state.8;

        cracked = cracked.max(new_geodes);

        if state.0 - 1 == 0 {
            continue;
        }
        // no way we'll get more than the final
        if cracked > ((state.0 * state.8) + (state.0 * (state.0 - 1) / 2) + state.1) {
            continue;
        }

        let mut new_state;

        // build new geode
        if (state.2 >= blueprint.geode.ore_cost) && (state.4 >= blueprint.geode.obsidian_cost) {
            new_state = (state.0 - 1, new_geodes, new_ore - blueprint.geode.ore_cost, new_clay, new_obs - blueprint.geode.obsidian_cost, state.5, state.6, state.7, state.8 + 1);
            if !states.contains(&new_state) {
                //queue.push_back(new_state);
                stack.push(new_state);
                states.insert(new_state);
                continue;
            }
        }
        // build new ore if had enough at start of the round
        if (state.2 >= blueprint.ore.ore_cost) && (state.5 < max_ore) && !(state.5 * state.0 + state.2 >= state.0 * max_ore) {
            new_state = (state.0 - 1, new_geodes, new_ore - blueprint.ore.ore_cost, new_clay, new_obs, state.5 + 1, state.6, state.7, state.8);
            if !states.contains(&new_state) {
                //queue.push_back(new_state);
                stack.push(new_state);
                states.insert(new_state);
            }
        }
        // build new clay if possible
        if (state.2 >= blueprint.clay.ore_cost) && (state.6 < max_clay) && !(state.6 * state.0 + state.3 >= state.0 * max_clay){
            new_state = (state.0 - 1, new_geodes, new_ore - blueprint.clay.ore_cost, new_clay, new_obs, state.5, state.6 + 1, state.7, state.8);
            if !states.contains(&new_state) {
                //queue.push_back(new_state);
                stack.push(new_state);
                states.insert(new_state);
            }
        }
        // build new obsidian if possible
        if (state.2 >= blueprint.obsidian.ore_cost) && (state.3 >= blueprint.obsidian.clay_cost) && (state.7 < max_obs) && !(state.7 * state.0 + state.4 >= state.0 * max_obs) {
            new_state = (state.0 - 1, new_geodes, new_ore - blueprint.obsidian.ore_cost, new_clay - blueprint.obsidian.clay_cost, new_obs, state.5, state.6, state.7 + 1, state.8);
            if !states.contains(&new_state) {
                //queue.push_back(new_state);
                stack.push(new_state);
                states.insert(new_state);
            }
        }
        new_state = (state.0 - 1, new_geodes, new_ore, new_clay, new_obs, state.5, state.6, state.7, state.8);
        if !states.contains(&new_state) {
            //queue.push_back(new_state);
            stack.push(new_state);
            states.insert(new_state);
        }
    }
    cracked
}