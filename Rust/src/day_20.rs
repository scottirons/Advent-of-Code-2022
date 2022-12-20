use std::cmp::Ordering;
use std::collections::HashMap;

pub fn solution() {

    // let mut nums = include_str!("input.txt").split('\n')
    //     .map(|n| n.parse::<isize>().unwrap())
    //     .collect::<Vec<isize>>();

    let mut nums = include_str!("../../inputs/day_20.txt").split('\n')
        .map(|n| n.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();

    let mut part_2_nums = nums.iter().map(|n| n * 811589153).collect::<Vec<isize>>();
    let mut order_index = HashMap::with_capacity(part_2_nums.len());
    let mut index_order = HashMap::with_capacity(part_2_nums.len());
    for i in 0..part_2_nums.len() {
        order_index.insert(i, i);
        index_order.insert(i, i);
    }
    // find max/min
    let mut max = isize::MIN;
    let mut min = isize::MAX;
    for num in &nums {
        max = max.max(*num);
        min = min.min(*num);
    }
    let dif = max - min + 1;

    // find max/min
    let mut max_b = isize::MIN;
    let mut min_b = isize::MAX;
    for num in &part_2_nums {
        max_b = max_b.max(*num);
        min_b = min_b.min(*num);
    }
    let dif_b = max_b - min_b + 1;

    //do_stuff(min, max, dif, &mut nums);
    //do_fancier_stuff(min, max, dif, &mut nums, &mut order_index, &mut index_order);
    do_fancier_stuff(min_b, max_b, dif_b, &mut part_2_nums, &mut order_index, &mut index_order);
}

fn do_stuff(min: isize, max: isize, dif: isize, mut nums: &mut Vec<isize>) {
    let mut moved = 0;
    let mut i = 0;
    let mut moves;
    while moved < nums.len() {
        if nums[i] <= max {
            //println!("Moving: {}\nCurrent state: {:?}", nums[i], nums);
            moved += 1;
            moves = nums[i].rem_euclid(nums.len() as isize - 1);

            nums[i] += dif;
            if i + moves as usize > nums.len() - 1 {
                moves = (nums.len() - 1 - moves as usize) as isize;
                swap_left(i, moves, &mut nums);
            } else {
                swap_right(i, moves, &mut nums);
            }
        } else {
            i = (i + 1).rem_euclid(nums.len());
        }

    }
    let mut start = 0;
    for i in 0..nums.len() {
        nums[i] -= dif;
        if nums[i] == 0 {
            start = i;
        }
    }
    let a = nums[(start + 1000).rem_euclid(nums.len())];
    let b = nums[(start + 2000).rem_euclid(nums.len())];
    let c = nums[(start + 3000).rem_euclid(nums.len())];
    println!("1000th: {}\n2000th: {}\n3000th: {}\nTotal: {}", a, b, c, a + b + c);
}

fn do_fancier_stuff(min: isize, max: isize, dif: isize, mut nums: &mut Vec<isize>, mut order_index: &mut HashMap<usize, usize>, mut index_order: &mut HashMap<usize, usize>) {

    let mut moves;
    for _ in 0..10 {
        for order_i in 0..nums.len() {
            let i = order_index[&order_i];
            //println!("Moving: {}\nCurrent state: {:?}", nums[i], nums);
            moves = nums[i].rem_euclid(nums.len() as isize - 1);
            if i + moves as usize > nums.len() - 1 {
                moves = (nums.len() - 1 - moves as usize) as isize;
                swap_left(i, moves, &mut nums);
                for j in ((i - moves as usize + 1)..i + 1).rev() {
                    index_order.insert(j, index_order[&(j - 1)]);
                    order_index.insert(index_order[&j], j);
                }
                index_order.insert(i - moves as usize, order_i);
                order_index.insert(order_i, i - moves as usize);

            } else {
                swap_right(i, moves, &mut nums);
                for j in i..(i + moves as usize) {
                    index_order.insert(j, index_order[&(j + 1)]);
                    order_index.insert(index_order[&j], j);
                }
                index_order.insert(i + moves as usize, order_i);
                order_index.insert(order_i, i + moves as usize);
            }
            // println!("Move number: index{:?}", order_index);
            // println!("Index: move_number{:?}", index_order);
            // println!("curr_array: {:?}\n", nums);
        }
    }
    let mut start = 0;
    for i in 0..nums.len() {
        if nums[i] == 0 {
            start = i;
        }
    }
    let a = nums[(start + 1000).rem_euclid(nums.len())];
    let b = nums[(start + 2000).rem_euclid(nums.len())];
    let c = nums[(start + 3000).rem_euclid(nums.len())];
    println!("1000th: {}\n2000th: {}\n3000th: {}\nTotal: {}", a, b, c, a + b + c);
}

fn swap_left(mut index: usize, jumps: isize, mut array: &mut Vec<isize>) {
    let end = array.len() - 1;

    for _ in 0..jumps {
        if index == 0 {
            let temp = array[index];
            array[0] = array[end];
            array[end] = temp;
            index = end;
        } else {
            let temp = array[index];
            array[index] = array[index - 1];
            array[index - 1] = temp;
            index -= 1;
        }
    }
}

fn swap_right(mut index: usize, jumps: isize, mut array: &mut Vec<isize>) {
    let end = array.len() - 1;
    for _ in 0..jumps {
        if index == end {
            let temp = array[index];
            array[end] = array[0];
            array[0] = temp;
            index = 0;
        } else {
            let temp = array[index];
            array[index] = array[index + 1];
            array[index + 1] = temp;
            index += 1;
        }
    }
}

// [1, 2, -3, 3, -2, 0, 4]