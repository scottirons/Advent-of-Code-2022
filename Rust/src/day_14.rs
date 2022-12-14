fn is_obstacle(x: &char) -> bool {
    if (*x == '#') || (*x == 'o') {
        return true;
    }
    false
}

fn move_sand(grid: &mut Vec<Vec<char>>, y: &mut usize, x: &mut usize) -> bool {
    loop {
        if (*y + 1 >= grid.len()) || (*x <= 0) || (*x > grid[0].len()) || is_obstacle(&grid[*y][*x]) {
            return false;
        }
        // stay where it is
        if is_obstacle(&grid[*y + 1][*x]) && is_obstacle(&grid[*y + 1][*x - 1]) &&
            is_obstacle(&grid[*y + 1][*x + 1]) {
            grid[*y][*x] = 'o';
            return true;
        }
        match (is_obstacle(&grid[*y + 1][*x - 1]), is_obstacle(&grid[*y + 1][*x]),
            is_obstacle(&grid[*y + 1][*x + 1])) {
            (true, true, true) => {
                grid[*y][*x] = 'o';
                return true;
            } (false, true, _) => {
                *x -= 1;
            } (true, true, false) => {
                *x += 1;
            } _ => ()
        } *y += 1;
    }
}

pub fn solution() {
    // parse it. Find min and max x as well as max y
    //let input = include_str!("input.txt").split('\n').collect::<Vec<&str>>();
    let input = include_str!("../../inputs/day_14.txt").split('\n').collect::<Vec<&str>>();
    let mut blocks: Vec<Vec<Vec<u32>>> = Vec::new();
    let mut min_x = 500;
    let mut max_x = 500;
    let mut max_y = 0;
    for line in input {
        let new_vec: Vec<Vec<u32>> = line
            .split(" -> ")
            .map(|p| p.split(",")
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<u32>>())
            .collect();
        for coord in &new_vec {
            min_x = min_x.min(coord[0]);
            max_x = max_x.max(coord[0]);
            max_y = max_y.max(coord[1]);
        }
        blocks.push(new_vec);
    }
    // create grid to be populated -- offset will be "Value - min_x"
    let mut grid = vec![vec!['.'; (max_x - min_x + 1) as usize]; (max_y + 1) as usize];

    for block in &blocks {
        let mut i = 0;
        let mut start = &block[i];
        let mut next;
        i += 1;
        while i < block.len() {
            next = &block[i];
            let mut x_start = (start[0] - min_x).min(next[0] - min_x);
            let mut x_end = (start[0] - min_x).max(next[0] - min_x);
            let mut y_start = start[1].min(next[1]);
            let mut y_end = start[1].max(next[1]);
            for x in x_start..(x_end + 1) {
                for y in y_start..(y_end + 1) {
                    grid[y as usize][x as usize] = '#';
                }
            }
            i += 1;
            start = next;
        }
    }
    let mut thicc_grid: Vec<Vec<char>> =  Vec::new();
    for line in &grid {
        let mut new_vec = Vec::with_capacity(line.len() + 2 * (max_y + 2) as usize);
        for _ in 0..(max_y + 2) {
            new_vec.push('.');
        }
        new_vec.append(&mut line.clone());
        for _ in 0..(max_y + 2) {
            new_vec.push('.');
        }
        thicc_grid.push(new_vec);
    }
    thicc_grid.push(vec!['.'; thicc_grid[0].len()]);
    thicc_grid.push(vec!['#'; thicc_grid[0].len()]);

    let mut y: usize = 0;
    let mut x = 500 - min_x as usize;
    let mut placed = 0;
    while move_sand(&mut grid, &mut y, &mut x) {
        placed += 1;
        y = 0;
        x = 500 - min_x as usize;
    }

    let mut placed_2_electric_boogaloo = 0;
    let mut y: usize = 0;
    let mut x = 500 - (min_x as usize) + (max_y + 2) as usize;
    while move_sand(&mut thicc_grid, &mut y, &mut x) {
        placed_2_electric_boogaloo += 1;
        y = 0;
        x = 500 - (min_x as usize) + (max_y + 2) as usize;
    }
    println!("Part A placed: {}\nPart B: {}", placed, placed_2_electric_boogaloo);
}