use std::collections::HashSet;

use day8::{Instruction, Vm};

fn main() {
    let mut vm = Vm::new(aoc::parser::lines_from_args_as::<Instruction>(1).collect());

    let last = std::iter::successors(Some((0, 0)), |_| Some((vm.cycle().unwrap(), vm.acc)))
        .scan(HashSet::new(), |set, (cycle, acc)| {
            if set.insert(cycle) {
                Some(acc)
            } else {
                None
            }
        })
        .last()
        .unwrap();

    println!("last value of acc before the cycle: {}", last);
}
