#[inline]
pub(crate) fn gcd(a: u32, b: u32) -> u32 {
    let (mut gcd, mut remainder) = if a > b { (a, b) } else { (b, a) };
    loop {
        if remainder == 0 {
            break;
        }

        let new_remainder = gcd % remainder;
        gcd = remainder;
        remainder = new_remainder;
    }

    gcd
}

#[inline]
pub(crate) fn lcm(lhs: u32, rhs: u32) -> u32 {
    let a = lhs;
    let b = rhs;

    let gcd = gcd(a, b);
    a / gcd * b
}
