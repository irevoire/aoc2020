use aoc::*;

fn main() {
    let res = aoc::parser::lines_from_args_as::<Movement>(1)
        .flat_map(|m| match m {
            Movement::Right(n) => std::iter::repeat(Movement::Right(0)).take((n / 90) as usize),
            Movement::Left(n) => std::iter::repeat(Movement::Left(0)).take((n / 90) as usize),
            m => std::iter::repeat(m).take(1),
        })
        .fold(
            Turtle::from(Coord::at(0, 0), Direction::East),
            |turtle, movement| turtle + movement,
        )
        .distance_from_base();

    println!("res: {}", res);
}
