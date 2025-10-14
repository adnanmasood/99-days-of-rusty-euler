# champernownes_constant.py
# Project Euler 40 - Champernowne's Constant (Python)

def digit_at(n: int) -> int:
    d = 1
    block = 9 * (10 ** (d - 1)) * d
    while n > block:
        n -= block
        d += 1
        block = 9 * (10 ** (d - 1)) * d
    idx = n - 1
    num = (10 ** (d - 1)) + idx // d
    digit_index = idx % d
    return int(str(num)[digit_index])

def main() -> None:
    targets = [1, 10, 1_000, 100, 10_000, 100_000, 1_000_000]  # order doesn't matter for product
    targets.sort()
    prod = 1
    for t in targets:
        prod *= digit_at(t)
    # print(prod)  # TODO: reveal locally
    print("product d1*d10*...*d1000000 = <redacted>")
    assert digit_at(12) == 1  # sanity from the statement

if __name__ == "__main__":
    main()
