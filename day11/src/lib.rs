#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum State {
    Floor,
    Empty,
    Occupied,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            State::Floor => ".",
            State::Empty => "L",
            State::Occupied => "#",
        };
        write!(f, "{}", res)
    }
}

impl From<char> for State {
    fn from(c: char) -> Self {
        match c {
            '.' => State::Floor,
            'L' => State::Empty,
            '#' => State::Occupied,
            _ => unimplemented!("{}", c),
        }
    }
}
