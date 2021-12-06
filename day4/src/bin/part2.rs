fn main() {
    let nb_valid_passport = aoc::parser::input::<String>()
        .split("\n\n")
        .map(|lines| lines.parse::<day4::Passport>().unwrap())
        .filter(|passport| passport.is_valid2())
        .count();

    println!("There is {} valid passport", nb_valid_passport);
}
