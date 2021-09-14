fn main() {
    let mut lines = aoc::parser::lines_from_args(1);

    let _time = lines.next();
    let bus = lines
        .next()
        .unwrap()
        .split(',')
        .map(|el| el.parse::<i128>().ok())
        .enumerate()
        .filter_map(|el| Some((-(el.0 as i128) % el.1?, el.1?)))
        .reduce(|acc, el| (bezout_identity(el, acc), el.1 * acc.1))
        .unwrap();

    println!("res: {}", bus.0);
}

/// x = a (mod n)
/// x = b (mod m)
///
/// return x
fn bezout_identity((a, n): (i128, i128), (b, m): (i128, i128)) -> i128 {
    let (_, u, v) = extended_euclidean_algorithm(n, m);
    let res = (b * u * n) + (a * v * m);
    res.rem_euclid(n * m)
}

/// compute the extended eclidean algorithm:
/// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
///
/// It return three number `(r, v, u)` corresponding to this equation:
/// `(a * v) + (b * u) = r`
/// Also `r` is the pgcd of `a` and `b`
fn extended_euclidean_algorithm(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (d, u, v) = extended_euclidean_algorithm(b, a.rem_euclid(b));
        (d, v, u - ((a / b) * v))
    }
}
