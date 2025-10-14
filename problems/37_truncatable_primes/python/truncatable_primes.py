# truncatable_primes.py
# Project Euler 37 - Truncatable Primes (Python)

from math import isqrt

def is_prime(n: int) -> bool:
    if n < 2: return False
    if n % 2 == 0: return n == 2
    if n % 3 == 0: return n == 3
    f = 5
    while f * f <= n:
        if n % f == 0 or n % (f + 2) == 0:
            return False
        f += 6
    return True

def digits_count(n: int) -> int:
    if n == 0: return 1
    k = 0
    while n > 0:
        n //= 10
        k += 1
    return k

def pow10i(e: int) -> int:
    r = 1
    for _ in range(e): r *= 10
    return r

def grow_right(layer: list[int]) -> list[int]:
    next_layer: list[int] = []
    for p in layer:
        for d in (1,3,7,9):
            n = p * 10 + d
            if is_prime(n): next_layer.append(n)
    return next_layer

def grow_left(layer: list[int]) -> list[int]:
    next_layer: list[int] = []
    for p in layer:
        base = pow10i(digits_count(p))
        for d in (1,2,3,5,7,9):
            n = d * base + p
            if is_prime(n): next_layer.append(n)
    return next_layer

def build(grow) -> set[int]:
    res: set[int] = set()
    layer = [2,3,5,7]
    while True:
        next_layer = grow(layer)
        for n in next_layer:
            if n >= 10: res.add(n)
        if not next_layer:
            break
        layer = next_layer
    return res

def main() -> None:
    right = build(grow_right)
    left  = build(grow_left)
    both = right & left
    total = sum(n for n in both if n >= 10)
    print(f"found {len(both)} truncatable primes (should be 11)")
    # print(total)  # TODO: reveal locally
    print("sum of the eleven truncatable primes = <redacted>")

if __name__ == "__main__":
    main()
