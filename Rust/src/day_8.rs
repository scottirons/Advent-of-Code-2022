
pub fn solution() {
    let input: Vec<Vec<u8>> = include_str!("../../inputs/day_8.txt")
        .split("\n")
        .map(|n| n.as_bytes().iter().map(|&b| b - 48).collect())
        .collect();

    let mut visible: Vec<Vec<bool>> = vec![vec![false; input.len()]; input[0].len()];
    // to right
    for row in 0..input.len() {
        let mut curr_height = input[row][0];
        visible[row][0] = true;
        for col in 1..input[0].len() {
            if input[row][col] > curr_height {
                visible[row][col] = true;
                curr_height = input[row][col];
            }
        }
    }
    // to left
    for row in (0..input.len()).rev() {
        let mut curr_height = input[row][input.len() - 1];
        visible[row][input.len() - 1] = true;
        for col in (0..(input.len() - 1)).rev() {
            if input[row][col] > curr_height {
                visible[row][col] = true;
                curr_height = input[row][col];
            }
        }
    }
    // going up
    for col in 0..input[0].len() {
        let mut curr_height = input[input.len() - 1][col];
        visible[input.len() - 1][col] = true;
        for row in (0..(input.len() - 1)).rev() {
            if input[row][col] > curr_height {
                visible[row][col] = true;
                curr_height = input[row][col];
            }
        }
    }
    // going down
    for col in 0..input[0].len() {
        let mut curr_height = input[0][col];
        visible[0][col] = true;
        for row in 1..input.len() {
            if input[row][col] > curr_height {
                visible[row][col] = true;
                curr_height = input[row][col];
            }
        }
    }
    let mut total_a: i32 = 0;
    for row in 0..visible.len() {
        for col in 0..visible[0].len() {
            if visible[row][col] == true {total_a += 1;}
        }
    }
    println!("{}", total_a);

    let mut view_score = 0;
    for row in 1..(input.len() - 1) {
        for col in 1..(input[0].len() - 1) {
            let mut curr_score = 1;

            // up
            let mut i: i32 = row as i32 - 1;
            let mut up = 0;
            let mut curr_val = input[row][col];
            while i >= 0 {
                if input[i as usize][col] >= curr_val {
                    up += 1;
                    break;
                } else {up += 1;}
                i -= 1;
            }
            curr_score = curr_score * up;

            // down
            let mut i: i32 = row as i32 + 1;
            let mut down = 0;
            let mut curr_val = input[row][col];
            while i < input.len() as i32 {
                if input[i as usize][col] >= curr_val {
                    down += 1;
                    break;
                } else {down += 1;}
                i += 1;
            }
            curr_score = curr_score * down;

            // left
            let mut i: i32 = col as i32 - 1;
            let mut left = 0;
            let mut curr_val = input[row][col];
            while i >= 0 {
                if input[row][i as usize] >= curr_val {
                    left += 1;
                    break;
                } else {left += 1;}
                i -= 1;
            } curr_score = curr_score * left;

            // right
            let mut i: i32 = col as i32 + 1;
            let mut right = 0;
            let mut curr_val = input[row][col];
            while i < input.len() as i32 {
                if input[row][i as usize] >= curr_val {
                    right += 1;
                    break;
                } else {right += 1;}
                i += 1;
            } curr_score = curr_score * right;
            view_score = view_score.max(curr_score);
        }
    }
    println!("{}", view_score);
}