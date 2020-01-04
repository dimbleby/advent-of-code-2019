pub(crate) fn gcd(x: usize, y: usize) -> usize {
    let (mut a, mut b) = if x > y { (x, y) } else { (y, x) };
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

pub(crate) fn lcm(x: usize, y: usize) -> usize {
    (x * y) / gcd(x, y)
}
