fn main() {
    let mut input = aoc::parser::lines_from_args_as::<usize>(1).collect::<Vec<usize>>();

    input.push(0);
    input.sort();
    input.push(input.last().unwrap() + 3);

    let input = &mut input.windows(2).map(|el| el[1] - el[0]);

    let res: usize = std::iter::repeat_with(|| {
        input
            .skip_while(|el| *el == 3)
            .take_while(|el| *el == 1)
            .count()
    })
    .take_while(|el| *el != 0)
    .map(|el| match el {
        0 | 1 => 1,
        2 => 2,
        3 => 4,
        4 => 7,
        _ => unimplemented!(),
    })
    .product();

    println!("res: {:?}", res);
}
