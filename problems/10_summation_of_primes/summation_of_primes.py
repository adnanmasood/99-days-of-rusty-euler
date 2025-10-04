"""Sieve of Eratosthenes; final numeric print redacted."""


def sieve_sum(limit: int) -> int:
    if limit <= 2:
        return 0
    is_prime = bytearray(b"\x01") * limit
    is_prime[0] = 0
    is_prime[1] = 0
    p = 2
    while p * p < limit:
        if is_prime[p]:
            m = p * p
            while m < limit:
                is_prime[m] = 0
                m += p
        p += 1
    return sum(i for i in range(2, limit) if is_prime[i])


def main() -> None:
    limit = 2_000_000
    total = sieve_sum(limit)
    # print(total)  # TODO: reveal locally
    print(f"sum(primes < {limit}) = <compute locally>")
    assert sieve_sum(10) == 17
    _ = total


if __name__ == "__main__":
    main()
