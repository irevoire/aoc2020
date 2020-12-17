#![feature(or_patterns)]

use std::collections::HashSet;

type Coord = (isize, isize, isize);
type Grid = HashSet<Coord>;

fn main() {
    let grid: Grid = aoc::parser::lines_from_args(1)
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_x, c)| *c == '#')
                .map(move |(x, _c)| (x as isize, y as isize, 0))
                .collect::<Vec<Coord>>()
        })
        .collect();

    let res = (0..6).fold(grid, |grid, _| cycle(grid)).iter().count();

    println!("res: {}", res);
}

fn cycle(grid: Grid) -> Grid {
    grid.iter()
        // all the coord we need to check, there is a lot of duplicate here but it should still be
        // fast enough
        .flat_map(|(x, y, z)| {
            around((*x, *y, *z)).chain(std::iter::once((x, y, z)).map(|(x, y, z)| (*x, *y, *z)))
        })
        .filter_map(|(x, y, z)| {
            let neighbors = around((x, y, z))
                .filter(|(x, y, z)| grid.contains(&(*x, *y, *z)))
                .count();
            match (grid.contains(&(x, y, z)), neighbors) {
                (true, 2 | 3) => Some((x, y, z)),
                (false, 3) => Some((x, y, z)),
                _ => None,
            }
        })
        .collect()
}

fn around((x, y, z): Coord) -> impl Iterator<Item = Coord> {
    (x - 1..=x + 1)
        .flat_map(move |x| {
            (y - 1..=y + 1).flat_map(move |y| (z - 1..=z + 1).map(move |z| (x, y, z)))
        })
        .filter(move |(cx, cy, cz)| (x, y, z) != (*cx, *cy, *cz))
}
