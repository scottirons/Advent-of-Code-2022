use std::ops::Rem;

#[derive(Debug)]
enum RowCol {
    Row,
    Col
}

#[derive(Debug)]
struct Dimension {
    row_col: RowCol,
    num: usize,
    start: usize,
    end: usize,
}

#[derive(Debug)]
enum Turn {
    L,
    R,
    N
}

#[derive(Debug)]
struct Instruction {
    moves: usize,
    turn: Turn
}

pub fn solution() {
    // total width is b + c - a in test
    // a + c - b in real deal
    //let mut input = include_str!("input.txt").split_once("\n\n").unwrap();
    let mut input = include_str!("../../inputs/day_22.txt").split_once("\n\n").unwrap();

    let mut grid = input.0.split("\n")
        .map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut max_len = 0;
    for row in &grid {
        max_len = max_len.max(row.len());
    }

    for mut row in &mut grid {
        if row.len() < max_len {
            for _ in 0..(max_len - row.len()) {
                row.push(' ');
            }
        }
    }

    let mut cols: Vec<Dimension> = vec![];
    let mut rows: Vec<Dimension> = vec![];

    // populate rows
    for (i, row) in grid.iter().enumerate() {
        let mut curr_i = 0;
        while row[curr_i] == ' ' {
            curr_i += 1;
        }
        let start = curr_i;
        while (curr_i < grid[0].len()) && (row[curr_i] != ' ') {
            curr_i += 1;
        }
        let end = curr_i - 1;
        rows.push(Dimension {
            row_col: RowCol::Row,
            num: i,
            start,
            end
        })
    }

    // populate columns
    for i in 0..grid[0].len() {
        let mut curr_row = 0;
        while grid[curr_row][i] == ' ' {
            curr_row += 1;
        }
        let start = curr_row;
        while (curr_row < grid.len()) && (grid[curr_row][i] != ' ') {
            curr_row += 1;
        }
        let end = curr_row - 1;
        cols.push(Dimension {
            row_col: RowCol::Col,
            num: i,
            start,
            end
        })
    }
    //println!("{:?}\n{:?}", rows, cols);

    // parse directions
    let mut instructions: Vec<Instruction> = vec![];
    let mut curr_count = 0;
    for c in input.1.chars() {
        match c {
            'L' => {
                instructions.push(Instruction {
                    moves: curr_count,
                    turn: Turn::L
                });
                curr_count = 0;
            }
            'R' => {
                instructions.push(Instruction {
                    moves: curr_count,
                    turn: Turn::R
                });
                curr_count = 0;
            }
            'N' => {
                instructions.push(Instruction {
                    moves: curr_count,
                    turn: Turn::N
                });
                curr_count = 0;
            }
            _ => {
                curr_count *= 10;
                curr_count += c.to_digit(10).unwrap() as usize;
            }
        }
    }
    part_a(&instructions, &rows, &cols, &grid);
    part_b(&instructions, &rows, &cols, &grid);
}

fn part_a(instructions: &Vec<Instruction>, rows: &Vec<Dimension>, cols: &Vec<Dimension>, grid: &Vec<Vec<char>>) {

    //println!("{:?}", instructions);
    let mut curr_loc: (usize, usize) = (0, rows[0].start);

    // 0: R, 1: D, 2: L, 3: U
    let mut curr_dir: i8 = 0;

    for instruction in instructions {
        curr_loc = match curr_dir.rem_euclid(2) {
            0 => move_row(grid, rows, &curr_loc, &instruction.moves, &curr_dir),
            1 => move_col(grid, cols, &curr_loc, &instruction.moves, &curr_dir),
            _ => panic!("AHHHHHH")
        };
        match instruction.turn {
            Turn::L => curr_dir = (curr_dir - 1).rem_euclid(4),
            Turn::R => curr_dir = (curr_dir + 1).rem_euclid(4),
            _ => ()
        }
    }
    println!("Part A: {}", (curr_loc.0 + 1) * 1000 + (curr_loc.1 + 1) * 4 + curr_dir as usize);
}

fn move_row(grid: &Vec<Vec<char>>, rows: &Vec<Dimension>, start: &(usize, usize), num: &usize, dir: &i8) -> (usize, usize) {
    let mut col = start.1;
    let boundary = (rows[start.0].start, rows[start.0].end);
    for _ in 0..*num {
        let next: usize = match dir {
            0 => {
                if col + 1 <= boundary.1 {
                    col + 1
                }
                else {
                    boundary.0
                }
            }
            2 => {
                if col >= boundary.0 + 1 {
                    col - 1
                } else {
                    boundary.1
                }
            }
            _ => panic!("Shouldn't be here!")
        };
        if grid[start.0][next] != '#' {
            col = next;
        } else {
            return (start.0, col);
        }
    }
    (start.0, col)
}

fn move_col(grid: &Vec<Vec<char>>, cols: &Vec<Dimension>, start: &(usize, usize), num: &usize, dir: &i8) -> (usize, usize) {
    let mut row = start.0;
    let boundary = (cols[start.1].start, cols[start.1].end);
    for _ in 0..*num {
        let next: usize = match dir {
            1 => {
                if row + 1 <= boundary.1 {
                    row + 1
                }
                else {
                    boundary.0
                }
            }
            3 => {
                if row >= boundary.0 + 1 {
                    row - 1
                } else {
                    boundary.1
                }
            }
            _ => panic!("Shouldn't be here!")
        };
        if grid[next][start.1] != '#' {
            row = next;
        } else {
            return (row, start.1);
        }
    }
    (row, start.1)
}

// okayyyy, let's go
fn part_b(instructions: &Vec<Instruction>, rows: &Vec<Dimension>, cols: &Vec<Dimension>, grid: &Vec<Vec<char>>) {

    let mut curr_loc: (usize, usize) = (0, rows[0].start);
    // 0: R, 1: D, 2: L, 3: U
    let mut curr_dir: i8 = 0;

    for instruction in instructions {
        let mut moves = instruction.moves;
        while moves > 0 {
            moves -= 1;
            match curr_dir {
                0 => {
                    if curr_loc.1 < rows[curr_loc.0].end {
                        if grid[curr_loc.0][curr_loc.1 + 1] != '#' {
                            curr_loc.1 += 1;
                        } else {
                            moves = 0;          // sketchy break statement???
                        }
                    } else {
                        let (next_loc, next_dir) = cube_stuff(grid, &curr_loc, &curr_dir,
                        rows, cols);
                        if grid[next_loc.0][next_loc.1] != '#' {
                            curr_loc = next_loc;
                            curr_dir = next_dir;
                        }
                    }
                }
                2 => {
                    if curr_loc.1 >= rows[curr_loc.0].start + 1 {
                        if grid[curr_loc.0][curr_loc.1 - 1] != '#' {
                            curr_loc.1 -= 1;
                        } else {
                            moves = 0;          // sketchy break statement???
                        }
                    } else {
                        let (next_loc, next_dir) = cube_stuff(grid, &curr_loc, &curr_dir,
                                                              rows, cols);
                        if grid[next_loc.0][next_loc.1] != '#' {
                            curr_loc = next_loc;
                            curr_dir = next_dir;
                        }
                    }
                }
                1 => {
                    if curr_loc.0 < cols[curr_loc.1].end {
                        if grid[curr_loc.0 + 1][curr_loc.1] != '#' {
                            curr_loc.0 += 1;
                        } else {
                            moves = 0;          // sketchy break statement???
                        }
                    } else {
                        let (next_loc, next_dir) = cube_stuff(grid, &curr_loc, &curr_dir,
                                                              rows, cols);
                        if grid[next_loc.0][next_loc.1] != '#' {
                            curr_loc = next_loc;
                            curr_dir = next_dir;
                        }
                    }
                }
                3 => {
                    if curr_loc.0 >= cols[curr_loc.1].start + 1 {
                        if grid[curr_loc.0 - 1][curr_loc.1] != '#' {
                            curr_loc.0 -= 1;
                        } else {
                            moves = 0;          // sketchy break statement???
                        }
                    } else {
                        let (next_loc, next_dir) = cube_stuff(grid, &curr_loc, &curr_dir,
                                                              rows, cols);
                        if grid[next_loc.0][next_loc.1] != '#' {
                            curr_loc = next_loc;
                            curr_dir = next_dir;
                        }
                    }
                }
                _ => panic!()
            }
        }
        // update next direction
        match instruction.turn {
            Turn::L => curr_dir = (curr_dir - 1).rem_euclid(4),
            Turn::R => curr_dir = (curr_dir + 1).rem_euclid(4),
            _ => ()
        }
    }
    println!("Part B: {}", (curr_loc.0 + 1) * 1000 + (curr_loc.1 + 1) * 4 + curr_dir as usize);
}

fn cube_stuff(grid: &Vec<Vec<char>>, start: &(usize, usize), dir: &i8,
              rows: &Vec<Dimension>, cols: &Vec<Dimension>) -> ((usize, usize), i8) {
    // going from back to bottom
    if (start.0 == 0) && (start.1 < 100) {
        return ((start.1 + 100, 0), 0);
    }

    // bottom to back (the reverse)
    if (start.0 > 149) && (start.1 == 0) {
        return ((0, start.0 - 100), 1);
    }

    // back to left
    else if (start.1 == 50) && (start.0 < 50) {
        return ((149 - start.0, 0), 0);
    }

    // left to back (reverse)
    else if (start.1 == 0) && (start.0 < 150) {
        return ((149 - start.0, 50), 0);
    }

    // right to bottom (only other case row is 0)
    else if start.0 == 0 {
        return ((199, start.1 - 100), 3);
    }

    // bottom to right (reverse)
    else if start.0 == 199 {
        return ((0, start.1 + 100), 1);
    }

    // right to front (only case col is 149)
    else if start.1 == 149 {
        return ((149 - start.0, 99), 2);
    }

    // front to right (reverse)
    else if (start.1 == 99) && (start.0 > 99) {
        return ((149 - start.0, 149), 2);
    }

    // right to top (only place this function will be called with row = 49)
    else if start.0 == 49 {
        return ((start.1 - 50, 99), 2);
    }

    // top to right (reverse, only other place it'll be 99)
    else if start.1 == 99 {
        return ((49, start.0 + 50), 3);
    }

    // top to left (only other place col is 50)
    else if start.1 == 50 {
        return ((100, start.0 - 50), 1);
    }

    // left to top (reverse, only case where starting row is 100))
    else if start.0 == 100 {
        return ((start.1 + 50, 50), 0);
    }

    // front to bottom (only place row is 149)
    else if start.0 == 149 {
        return ((start.1 + 100, 49), 2);
    }

    // bottom to front (final option)
    else if start.1 == 49 {
        return ((149, start.0 - 100), 3);
    }

    panic!("We shouldn't have reached this point!")
}
