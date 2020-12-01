fn main() {
    let iter = aoc::parser::lines_from_args(1)
        .filter_map(|i| i.parse().ok())
        .collect::<Vec<usize>>();

    for left in &iter {
        for right in &iter {
            if left + right == 2020 {
                println!("{}", left * right);
                return;
            }
        }
    }
}
