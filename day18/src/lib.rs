pub fn execute(s: &str) -> usize {
    let mut s = s.to_string();
    loop {
        let (n, sub) = execute_binop(&s);
        if sub == "" {
            return n.unwrap();
        } else if n.is_none() {
            panic!("n is none");
        }
        s = sub.to_string();
        s.insert_str(0, &n.unwrap().to_string());
    }
}

// execute things in the form of (a + b)
fn execute_parens(s: &str) -> (Option<usize>, &str) {
    let parens = s
        .chars()
        .skip(1)
        .scan(1, |acc, c| {
            match c {
                '(' => *acc += 1,
                ')' => *acc -= 1,
                _ => (),
            };
            Some(*acc)
        })
        .position(|c| c == 0)
        .unwrap()
        + 1;

    let parens = s.split_at(parens);
    let n = execute(parens.0.split_at(1).1);

    (Some(n), parens.1.split_at(1).1)
}

// execute things in the form of `a + b` or `a * b`
fn execute_binop(s: &str) -> (Option<usize>, &str) {
    let (left, sub) = get_n(s);
    let (left, sub) = left.map_or_else(|| execute_parens(s), |v| (Some(v), sub));

    if left.is_none() {
        return (None, s);
    }
    let (op, sub) = get_op(sub);
    if op.is_none() {
        return (left, sub);
    }
    let (right, sub) = get_n(sub);
    let (right, sub) = right.map_or_else(|| execute_parens(sub), |v| (Some(v), sub));
    if right.is_none() {
        return (None, s);
    }

    match op.unwrap() {
        '+' => (Some(left.unwrap() + right.unwrap()), sub),
        '*' => (Some(left.unwrap() * right.unwrap()), sub),
        c => panic!("found {}", c),
    }
}

// === primitive parser ===

// parse number
fn get_n(s: &str) -> (Option<usize>, &str) {
    match s
        .chars()
        .take_while(|c| c.is_numeric())
        .collect::<String>()
        .parse::<usize>()
        .ok()
    {
        res @ Some(_) => (
            res,
            s.split_at(s.chars().position(|c| !c.is_numeric()).unwrap_or(s.len()))
                .1,
        ),
        None => (None, s),
    }
}

// parse operator
fn get_op(s: &str) -> (Option<char>, &str) {
    match s.chars().next() {
        op @ Some('+' | '*') => (op, s.split_at(1).1),
        None => (None, s),
        _ => (None, s),
    }
}
