use day18::*;

fn main() {
    let res: usize = aoc::parser::lines_from_args(1)
        .map(|s| execute(&s.replace(" ", "")))
        .sum();

    println!("res: {:?}", res);
}
