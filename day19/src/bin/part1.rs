use day19::*;

fn main() {
    let file = aoc::parser::read_file_from_args(1);
    let mut file = file.split("\n\n");

    let rules = file.next().unwrap().parse::<Rules>().unwrap();

    let res = file
        .next()
        .unwrap()
        .lines()
        .filter(|line| rules.check(line))
        .count();

    println!("res: {}", res);
}
