fn main() {
    let res: usize = aoc::parser::lines::<String>()
        .map(|s| day18::execute(&patch(&s.replace(" ", ""))))
        .sum();

    println!("res: {:?}", res);
}

fn patch(s: &str) -> String {
    let mut v: Vec<char> = s.chars().collect();
    let mut start = 0;

    while let Some(i) = v.iter().skip(start).position(|&c| c == '+') {
        let i = start + i;

        let left = match v[i - 1] {
            ')' => {
                i - v[..i - 1]
                    .iter()
                    .rev()
                    .scan(-1, |acc, &c| {
                        match c {
                            '(' => *acc += 1,
                            ')' => *acc -= 1,
                            _ => (),
                        };
                        Some(*acc)
                    })
                    .position(|c| c == 0)
                    .unwrap()
            }
            _ => i,
        } - 1;
        let right = match v[i + 1] {
            '(' => {
                i + v[i + 2..]
                    .iter()
                    .scan(1, |acc, &c| {
                        match c {
                            '(' => *acc += 1,
                            ')' => *acc -= 1,
                            _ => (),
                        };
                        Some(*acc)
                    })
                    .position(|c| c == 0)
                    .unwrap()
            }
            _ => i,
        } + 2;

        v.insert(right, ')');
        v.insert(left, '(');

        start = i + 2;
    }

    v.iter().collect()
}
