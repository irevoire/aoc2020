fn main() {
    let max = aoc::parser::lines_from_args(1)
        .map(day5::str_to_seat_id)
        .max();

    println!("The highest seat ID is: {}", max.unwrap());
}
