use day7::*;
use std::collections::HashSet;

fn main() {
    let map = parse();

    let mut cpt = 0;
    let mut set = HashSet::new();
    set.insert("shiny gold".to_string());

    for value in map.keys() {
        if contain(value, &set, &map) {
            set.insert(value.to_string());
            cpt += 1;
        }
    }
    // the shiny gold bag don't contain itself
    println!("{}", cpt - 1);
}

fn contain(s: &str, memory: &HashSet<String>, map: &Input) -> bool {
    memory.contains(s)
        || map[s]
            .iter()
            .any(|(_, el)| memory.contains(el) || contain(el, memory, map))
}
