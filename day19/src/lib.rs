#[derive(Debug, Clone)]
pub struct Rules {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub enum Rule {
    One(char),
    Other(usize),
    List(Vec<Rule>),
    Or(Vec<Rule>),
}

impl Rules {
    pub fn check(&self, s: &str) -> bool {
        let mut index = 0;
        self.check_sub(&self.rules[0], s, &mut index) && index == s.chars().count()
    }

    fn check_sub(&self, sub_rule: &Rule, s: &str, index: &mut usize) -> bool {
        match sub_rule {
            Rule::One(c) => {
                *index += 1;
                s.chars().nth(*index - 1).unwrap() == *c
            }
            Rule::Other(i) => self.check_sub(&self.rules[*i], s, index),
            Rule::List(v) => {
                let old = *index;
                let b = v.iter().all(|r| self.check_sub(r, s, index));
                if !b {
                    *index += old;
                }
                b
            }
            Rule::Or(v) => {
                let mut base = *index;
                let b = v.iter().any(|r| {
                    base = *index;
                    self.check_sub(r, s, &mut base)
                });
                if b {
                    *index = base
                }
                b
            }
        }
    }
}

impl std::str::FromStr for Rules {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = s
            .lines()
            .map(|s| s.split(':').collect::<Vec<&str>>())
            .collect::<Vec<_>>();

        v.sort_by(|left, right| {
            left[0]
                .parse::<usize>()
                .unwrap()
                .partial_cmp(&right[0].parse::<usize>().unwrap())
                .unwrap()
        });
        let v = v.iter().map(|el| el[1].trim().parse().unwrap()).collect();

        Ok(Self { rules: v })
    }
}

impl std::str::FromStr for Rule {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = if s.starts_with('"') {
            Rule::One(s.trim().trim_matches(|c| c == '"').chars().next().unwrap())
        } else {
            Rule::Or(
                s.split('|')
                    .map(|sub| {
                        Rule::List(
                            sub.trim()
                                .split(' ')
                                .map(|s| Rule::Other(s.parse::<usize>().unwrap()))
                                .collect(),
                        )
                    })
                    .collect(),
            )
        };

        Ok(res)
    }
}
