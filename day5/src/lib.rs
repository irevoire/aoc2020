pub fn str_to_seat_id(s: String) -> usize {
    let (row, col) = s.split_at(7);
    let row = row
        .chars()
        .fold(0..128, |range, c| match c {
            'F' => range.start..(range.end - ((range.end - range.start) / 2)),
            'B' => (range.start + (range.end - range.start) / 2)..range.end,
            _ => panic!("unexpected letter: {}", c),
        })
        .start;

    let col = col
        .chars()
        .fold(0..8, |range, c| match c {
            'L' => range.start..(range.end - ((range.end - range.start) / 2)),
            'R' => (range.start + (range.end - range.start) / 2)..range.end,
            _ => panic!("unexpected letter: {}", c),
        })
        .start;

    row * 8 + col
}
