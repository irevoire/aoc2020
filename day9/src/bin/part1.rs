fn main() {
    let input: Vec<usize> = aoc::parser::lines::<usize>().collect();

    println!("{}", day9::part1(&input));
}
