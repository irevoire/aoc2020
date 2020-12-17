#![feature(or_patterns)]

use std::collections::HashSet;

type Coord = (isize, isize, isize, isize);
type Grid = HashSet<Coord>;

fn main() {
    let grid: Grid = aoc::parser::lines_from_args(1)
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_x, c)| *c == '#')
                .map(move |(x, _c)| (x as isize, y as isize, 0, 0))
                .collect::<Vec<Coord>>()
        })
        .collect();

    let res = (0..6).fold(grid, |grid, _| cycle(grid)).iter().count();

    println!("res: {}", res);
}

fn cycle(grid: Grid) -> Grid {
    grid.iter()
        .flat_map(|(x, y, z, w)| {
            around((*x, *y, *z, *w))
                .chain(std::iter::once((x, y, z, w)).map(|(x, y, z, w)| (*x, *y, *z, *w)))
        })
        .collect::<Grid>()
        .into_iter()
        .filter_map(|(x, y, z, w)| {
            let neighbors = around((x, y, z, w))
                .filter(|(x, y, z, w)| grid.contains(&(*x, *y, *z, *w)))
                .count();
            match (grid.contains(&(x, y, z, w)), neighbors) {
                (true, 2 | 3) => Some((x, y, z, w)),
                (false, 3) => Some((x, y, z, w)),
                _ => None,
            }
        })
        .collect()
}

fn around((x, y, z, w): Coord) -> impl Iterator<Item = Coord> {
    (x - 1..=x + 1)
        .flat_map(move |x| {
            (y - 1..=y + 1).flat_map(move |y| {
                (z - 1..=z + 1).flat_map(move |z| (w - 1..=w + 1).map(move |w| (x, y, z, w)))
            })
        })
        .filter(move |(cx, cy, cz, cw)| (x, y, z, w) != (*cx, *cy, *cz, *cw))
}
