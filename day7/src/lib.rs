use std::collections::HashMap;

pub type Input = HashMap<String, Vec<(usize, String)>>;

pub fn parse() -> Input {
    aoc::parser::lines_from_args(1)
        .map(|line| {
            let line = line.replace("bags", "").replace("bag", "").replace(".", "");
            let line = line.split("contain").collect::<Vec<&str>>();
            let (key, values) = (line[0].trim().to_string(), line[1]);
            let values = values
                .split(",")
                .filter_map(|s| {
                    if s.trim() != "no other" {
                        let s = s.trim().splitn(2, ' ').collect::<Vec<&str>>();
                        Some((s[0].trim().parse().unwrap(), s[1].trim().to_string()))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(usize, String)>>();
            (key, values)
        })
        .collect::<HashMap<_, _>>()
}
