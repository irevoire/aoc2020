fn main() {
    let input: Vec<usize> = aoc::parser::lines_from_args_as::<usize>(1).collect();

    println!("{}", day9::part1(&input));
}
