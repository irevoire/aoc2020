use std::collections::HashSet;

fn main() {
    let res: usize = aoc::parser::read_file_from_args(1)
        .split("\n\n")
        .map(|form| {
            form.replace(" ", "")
                .split("\n")
                .filter(|p| p != &"")
                .map(|passager| passager.chars().collect::<HashSet<_>>())
                .reduce(|acc, passenger| acc.intersection(&passenger).copied().collect())
                .unwrap()
                .len()
        })
        .sum();

    println!("res: {}", res);
}
