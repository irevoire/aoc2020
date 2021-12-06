fn main() {
    let mut lines = aoc::parser::lines::<String>();

    let time: usize = lines.next().unwrap().parse().unwrap();
    let bus: (usize, usize) = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|el| el.parse::<usize>().ok())
        .map(|id| (id, id - (time % id)))
        .reduce(|acc, current| if acc.1 < current.1 { acc } else { current })
        .unwrap();

    println!("res: {}", bus.0 * bus.1);
}
