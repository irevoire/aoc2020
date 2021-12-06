use std::collections::HashMap;

pub fn compute(n: usize) -> usize {
    let input = aoc::parser::input::<String>()
        .split(',')
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .collect::<Vec<usize>>();

    // init
    let mut memory = input[..input.len() - 1]
        .iter()
        .enumerate()
        .map(|(e, i)| (*i, e + 1))
        .collect::<HashMap<usize, usize>>();

    (input.len()..n).fold(*input.last().unwrap(), |last, i| {
        i - match memory.insert(last, i) {
            Some(v) => v,
            None => i,
        }
    })
}
