use std::collections::HashSet;

use day8::{Instruction, Vm};

fn main() {
    let mem = aoc::parser::lines_from_args_as::<Instruction>(1).collect::<Vec<_>>();

    let res = (0..mem.len())
        .find_map(|index| match mem[index] {
            Instruction::Nop(i) => {
                let mut mem = mem.clone();
                mem[index] = Instruction::Jmp(i);
                looping(Vm::new(mem))
            }
            Instruction::Jmp(i) => {
                let mut mem = mem.clone();
                mem[index] = Instruction::Nop(i);
                looping(Vm::new(mem))
            }
            _ => None,
        })
        .unwrap();

    println!("{}", res);
}

fn looping(mut vm: Vm) -> Option<isize> {
    let mut end = None;

    let _ = std::iter::successors(Some((0, 0)), |_| {
        let cycle = vm.cycle();
        if cycle.is_none() {
            end = Some(vm.acc);
            None
        } else {
            Some((cycle.unwrap(), vm.acc))
        }
    })
    .scan(HashSet::new(), |set, (cycle, acc)| {
        if set.insert(cycle) {
            Some(acc)
        } else {
            None
        }
    })
    .last();

    end
}
