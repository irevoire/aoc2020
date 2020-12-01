fn main() {
    let iter = aoc::parser::lines_from_args(1)
        .filter_map(|i| i.parse().ok())
        .collect::<Vec<usize>>();

    for one in &iter {
        for two in &iter {
            for three in &iter {
                if one + two + three == 2020 {
                    println!("{}", one * two * three);
                    return;
                }
            }
        }
    }
}
