# pandigital_multiples.py
# Project Euler 38 - Pandigital Multiples (Python)

def digits(n: int) -> int:
    if n == 0: return 1
    k = 0
    while n > 0:
        n //= 10
        k += 1
    return k

def pow10(e: int) -> int:
    r = 1
    for _ in range(e): r *= 10
    return r

def is_pandigital_1_to_9(n: int) -> bool:
    seen = 0
    count = 0
    while n > 0:
        d = n % 10
        if d == 0: return False
        bit = 1 << d
        if seen & bit: return False
        seen |= bit
        count += 1
        n //= 10
    return count == 9 and seen == 0b1111111110

def concat_prod(x: int) -> int | None:
    v = 0
    length = 0
    k = 1
    while length < 9:
        p = x * k
        dp = digits(p)
        v = v * pow10(dp) + p
        length += dp
        k += 1
    if length == 9 and k - 1 >= 2:
        return v
    return None

def main() -> None:
    best = 0
    for x in range(1, 10000):
        v = concat_prod(x)
        if v is not None and is_pandigital_1_to_9(v):
            best = max(best, v)
    # print(best)  # TODO: reveal locally
    print("largest pandigital concatenated product = <redacted>")

if __name__ == "__main__":
    main()
