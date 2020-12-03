pub fn how_many_tree(grid: &aoc::Grid<bool>, slope: &aoc::Coord<usize>) -> usize {
    let mut pos = aoc::Coord::default();

    let width = grid.data[0].len();
    let height = grid.data.len();

    let mut cpt = 0_usize;
    while pos.y < height {
        cpt += grid[pos] as usize;
        pos += *slope;
        pos.x %= width;
    }

    cpt
}
