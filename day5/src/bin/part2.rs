fn main() {
    let mut seats = aoc::parser::lines::<String>()
        .map(day5::str_to_seat_id)
        .collect::<Vec<_>>();
    seats.sort();
    let res = seats
        .windows(2)
        .filter(|seats| seats[1] - seats[0] == 2)
        .map(|seats| seats[0] + 1)
        .collect::<Vec<_>>();

    println!("My seat is: {:?}", res);
}
