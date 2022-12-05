

pub fn solution() {
    let input = include_str!("../../inputs/day_4.txt")
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(a, b)| (a.split_once('-').unwrap(),
             b.split_once('-').unwrap()))
        .map(|((c, d), (e, f))| (c.parse::<i32>().unwrap(), d.parse::<i32>().unwrap(),
                                 e.parse::<i32>().unwrap(), f.parse::<i32>().unwrap())).collect::<Vec<(i32, i32, i32, i32)>>();

    //println!("{:?}", input);

    let mut part_a_total = 0;
    let mut part_b_total = 0;

    // if ((b >= d) and (a <= c)) or ((d >= b) and (c <= a)):
    //     total_a += 1
    // if (d >= b >= c) or (c <= a <= d) or (a <= c <= b) or (a <= d <= b):
    //     total_b += 1

    // Part A
    for (a, b, c, d) in input.iter() {
        if ((*a <= *c) & (*b >= *d)) | ((*d >= *b) & (*c <= *a)) {
            part_a_total += 1;
        }
        if ((*d >= *b) & (*b >= *c)) | ((*c <= *a) & (*a <= *d)) | ((*a <= *c) & (*c <= *b)) | ((*a <= *d) & (*d <= *b)) {
            part_b_total += 1;
        }
    }
    println!("Part A total: {}\nPart B total: {}", part_a_total, part_b_total)

}