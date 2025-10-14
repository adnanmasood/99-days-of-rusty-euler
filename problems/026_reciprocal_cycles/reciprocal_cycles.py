# reciprocal_cycles.py
# Python reference solution for Project Euler 26. Final answer redacted.

def strip_twos_fives(n: int) -> int:
    while n % 2 == 0:
        n //= 2
    while n % 5 == 0:
        n //= 5
    return n

def cycle_len(d: int) -> int:
    n = strip_twos_fives(d)
    if n == 1:
        return 0
    k = 1
    rem = 10 % n
    while rem != 1:
        rem = (rem * 10) % n
        k += 1
    return k

def main() -> None:
    assert cycle_len(2) == 0
    assert cycle_len(3) == 1
    assert cycle_len(7) == 6

    best_d, best_len = 0, 0
    for d in range(2, 1000):
        l = cycle_len(d)
        if l > best_len:
            best_len, best_d = l, d

    # print(best_d, best_len)  # TODO: reveal locally
    print("d<1000 with longest recurring cycle: d=[redacted], len=[redacted]")

if __name__ == "__main__":
    main()
