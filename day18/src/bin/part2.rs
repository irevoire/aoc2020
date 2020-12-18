fn main() {
    let res: usize = aoc::parser::lines_from_args(1)
        .map(|s| execute(s.replace(" ", "").as_bytes()))
        .sum();

    println!("res: {:?}", res);
}

enum Op {
    El(char),
    Op(Vec<Op>),
}

fn patch(s: &str) -> String {
    let v = s.chars().enumerate().map(|(i, c)| match c {
        '(' => patch(s.split_at(i).1),
        c' => c.to_string(),
    }).

}

fn execute(s: &[u8]) -> usize {
    let mut closure: Option<Box<dyn Fn(usize) -> usize>> = None;
    let mut last = 0;

    s.iter().enumerate().filter_map(|(i, c)| match c {
        b'*' => {
            closure = Some(Box::new(move |a| a * last));
            None
        }
        b'+' => {
            closure = Some(Box::new(move |a| last + a));
            None
        }
        b'0'..=b'9' => {
            last = c.to_string().parse::<usize>().unwrap();
            if closure.is_some() {
                let res = Some(closure.as_ref().unwrap()(last));
                closure = None;
                Some(res)
            } else {
                None
            }
        }
        _ => panic!(),
    });
}
