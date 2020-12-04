#[derive(Debug, Default)]
pub struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
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
                "byr" => passport.byr = Some(value.to_string()),
                "iyr" => passport.iyr = Some(value.to_string()),
                "eyr" => passport.eyr = Some(value.to_string()),
                "hgt" => passport.hgt = Some(value.to_string()),
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
