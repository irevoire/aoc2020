use day7::*;
use std::collections::HashMap;

fn main() {
    let input = parse();

    let count = count("shiny gold", &mut HashMap::new(), &input);

    // the shiny gold bag don't contain itself
    println!("{}", count - 1);
}

fn count(s: &str, memory: &mut HashMap<String, usize>, map: &Input) -> usize {
    if memory.contains_key(s) {
        memory[s]
    } else {
        let res = map[s]
            .iter()
            .map(|(n, el)| n * count(el, memory, map))
            .sum::<usize>()
            + 1;
        memory.insert(s.to_string(), res);
        res
    }
}
