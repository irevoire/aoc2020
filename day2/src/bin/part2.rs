use day2::*;

fn main() {
    let valid_pass = aoc::parser::lines_from_args_as::<Password>(1)
        .filter(|pass| pass.is_valid2())
        .count();

    println!("There is {} valid passwords", valid_pass);
}
