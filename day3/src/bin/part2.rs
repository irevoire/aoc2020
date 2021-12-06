fn main() {
    let grid = aoc::Grid::from(
        aoc::parser::lines::<String>()
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect(),
    );

    let res: usize = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|slope| day3::how_many_tree(&grid, slope.into()))
        .product();

    println!("{}", res);
}
