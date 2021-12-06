use day14::*;
use std::collections::HashMap;
type Memory = HashMap<usize, usize>;

fn main() {
    let mut memory = Memory::new();
    let mut current_mask = String::new();

    for op in aoc::parser::lines::<Op>().collect::<Vec<_>>() {
        match op {
            Op::Mask(s) => current_mask = s,
            Op::Mem(addr, value) => {
                memory.insert(
                    addr,
                    implode_num(&apply_mask(&current_mask, &explode_num(value))),
                );
            }
        }
    }

    let res: usize = memory.values().sum();

    println!("res: {}", res);
}

pub fn apply_mask(mask: &str, value: &str) -> String {
    mask.chars()
        .zip(
            // we add some padding so the value get to the right of the mask
            std::iter::repeat('0')
                .take(mask.len() - value.len())
                .chain(value.chars()),
        )
        .map(|(mask, value)| match mask {
            'X' => value,
            m => m,
        })
        .collect()
}
