

pub fn solution() {

    let mut total: isize = include_str!("../../inputs/day_25.txt").split('\n')
        .map(|n|
            n.chars().into_iter().enumerate()
                .fold(0, |s, (i, j)| match j {
                    '=' => s - (2 * (5_isize.pow(n.len() as u32 - i as u32 - 1))),
                    '-' => s - (5_isize.pow(n.len() as u32 - i as u32 - 1)),
                    '0' => s,
                    '1' => s + (5_isize.pow(n.len() as u32 - i as u32 - 1)),
                    '2' => s + (2 * (5_isize.pow(n.len() as u32 - i as u32 - 1))),
                    _ => panic!("Unrecogniced SNAFU character!")
                }))
        .sum();

    let mut result = String::new();

    while total != 0 {
        let next_val = (total + 2).rem_euclid(5) - 2;
        match next_val {
            2 => result.insert(0, '2'),
            1 => result.insert(0, '1'),
            0 => result.insert(0, '0'),
            -1 => result.insert(0, '-'),
            -2 => result.insert(0, '='),
            _ => panic!("Shouldn't get this")
        }
        total = (total - next_val) / 5;
    }

    println!("{}", result);
}
