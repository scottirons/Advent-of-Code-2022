pub fn solution() {

    let input = include_str!("./input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(a, b)|
            (a.to_string().as_bytes().to_owned()[0] - 64, b.to_string().as_bytes().to_owned()[0] - 87))
        .collect::<Vec<(u8, u8)>>();

    // println!("{:?}", input);
    // Rock: 1
    // Paper: 2
    // Scissors: 3
    // Differences = 0 => val[1] + 3 (3 combos)
    //               1 => val[1] + 6 (2 combos)
    //               2 => val[1]     (1 combo)
    //              -1 => val[1]     (2 combos)
    //              -2 => val[1] + 6 (1 combo)

    // part 1
    let mut total_1: i32 = input.iter().fold(0, |acc, (a, b)|
    match *b as i32 - *a as i32 {
         0 => acc + 3 + *b as i32,
         1 => acc + 6 + *b as i32,
         2 => acc + *b as i32,
        -1 => acc + *b as i32,
        -2 => acc + *b as i32 + 6,
        _ => acc
    });

    // 1 -> lose    *a == 1: 3, *a == 2: 1, *a == 3: 2
    // 2 -> draw    3 + *a
    // 3 -> win     *a == 1: 2, *a == 2: 3, *a == 3: 1 (all plus 6)

    let mut total_2: i32 = input.iter().fold(0, |acc, (a, b)|
        match *b as i32 {
            1 => acc + match *a as i32 {
                1 => 3,
                2 => 1,
                3 => 2,
                _ => 0
            },
            2 => acc + *a as i32 + 3,
            3 => acc + 6 + match *a as i32 {
                1 => 2,
                2 => 3,
                3 => 1,
                _ => 0
            },
            _ => acc
        });

    println!("The total for part 1: {} \n", total_1);
    println!("The total for part 2: {} \n", total_2);
}