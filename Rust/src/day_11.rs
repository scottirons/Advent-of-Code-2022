use std::collections::VecDeque;

const SUPERMODULO: usize = 2 * 7 * 13 * 5 * 3 * 19 * 11 * 17;

enum Operator {
    Plus,
    Times,
}

struct Monkey {
    vals: VecDeque<usize>,
    operator: Operator,
    operand: usize,
    modulo: usize,
    res_a: usize,
    res_b: usize,
    num_moves: usize,
}

impl Monkey {
    fn new(operator: Operator, operand: usize, modulo: usize, res_a: usize, res_b: usize) -> Monkey {
        Monkey {
            vals: VecDeque::new(),
            operator,
            operand,
            modulo,
            res_a,
            res_b,
            num_moves: 0,
        }
    }

    fn add_item(&mut self, item: usize) {
        self.vals.push_back(item);
    }

    fn has_items(&self) -> bool {
        !self.vals.is_empty()
    }

    fn process_a(&mut self) -> (usize, usize) {
        self.num_moves += 1;
        let mut val = self.vals.pop_front().unwrap();
        let mut real_operand = match self.operand {
            0 => val,
            _ => self.operand,
        };
        match self.operator {
            Operator::Plus => val = val + real_operand,
            Operator::Times => val = val * real_operand
        }
        val = val / 3;

        (val,
        match val.rem_euclid(self.modulo) {
            0 => self.res_a as usize,
            _ => self.res_b as usize
        })

    }

    fn process_b(&mut self) -> (usize, usize) {
        self.num_moves += 1;
        let mut val = self.vals.pop_front().unwrap();
        let mut real_operand = match self.operand {
            0 => val,
            _ => self.operand,
        };
        match self.operator {
            Operator::Plus => val = val + real_operand,
            Operator::Times => val = val * real_operand
        }
        val = val.rem_euclid(SUPERMODULO);

        (val.to_owned(),
         match val.rem_euclid(self.modulo) {
             0 => self.res_a as usize,
             _ => self.res_b as usize
         })
    }
}


pub fn solution(part: u8) {

    let mut monkeys: Vec<Monkey> = Vec::new();

    let mut monkey_1 = Monkey::new(Operator::Times, 17, 2, 2, 6);
    monkeys.push(monkey_1);

    let mut monkey_2 = Monkey::new(Operator::Times, 0, 7, 0, 2);
    monkeys.push(monkey_2);

    let mut monkey_3 = Monkey::new(Operator::Plus, 7, 13, 7, 6);
    monkeys.push(monkey_3);

    let mut monkey_4 = Monkey::new(Operator::Plus, 4, 5, 4, 5);
    monkeys.push(monkey_4);

    let mut monkey_5 = Monkey::new(Operator::Plus, 5, 3, 1, 5);
    monkeys.push(monkey_5);

    let mut monkey_6 = Monkey::new(Operator::Plus, 6, 19, 1, 0);
    monkeys.push(monkey_6);

    let mut monkey_7 = Monkey::new(Operator::Times, 13, 11, 3, 7);
    monkeys.push(monkey_7);

    let mut monkey_8 = Monkey::new(Operator::Plus, 2, 17, 4, 3);
    monkeys.push(monkey_8);

    let vals: Vec<Vec<usize>> = vec![vec![85, 79, 63, 72], vec![53, 94, 65, 81, 93, 73, 57, 92],
    vec![62, 63], vec![57, 92, 56], vec![67], vec![85, 56, 66, 72, 57, 99], vec![86, 65, 98, 97, 69],
    vec![87, 68, 92, 66, 91, 50, 68]];

    for (i, start_nums) in vals.iter().enumerate() {
        for num in start_nums.iter() {
            monkeys[i].add_item(*num);
        }
    }
    let num_moves: usize = match part {
        1 => 20,
        2 => 10000,
        _ => panic!()
    };

    for _ in 0..num_moves {
        for i in 0..monkeys.len() {
            while monkeys[i].has_items() {
                let (val, next_monkey) = match part {
                    1 => monkeys[i].process_a(),
                    2 => monkeys[i].process_b(),
                    _ => panic!("Gotta be part 1 or 2, yo!")
                };
                monkeys[next_monkey].add_item(val);
            }
        }
    }
    let (mut a, mut b): (usize, usize) = (0, 0);
    for monkey in &monkeys {
        if monkey.num_moves > b {
            a = b;
            b = monkey.num_moves;
        } else if monkey.num_moves > a {
            a = monkey.num_moves;
        }
    }
    println!("{}", a * b);
}