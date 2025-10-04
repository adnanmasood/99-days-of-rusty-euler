"""largest_palindrome_product.py
Simple palindrome helpers and optimized search. Final print redacted.
"""

from __future__ import annotations


def is_palindrome(n: int) -> bool:
    orig, rev = n, 0
    while n > 0:
        rev = rev * 10 + (n % 10)
        n //= 10
    return orig == rev


def largest_palindrome_product_3digits() -> int:
    best = 0
    for i in range(999, 99, -1):
        step = 1 if i % 11 == 0 else 11
        j = 990 if step == 11 else 999
        if i * j <= best:
            continue
        current = j
        while current >= i:
            prod = i * current
            if prod <= best:
                break
            if is_palindrome(prod):
                best = prod
            current -= step
    return best


def main() -> None:
    best = largest_palindrome_product_3digits()
    # print(best)  # TODO: reveal locally
    print("largest pal product (3 digits) = <compute locally>")

    best2 = 0
    for a in range(10, 100):
        for b in range(a, 100):
            p = a * b
            if p > best2 and is_palindrome(p):
                best2 = p
    assert best2 == 9009
    _ = best  # avoid lint complaining about unused variable when print is redacted


if __name__ == "__main__":
    main()
