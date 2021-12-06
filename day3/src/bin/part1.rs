fn main() {
    let grid = aoc::Grid::from(
        aoc::parser::lines::<String>()
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect(),
    );

    println!("{}", day3::how_many_tree(&grid, &(3, 1).into()));
}
