# largest_prime_factor.py
# Simple trial division; final print redacted.

def largest_prime_factor(n: int) -> int:
    best = 0

    while n % 2 == 0:
        best = 2
        n //= 2

    f = 3
    while f <= n // f:
        if n % f == 0:
            best = f
            while n % f == 0:
                n //= f
        f += 2

    if n > 1:
        best = max(best, n)

    return best


def main() -> None:
    n = 600_851_475_143
    ans = largest_prime_factor(n)
    # print(f"Largest prime factor of {n} is {ans}")  # TODO: reveal locally
    print(f"Largest prime factor of {n} is <compute locally>")

    # Statement check:
    assert largest_prime_factor(13_195) == 29


if __name__ == "__main__":
    main()
