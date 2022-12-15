use std::cmp::Ordering;
use std::collections::HashSet;

const TARGET_A: i64 = 2000000;
const TARGET_TEST: i64 = 10;

#[derive(Debug)]
struct Beacon {
    coords: (i64, i64),
}

#[derive(Debug)]
struct Signal {
    coords: (i64, i64),
    closest: Beacon,
    dist: i64,
}

impl Signal {
    fn create(coords: (i64, i64), beacon: Beacon) -> Signal {
        let dist = (coords.0 - beacon.coords.0).abs() + (coords.1 - beacon.coords.1).abs();
        Signal {
            coords,
            closest: beacon,
            dist
        }
    }
}

impl PartialEq for Signal {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl Eq for Signal {}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> Ordering {
        //use Signal::*;
        self.dist.cmp(&other.dist)
    }
}

struct Sensomatic {
    signals: Vec<Signal>,
    occupied: HashSet<(i64, i64)>,
}

impl Sensomatic {
    fn new() -> Sensomatic {
        Sensomatic {
            signals: Vec::new(),
            occupied: HashSet::new(),
        }
    }
    fn add(&mut self, signal: Signal) {
        self.occupied.insert(signal.coords);
        self.occupied.insert(signal.closest.coords);
        self.signals.push(signal);
    }
}

fn calculate_manhattan_distance(first: &(i64, i64), second: &(i64, i64)) -> i64 {
    (first.0 - second.0).abs() + (first.1 - second.1).abs()
}

fn part_a(signals: &Sensomatic, boundaries: &[i64; 2], y: i64) -> i64 {
    let mut count = 0;
    let mut x = boundaries[0];
    while x <= boundaries[1] {
        for signal in signals.signals.iter() {
            let dist = calculate_manhattan_distance(&signal.coords, &(x, y));
            if (dist <= signal.dist) && (!signals.occupied.contains(&(x, y))){
                count += 1;
                break;
            }
        }
        x += 1;
    }
    count
}

fn part_b(signals: &Sensomatic) -> (i64, i64) {
    let mut result = (0, 0);
    for x in 0..=4000000 {
        for y in 0..=4000000 {
            let mut found_count = 0;
            for signal in signals.signals.iter() {
                let dist = calculate_manhattan_distance(&signal.coords, &(x, y));
                if dist > signal.dist {
                    found_count += 1;
                }
            }
            if found_count == signals.signals.len() {
                return (x, y);
            }
        }
    }
    result
}

pub fn solution() {
    //let input = include_str!("input.txt").split('\n').collect::<Vec<&str>>();
    let input = include_str!("../../inputs/day_15.txt").split('\n').collect::<Vec<&str>>();
    let mut collection = Sensomatic::new();
    let mut boundaries = [i64::MAX, i64::MIN];
    for line in input {
        let mut all_coords: [i64; 4] = [0; 4];
        let mut neg: [bool; 4] = [false; 4];
        let mut count = 0;
        let mut iterator = line.chars().into_iter();
        let mut curr_char;
        for _ in 0..line.len() {
            curr_char = iterator.next().unwrap();
            if curr_char.is_digit(10) {
                all_coords[count] *= 10;
                all_coords[count] += curr_char.to_digit(10).unwrap() as i64;
            } else if curr_char == '-' {
                neg[count] = true;
            } else if (curr_char == ',') || (curr_char == ':') {
                if neg[count] {
                    all_coords[count] *= -1;
                }
                count += 1;
            }
        }
        let new_beacon = Beacon {
            coords: (all_coords[2], all_coords[3])
        };
        let new_sensor = Signal::create((all_coords[0], all_coords[1]), new_beacon);
        boundaries[0] = boundaries[0].min(all_coords[0] - new_sensor.dist - 1);
        boundaries[1] = boundaries[1].max(all_coords[0] + new_sensor.dist + 1);
        collection.add(new_sensor);
    }
    let mut found: Option<(i64, i64)> = None;
    //println!("Part A total: {}", part_a(&collection, &boundaries, TARGET_TEST));
    println!("Part A total: {}", part_a(&collection, &boundaries, TARGET_A));
    println!("Part B tuple: {:?}", part_b(&collection));
}