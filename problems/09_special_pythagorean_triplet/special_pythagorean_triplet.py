# special_pythagorean_triplet.py
# Euclid parameterization; final numeric print redacted.
from __future__ import annotations

from math import gcd


def find_triplet_sum(s: int) -> tuple[int, int, int] | None:
    m = 2
    while 2 * m * (m + 1) <= s:
        for n in range(1, m):
            if ((m - n) & 1) == 1 and gcd(m, n) == 1:
                denom = 2 * m * (m + n)
                if s % denom == 0:
                    k = s // denom
                    a = k * (m * m - n * n)
                    b = k * (2 * m * n)
                    c = k * (m * m + n * n)
                    if a > b:
                        a, b = b, a
                    return a, b, c
        m += 1
    return None


def main() -> None:
    s = 1000
    t = find_triplet_sum(s)
    if t:
        a, b, c = t
        _prod = a * b * c
        # print(a, b, c, _prod)  # TODO: reveal locally
        print("Product = <compute locally>")
    # sanity
    assert find_triplet_sum(12) == (3, 4, 5)


if __name__ == "__main__":
    main()
