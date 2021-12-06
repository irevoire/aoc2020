fn main() {
    let max = aoc::parser::lines::<String>()
        .map(day5::str_to_seat_id)
        .max();

    println!("The highest seat ID is: {}", max.unwrap());
}
