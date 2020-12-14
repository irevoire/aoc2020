#[derive(Debug, Clone)]
pub enum Op {
    Mask(String),
    Mem(usize, u64),
}

impl std::str::FromStr for Op {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mask") {
            Ok(Op::Mask(s.split("=").nth(1).unwrap().trim().to_string()))
        } else if s.starts_with("mem") {
            let m: &[_] = &['m', 'e', '[', ']', ' '];
            let mut s = s.split("=").map(|s| s.trim_matches(m));
            Ok(Op::Mem(
                s.next().unwrap().parse()?,
                s.next().unwrap().parse()?,
            ))
        } else {
            panic!("fuck ya")
        }
    }
}

pub fn explode_num(n: usize) -> String {
    let mut s = String::new();
    if n != 0 {
        s.push_str(&explode_num(n >> 1));
        s.push_str(&(n & 1).to_string());
    }
    s
}

pub fn implode_num(n: &str) -> usize {
    n.chars()
        .map(|c| (c == '1') as usize)
        .fold(0, |acc, n| (acc << 1) + n)
}

pub fn apply_mask(mask: &str, value: &str) -> String {
    mask.chars()
        .zip(
            std::iter::repeat('0')
                .take(mask.len() - value.len())
                .chain(value.chars()),
        )
        .map(|(mask, value)| match mask {
            'X' => value,
            m => m,
        })
        .collect()
}
