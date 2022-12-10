use std::ops::Rem;

pub fn solution() {

    //let input = include_str!("input.txt").split("\n").collect::<Vec<&str>>();
    let input = include_str!("../../inputs/day_10.txt").split("\n").collect::<Vec<&str>>();
    // println!("{:?}", input);

    let mut register_val = 1;   // sprite position
    let mut total = 0;
    let mut cycles: i32 = 0;
    let mut crt: Vec<Vec<String>> = Vec::new();
    // i is one behind
    let mut line_vec:  Vec<String> = Vec::new();
    for line in input.iter() {

        let (instr, value) = if let Some((instr, value)) = line.trim().split_once(" ")
        { (instr, value.parse::<i32>().unwrap()) } else { ("noop", 0) };

        if (cycles + 1 - 20).rem_euclid(40) == 0 {
            total += (register_val * (cycles + 1));
        } else if (cycles + 2 - 20).rem_euclid(40) == 0 && instr == "addx" {
            total += (register_val * (cycles + 2));
        }
        let moves = match instr {
            "addx" => 2,
            "noop" => 1,
            _ => panic!("invalid instruction!")
        };
        for _ in 0..moves {
            if (cycles.rem_euclid(40) - register_val).abs() <= 1 {
                line_vec.push(String::from("#"));
            } else {
                line_vec.push(String::from("."));
            }
            if line_vec.len() == 40 {
                crt.push(line_vec.to_vec());
                line_vec.clear();
            }
            cycles += 1;
        }
        register_val += value;
    }
    println!("{}", total);
    for vec in crt {
        for character in vec.iter() {
            print!("{}", character);
        } println!();
    }
}