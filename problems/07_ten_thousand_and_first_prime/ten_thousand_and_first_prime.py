# ten_thousand_and_first_prime.py
# Sieve with PNT-style upper bound. Final output redacted.

import math


def upper_bound_nth_prime(n: int) -> int:
    if n < 6:
        return 15
    nn = float(n)
    ub = nn * (math.log(nn) + math.log(math.log(nn)))
    return int(math.ceil(ub)) + 10


def sieve(limit: int) -> list[int]:
    is_prime = [True] * (limit + 1)
    if limit >= 0:
        is_prime[0] = False
    if limit >= 1:
        is_prime[1] = False
    p = 2
    while p * p <= limit:
        if is_prime[p]:
            m = p * p
            while m <= limit:
                is_prime[m] = False
                m += p
        p += 1
    return [i for i in range(2, limit + 1) if is_prime[i]]


def nth_prime(n: int) -> int:
    limit = upper_bound_nth_prime(n)
    while True:
        primes = sieve(limit)
        if len(primes) >= n:
            return primes[n - 1]
        limit *= 2


def main() -> None:
    N = 10001
    p = nth_prime(N)
    # print(p)  # TODO: reveal locally
    print(f"p_{N} = <compute locally>")
    assert nth_prime(6) == 13
    _ = p


if __name__ == "__main__":
    main()
