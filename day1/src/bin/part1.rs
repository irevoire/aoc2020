fn main() {
    let iter = aoc::parser::lines_from_args_as::<usize>(1).collect::<Vec<usize>>();

    for left in &iter {
        for right in &iter {
            if left + right == 2020 {
                println!("{}", left * right);
                return;
            }
        }
    }
}
