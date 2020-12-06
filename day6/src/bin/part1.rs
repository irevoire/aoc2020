fn main() {
    let res: usize = aoc::parser::read_file_from_args(1)
        .split("\n\n")
        .map(|form| {
            form.replace("\n", "")
                .replace(" ", "")
                .chars()
                .collect::<std::collections::HashSet<_>>()
                .len()
        })
        .sum();

    println!("res: {}", res);
}
