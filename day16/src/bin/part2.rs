use std::ops::Range;

fn main() {
    let (categories, ticket, nearby_tickets) = day16::parse();
    let all_ranges: Vec<Range<usize>> = categories
        .iter()
        .map(|(_, l, r)| vec![l, r])
        .flatten()
        .cloned()
        .collect();

    let nearby_tickets: Vec<day16::Ticket> = nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|current| all_ranges.iter().any(|range| range.contains(current)))
        })
        .cloned()
        .collect();

    let mut res: Vec<_> = (0..nearby_tickets[0].len())
        .map(|i| {
            nearby_tickets
                .iter()
                .fold(categories.clone(), |acc, ticket| {
                    acc.iter()
                        .filter(|(_, range1, range2)| {
                            range1.contains(&ticket[i]) || range2.contains(&ticket[i])
                        })
                        .cloned()
                        .collect()
                })
        })
        .enumerate()
        .collect();

    res.sort_by(|left, right| left.1.len().partial_cmp(&right.1.len()).unwrap());

    let mut res: Vec<_> = (0..res.len())
        .scan(res.clone(), |acc, i| {
            let res = acc[i].1[0].clone();
            acc.iter_mut().skip(i + 1).for_each(|line| {
                line.1
                    .remove(line.1.iter().position(|class| class.0 == res.0).unwrap());
            });
            Some((acc[i].0, res))
        })
        .collect();

    // re-sort by index
    res.sort_by(|(left, _), (right, _)| left.partial_cmp(&right).unwrap());

    let res: usize = res
        .iter()
        .filter(|(_i, el)| el.0.starts_with("departure"))
        .map(|(i, _el)| ticket[*i])
        .product();

    println!("res: {}", res);
}
