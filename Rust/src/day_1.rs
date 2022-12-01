pub fn solution() {

    let input: Vec<i32> = include_str!("./input.txt")
        .split("\n\n")
        .map(|n| {
                // 83837\n
                n
                    .lines()
                    .map(|i| i.parse::<i32>().unwrap())
                    .sum::<i32>()
                    //.collect::<Vec<i32>>()
        })
        .collect::<Vec<i32>>();

    // part a result
    let max = input.iter().fold(i32::MIN, |a, &b| a.max(b));
    println!("Part A: {:?}", max);

    // part b result
    let mut top_3: (i32, i32, i32) = (0, 0, 0);

    for val in input.iter() {
        if *val >= top_3.2 {
            top_3.0 = top_3.1;
            top_3.1 = top_3.2;
            top_3.2 = *val;
        } else if *val >= top_3.1 {
            top_3.0 = top_3.1;
            top_3.1 = *val;
        } else if *val >= top_3.0 {
            top_3.0 = *val;
        }
    }
    let top_3_sum = top_3.0 + top_3.1 + top_3.2;

    println!("Part B: {:?}", top_3_sum);
}