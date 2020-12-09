pub fn part1(input: &[usize]) -> usize {
    let preamble = 26;

    *input
        .windows(preamble)
        .find(|window| !is_valid(window[preamble - 1], &window[0..preamble]))
        .unwrap()
        .last()
        .unwrap()
}

fn is_valid(next: usize, buf: &[usize]) -> bool {
    buf.iter()
        .flat_map(|&current| buf.iter().map(move |&el| current + el))
        .any(|x| x == next) // no contains srsly?
}
