# amicable_numbers.py
# Python reference solution (sqrt divisor scan).
# Final Euler answer intentionally not printed.

def sum_proper_divisors(n: int) -> int:
    if n <= 1:
        return 0
    s = 1
    i = 2
    while i * i <= n:
        if n % i == 0:
            s += i
            d = n // i
            if d != i:
                s += d
        i += 1
    return s

def main() -> None:
    LIMIT = 10_000
    total = 0
    for a in range(2, LIMIT):
        b = sum_proper_divisors(a)
        if b != a and sum_proper_divisors(b) == a:
            total += a
    # print(total)  # TODO: reveal locally
    print(f"Sum under {LIMIT} = [redacted]")

if __name__ == "__main__":
    main()
