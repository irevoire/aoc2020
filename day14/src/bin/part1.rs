use day14::*;
use std::collections::HashMap;
type Memory = HashMap<usize, usize>;

fn main() {
    let mut memory = Memory::new();
    let mut current_mask = String::new();

    for op in aoc::parser::lines_from_args_as::<Op>(1).collect::<Vec<_>>() {
        match op {
            Op::Mask(s) => current_mask = s,
            Op::Mem(addr, value) => {
                memory.insert(
                    addr,
                    implode_num(&apply_mask(&current_mask, &explode_num(value as usize))),
                );
            }
        }
    }

    let res: usize = memory.values().sum();

    println!("res: {}", res);
}
