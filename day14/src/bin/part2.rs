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
                let nb_fluct = current_mask.chars().filter(|c| *c == 'X').count();
                Fluctuation::new(nb_fluct).for_each(|fluctuation| {
                    memory.insert(
                        implode_num(&apply_mask(&current_mask, &fluctuation, &explode_num(addr))),
                        value,
                    );
                });
            }
        }
    }

    let res: usize = memory.values().sum();

    println!("res: {}", res);
}

pub fn apply_mask(mask: &str, fluctuation: &str, value: &str) -> String {
    let mut fluctuation = fluctuation.chars();

    mask.chars()
        .zip(
            // we add some padding so the value get to the right of the mask
            std::iter::repeat('0')
                .take(mask.len() - value.len())
                .chain(value.chars()),
        )
        .map(|(mask, value)| match mask {
            '0' => value,
            'X' => fluctuation.next().unwrap(),
            m => m,
        })
        .collect()
}

pub struct Fluctuation {
    s: Vec<char>,
}

impl Fluctuation {
    pub fn new(n: usize) -> Self {
        Self { s: vec!['0'; n] }
    }
}

impl Iterator for Fluctuation {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let res = Some(self.s.iter().collect());

        if self.s.is_empty() {
            return None;
        } else if self.s.iter().all(|c| *c == '1') {
            self.s = Vec::new();
            return res;
        }
        for i in (0..self.s.len()).rev() {
            match self.s[i] {
                '0' => {
                    self.s[i] = '1';
                    break;
                }
                '1' => self.s[i] = '0',
                _ => unreachable!(),
            }
        }

        res
    }
}
