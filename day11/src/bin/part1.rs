#![feature(half_open_range_patterns)]
#![feature(bool_to_option)]
#![feature(exclusive_range_pattern)]

use aoc::*;
use day11::*;

fn main() {
    let v = aoc::parser::lines_from_args(1)
        .map(|line| line.chars().map(|c| c.into()).collect::<Vec<State>>())
        .collect();
    let base = Grid::from(v);

    let res = std::iter::successors(Some(base), |prev| {
        let current = cycle(prev);
        (current != *prev).then_some(current)
    })
    .last()
    .unwrap()
    .iter()
    .map(|&el| (el == State::Occupied) as usize)
    .sum::<usize>();

    println!("There is {} seats occupied", res);
}

fn cycle(grid: &Grid<State>) -> Grid<State> {
    let (h, w) = (grid.height(), grid.width());
    let mut new = Grid::from(vec![vec![State::Floor; w]; h]);

    grid.lines().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, el)| {
            let current = Coord::at(x, y);
            if grid[current] != State::Floor {
                let new_state = match (
                    el,
                    grid.through(
                        Coord::at(x.saturating_sub(1), y.saturating_sub(1)),
                        Coord::at((w - 1).min(x + 1), (h - 1).min(y + 1)),
                    )
                    .unwrap()
                    .map(|el| (*el == State::Occupied) as usize)
                    .sum(),
                ) {
                    (State::Empty, 0) => State::Occupied,
                    (State::Occupied, 5..) => State::Empty, // here the range start at 5 because we counted ourselves previously
                    (state, _) => *state,
                };
                new[current] = new_state;
            }
        })
    });

    new
}
