fn main() {
    let input: Vec<usize> = aoc::parser::lines_from_args_as::<usize>(1).collect();

    let part1 = day9::part1(&input);

    let res = (2..input.len())
        .filter_map(|i| input.windows(i).find(|w| is_valid(part1, w)))
        .next()
        .unwrap();

    println!("{}", res.iter().min().unwrap() + res.iter().max().unwrap());
}

fn is_valid(next: usize, buf: &[usize]) -> bool {
    buf.iter().sum::<usize>() == next
}
