// smallest_multiple_simple.rs
// LCM by folding with gcd. Final answer redacted per the near-finish policy.

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn lcm(a: u128, b: u128) -> u128 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a / gcd(a, b)) * b
}

/// LCM of all integers in 1..=n via a fold over gcd.
fn lcm_range(n: u128) -> u128 {
    (1..=n).fold(1u128, |acc, x| lcm(acc, x))
}

fn main() {
    const N: u128 = 20;
    let result = lcm_range(N);
    // println!("LCM(1..={}) = {}", N, result); // TODO: reveal locally
    println!("LCM(1..={}) = <compute locally>", N);
    // Sanity check from the statement: LCM(1..=10) = 2520.
    assert_eq!(lcm_range(10), 2520);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcm_up_to_10() {
        assert_eq!(lcm_range(10), 2520);
    }

    #[test]
    fn lcm_up_to_1() {
        assert_eq!(lcm_range(1), 1);
    }
}
