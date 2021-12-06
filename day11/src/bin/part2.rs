#![feature(half_open_range_patterns)]
#![feature(bool_to_option)]
#![feature(exclusive_range_pattern)]

use aoc::*;
use day11::*;

fn main() {
    let v = aoc::parser::lines::<String>()
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

fn check_in_all_directions(grid: &Grid<State>, current: &Coord<usize>) -> usize {
    let (w, h) = (grid.width(), grid.height());
    [
        std::iter::successors(Some(Coord::at(current.x, current.y)), |c| {
            Some(Coord::at(c.x.checked_sub(1)?, c.y))
        })
        .skip(1)
        .map(|el| grid[el])
        .find(|&el| el != State::Floor)
        .map(|state| state == State::Occupied)
        .unwrap_or(false) as usize,
        std::iter::successors(Some(Coord::at(current.x, current.y)), |c| {
            Some(Coord::at(c.x.checked_sub(1)?, c.y.checked_sub(1)?))
        })
        .skip(1)
        .map(|el| grid[el])
        .find(|&el| el != State::Floor)
        .map(|state| state == State::Occupied)
        .unwrap_or(false) as usize,
        std::iter::successors(Some(Coord::at(current.x, current.y)), |c| {
            Some(Coord::at(c.x, c.y.checked_sub(1)?))
        })
        .skip(1)
        .map(|el| grid[el])
        .find(|&el| el != State::Floor)
        .map(|state| state == State::Occupied)
        .unwrap_or(false) as usize,
        std::iter::successors(Some(Coord::at(current.x, current.y)), |c| {
            Some(Coord::at(
                (w > (c.x + 1)).then_some(c.x + 1)?,
                c.y.checked_sub(1)?,
            ))
        })
        .skip(1)
        .map(|el| grid[el])
        .find(|&el| el != State::Floor)
        .map(|state| state == State::Occupied)
        .unwrap_or(false) as usize,
        std::iter::successors(Some(Coord::at(current.x, current.y)), |c| {
            Some(Coord::at((w > (c.x + 1)).then_some(c.x + 1)?, c.y))
        })
        .skip(1)
        .map(|el| grid[el])
        .find(|&el| el != State::Floor)
        .map(|state| state == State::Occupied)
        .unwrap_or(false) as usize,
        std::iter::successors(Some(Coord::at(current.x, current.y)), |c| {
            Some(Coord::at(
                (w > (c.x + 1)).then_some(c.x + 1)?,
                (h > (c.y + 1)).then_some(c.y + 1)?,
            ))
        })
        .skip(1)
        .map(|el| grid[el])
        .find(|&el| el != State::Floor)
        .map(|state| state == State::Occupied)
        .unwrap_or(false) as usize,
        std::iter::successors(Some(Coord::at(current.x, current.y)), |c| {
            Some(Coord::at(c.x, (h > (c.y + 1)).then_some(c.y + 1)?))
        })
        .skip(1)
        .map(|el| grid[el])
        .find(|&el| el != State::Floor)
        .map(|state| state == State::Occupied)
        .unwrap_or(false) as usize,
        std::iter::successors(Some(Coord::at(current.x, current.y)), |c| {
            Some(Coord::at(
                c.x.checked_sub(1)?,
                (h > (c.y + 1)).then_some(c.y + 1)?,
            ))
        })
        .skip(1)
        .map(|el| grid[el])
        .find(|&el| el != State::Floor)
        .map(|state| state == State::Occupied)
        .unwrap_or(false) as usize,
    ]
    .iter()
    .sum()
}

fn cycle(grid: &Grid<State>) -> Grid<State> {
    let (h, w) = (grid.height(), grid.width());
    let mut new = Grid::from(vec![vec![State::Floor; w]; h]);

    grid.lines().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, el)| {
            let current = Coord::at(x, y);
            if grid[current] != State::Floor {
                let new_state = match (el, check_in_all_directions(grid, &current)) {
                    (State::Empty, 0) => State::Occupied,
                    (State::Occupied, 5..) => State::Empty,
                    (state, _) => *state,
                };
                new[current] = new_state;
            }
        })
    });

    new
}
