use std::ops::Range;
use std::str::FromStr;

#[derive(Debug)]
pub struct Password {
    policy: Policy,
    password: String,
}

#[derive(Debug)]
pub struct Policy {
    c: char,
    range: Range<usize>,
}

impl FromStr for Password {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(':').map(|s| s.trim());
        let policy = s.next().unwrap().parse()?;

        Ok(Self {
            policy,
            password: s.next().unwrap().to_string(),
        })
    }
}

impl FromStr for Policy {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(' ');
        let (range, c) = (s.next().unwrap(), s.next().unwrap());
        let range = range
            .split('-')
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?;
        let range: Range<usize> = Range {
            start: range[0],
            end: range[1] + 1,
        };
        let c = c.chars().next().unwrap();

        Ok(Self { c, range })
    }
}

impl Password {
    pub fn is_valid1(&self) -> bool {
        self.policy.range.contains(
            &self
                .password
                .chars()
                .filter(|c| *c == self.policy.c)
                .count(),
        )
    }

    pub fn is_valid2(&self) -> bool {
        (self
            .password
            .chars()
            .nth(self.policy.range.start - 1)
            .unwrap()
            == self.policy.c)
            ^ (self
                .password
                .chars()
                .nth(self.policy.range.end - 2)
                .unwrap()
                == self.policy.c)
    }
}
