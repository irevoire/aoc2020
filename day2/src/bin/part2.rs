use day2::*;

fn main() {
    let valid_pass = aoc::parser::lines::<Password>()
        .filter(|pass| pass.is_valid2())
        .count();

    println!("There is {} valid passwords", valid_pass);
}
