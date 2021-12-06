use day18::*;

fn main() {
    let res: usize = aoc::parser::lines::<String>()
        .map(|s| execute(&s.replace(" ", "")))
        .sum();

    println!("res: {:?}", res);
}
