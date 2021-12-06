use std::ops::Range;

pub type Ticket = Vec<usize>;

pub fn parse() -> (
    Vec<(String, Range<usize>, Range<usize>)>,
    Ticket,
    Vec<Ticket>,
) {
    let input = aoc::parser::input::<String>();
    let mut input = input.split("\n\n");

    let categories: Vec<(String, Range<usize>, Range<usize>)> = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut values = line.split(':').flat_map(|s| s.split(" or "));
            let name = values.next().unwrap();
            let mut range = values.map(|s| {
                let mut range = s.trim().split("-").map(|s| s.parse::<usize>().unwrap());
                range.next().unwrap()..range.next().unwrap() + 1
            });
            (
                name.to_string(),
                range.next().unwrap(),
                range.next().unwrap(),
            )
        })
        .collect();

    let ticket: Ticket = input
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let nearby_tickets: Vec<Ticket> = input
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| line.split(",").map(|s| s.parse().unwrap()).collect())
        .collect();

    (categories, ticket, nearby_tickets)
}
