use std::ops::Range;

fn main() {
    let (categories, _, nearby_tickets) = day16::parse();
    let all_ranges: Vec<Range<usize>> = categories
        .iter()
        .map(|(_, l, r)| vec![l, r])
        .flatten()
        .cloned()
        .collect();

    let res: usize = nearby_tickets
        .iter()
        .map(|ticket| {
            ticket.iter().fold(0, |acc, current| {
                acc + (!all_ranges.iter().any(|range| range.contains(current)) as usize) * current
            })
        })
        .sum();

    println!("res: {}", res);
}
