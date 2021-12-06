fn main() {
    let mut input = aoc::parser::lines::<usize>().collect::<Vec<usize>>();

    input.sort();

    let input = input
        .windows(2)
        .map(|el| el[1] - el[0])
        .collect::<Vec<usize>>();

    let (one, three) = (
        input.iter().filter(|&el| *el == 1).count() + 1,
        input.iter().filter(|&el| *el == 3).count() + 1,
    );

    println!("res: {} * {} = {}", one, three, one * three);
}
