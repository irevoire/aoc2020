use aoc::*;

fn main() {
    let res = aoc::parser::lines::<Movement>()
        .flat_map(|m| match m {
            Movement::Right(n) => std::iter::repeat(Movement::Right(0)).take((n / 90) as usize),
            Movement::Left(n) => std::iter::repeat(Movement::Left(0)).take((n / 90) as usize),
            m => std::iter::repeat(m).take(1),
        })
        .fold(
            (Coord::<isize>::default(), Coord::<isize>::at(10, -1)),
            |(ship, waypoint), movement| match movement {
                Movement::Forward(n) => (ship + (waypoint * n), waypoint),
                Movement::Right(_) => (ship, (-waypoint.y, waypoint.x).into()),
                Movement::Left(_) => (ship, (waypoint.y, -waypoint.x).into()),
                mov => (ship, waypoint + mov),
            },
        )
        .0
        .distance_from_base();

    println!("res: {}", res);
}
