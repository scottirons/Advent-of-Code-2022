use std::collections::{ HashMap, VecDeque };

enum Operation {
    Plus,
    Minus,
    Times,
    Divided,
}

struct OpMonkey {
    name: String,
    a: String,
    op: Operation,
    b: String,
}

impl OpMonkey {
    fn calculate(&self, a: f64, b: f64) -> f64 {
        return match self.op {
            Operation::Minus => a - b,
            Operation::Plus => a + b,
            Operation::Divided => a / b,
            Operation::Times => a * b,
        }
    }
}

pub fn solution() {
    //let mut input = include_str!("input.txt").split("\n").collect::<Vec<&str>>();
    let mut input = include_str!("../../inputs/day_21.txt").split("\n").collect::<Vec<&str>>();

    let mut waiting: VecDeque<OpMonkey> = VecDeque::new();
    let mut calculated: HashMap<String, f64> = HashMap::new();

    for line in input {
        let mut split = line.split_whitespace().collect::<Vec<&str>>();
        split[0] = &split[0][0..4];
        match split.len() {
            2 => {
                let name = String::from(split[0]);
                let val = split[1].parse::<f64>().unwrap();
                calculated.insert(name, val);
            }
            4 => {
                let name = String::from(split[0]);
                let a = String::from(split[1]);
                let op = match split[2] {
                    "*" => Operation::Times,
                    "/" => Operation::Divided,
                    "+" => Operation::Plus,
                    "-" => Operation::Minus,
                    _ => panic!("Unrecognized sign")
                };
                let b = String::from(split[3]);
                waiting.push_back(OpMonkey{ name, a, op, b });
            }
            _ => panic!("Invalid number of lines in input")
        }
    }
    //println!("{}", part_a(&mut waiting, &mut calculated));
    println!("{}", part_b(&mut waiting, &mut calculated));
}

fn part_a(mut q: &mut VecDeque<OpMonkey>, mut calculated: &mut HashMap<String, f64>) -> f64 {
    while q.len() > 0 {
        if calculated.contains_key(&String::from("root")) {
            return calculated[&String::from("root")];
        }
        let curr = q.pop_front().unwrap();
        if calculated.contains_key(&curr.a) && calculated.contains_key(&curr.b) {
            let new_val = curr.calculate(calculated[&curr.a], calculated[&curr.b]);
            calculated.insert(curr.name, new_val);
        } else {
            q.push_back(curr);
        }
    }
    calculated[&String::from("root")]
}

fn part_b(mut q: &mut VecDeque<OpMonkey>, mut calculated: &mut HashMap<String, f64>) -> f64 {
    let mut prev_len = q.len();
    let mut root = OpMonkey {
        name: "".to_string(),
        a: "".to_string(),
        op: Operation::Plus,
        b: "".to_string(),
    };
    calculated.remove_entry(&String::from("humn"));
    while q.len() > 0 {
        let curr = q.pop_front().unwrap();
        if curr.name == String::from("root") {
            match (calculated.contains_key(&curr.a), calculated.contains_key(&curr.b)) {
                (true, false) => {
                    if prev_len == q.len() {
                        root = curr;
                        break;
                    } prev_len = q.len();
                }
                (false, true) => {
                    if prev_len == q.len() {
                        root = curr;
                        break;
                    } prev_len = q.len();
                }
                _ => ()
            }
        }
        if calculated.contains_key(&curr.a) && calculated.contains_key(&curr.b) {
            let new_val = curr.calculate(calculated[&curr.a], calculated[&curr.b]);
            calculated.insert(curr.name, new_val);
        } else {
            q.push_back(curr);
        }
    }
    let mut target = 0.0;
    match (calculated.contains_key(&root.a), calculated.contains_key(&root.b)) {
        (true, false) => {
            target = calculated[&root.a];
            calculated.insert(root.b, calculated[&root.a]);
        },
        (false, true) => {
            target = calculated[&root.b];
            calculated.insert(root.a, calculated[&root.b]);
        },
        _ => panic!("this shouldn't have happened")
    };
    while q.len() > 0 {
        let curr = q.pop_front().unwrap();
        if calculated.contains_key(&curr.name) {
            let mut new_val= 0.0;
            if calculated.contains_key(&curr.a) {
                new_val = match curr.op {
                    Operation::Minus => calculated[&curr.a] - calculated[&curr.name],
                    Operation::Divided => calculated[&curr.a] / calculated[&curr.name],
                    Operation::Plus => calculated[&curr.name] - calculated[&curr.a],
                    Operation::Times => calculated[&curr.name] / calculated[&curr.a],
                };
                calculated.insert(curr.b, new_val);
            } else if calculated.contains_key(&curr.b) {
                new_val = match curr.op {
                    Operation::Minus => calculated[&curr.name] + calculated[&curr.b],
                    Operation::Divided => calculated[&curr.name] * calculated[&curr.b],
                    Operation::Plus => calculated[&curr.name] - calculated[&curr.b],
                    Operation::Times => calculated[&curr.name] / calculated[&curr.b],
                };
                calculated.insert(curr.a, new_val);
            }
        } else {
            q.push_back(curr);
        }
    }
    calculated[&String::from("humn")]
}