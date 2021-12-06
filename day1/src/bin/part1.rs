fn main() {
    let iter = aoc::parser::lines::<usize>().collect::<Vec<usize>>();

    for left in &iter {
        for right in &iter {
            if left + right == 2020 {
                println!("{}", left * right);
                return;
            }
        }
    }
}
