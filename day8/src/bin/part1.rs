#![feature(bool_to_option)]

use std::collections::HashSet;

use day8::{Instruction, Vm};

fn main() {
    let mut vm = Vm::new(aoc::parser::lines::<Instruction>().collect());

    let last = std::iter::successors(Some((0, 0)), |_| Some((vm.cycle().unwrap(), vm.acc)))
        .scan(HashSet::new(), |set, (cycle, acc)| {
            set.insert(cycle).then_some(acc)
        })
        .last()
        .unwrap();

    println!("last value of acc before the cycle: {}", last);
}
