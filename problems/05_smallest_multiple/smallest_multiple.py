"""Compute the least common multiple of 1..=20 via gcd folding.
Final print is redacted per the near-finish convention.
"""

from functools import reduce
from math import gcd


def lcm(a: int, b: int) -> int:
    if a == 0 or b == 0:
        return 0
    return a // gcd(a, b) * b


def lcm_range(n: int) -> int:
    return reduce(lcm, range(1, n + 1), 1)


def main() -> None:
    n = 20
    result = lcm_range(n)
    # print(f"LCM(1..={n}) = {result}")  # TODO: reveal locally
    print(f"LCM(1..={n}) = <compute locally>")
    assert lcm_range(10) == 2520


if __name__ == "__main__":
    main()
