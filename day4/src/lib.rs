#[derive(Debug, Default)]
pub struct Passport {
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hgt: Option<Height>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    pub fn is_valid(&self) -> bool {
        self.byr
            .as_ref()
            .and(self.iyr.as_ref())
            .and(self.eyr.as_ref())
            .and(self.hgt.as_ref())
            .and(self.hcl.as_ref())
            .and(self.ecl.as_ref())
            .and(self.pid.as_ref())
            .is_some()
    }

    pub fn is_valid2(&self) -> bool {
        self.is_valid()
            && (1920..=2002).contains(&self.byr.unwrap())
            && (2010..=2020).contains(&self.iyr.unwrap())
            && (2020..=2030).contains(&self.eyr.unwrap())
            && self.hgt.unwrap().valid()
            && self.hcl.as_ref().unwrap().starts_with('#')
            && self
                .hcl
                .as_ref()
                .unwrap()
                .chars()
                .skip(1)
                .all(char::is_alphanumeric)
            && ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .contains(&self.ecl.as_ref().unwrap().as_ref())
            && self.pid.as_ref().unwrap().chars().count() == 9
            && self.pid.as_ref().unwrap().chars().all(char::is_numeric)
    }
}

impl std::str::FromStr for Passport {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(":", " ");
        let s = s.split_whitespace().collect::<Vec<&str>>();

        let mut passport = Passport::default();

        for data in s.chunks(2) {
            let (name, value) = (data[0], data[1]);
            match name {
                "byr" => passport.byr = Some(value.parse()?),
                "iyr" => passport.iyr = Some(value.parse()?),
                "eyr" => passport.eyr = Some(value.parse()?),
                "hgt" => passport.hgt = Some(value.parse()?),
                "hcl" => passport.hcl = Some(value.to_string()),
                "ecl" => passport.ecl = Some(value.to_string()),
                "pid" => passport.pid = Some(value.to_string()),
                "cid" => passport.cid = Some(value.to_string()),
                _ => panic!("{}", name),
            }
        }

        Ok(passport)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Height {
    Cm(usize),
    In(usize),
    Unknown(usize),
}

impl Height {
    pub fn valid(&self) -> bool {
        match self {
            Self::Cm(u) => (150..=193).contains(u),
            Self::In(u) => (59..=76).contains(u),
            _ => false,
        }
    }
}

impl std::str::FromStr for Height {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.split(char::is_alphabetic).next().unwrap().parse()?;
        let unit = s.rsplit(char::is_numeric).next();

        Ok(match unit {
            Some("cm") => Height::Cm(value),
            Some("in") => Height::In(value),
            _ => Height::Unknown(value),
        })
    }
}
