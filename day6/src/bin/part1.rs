fn main() {
    let res: usize = aoc::parser::input::<String>()
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
