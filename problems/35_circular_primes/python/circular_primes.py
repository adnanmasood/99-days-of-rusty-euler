# circular_primes.py
# Project Euler 35 - Circular Primes (Python)

def sieve(limit: int) -> list[bool]:
    is_prime = [True] * limit
    if limit > 0: is_prime[0] = False
    if limit > 1: is_prime[1] = False
    p = 2
    while p * p < limit:
        if is_prime[p]:
            m = p * p
            while m < limit:
                is_prime[m] = False
                m += p
        p += 1
    return is_prime

def digit_count(n: int) -> int:
    if n == 0: return 1
    k = 0
    while n > 0:
        n //= 10
        k += 1
    return k

def pow10(e: int) -> int:
    r = 1
    for _ in range(e):
        r *= 10
    return r

def has_forbidden_digits(n: int) -> bool:
    while n > 0:
        d = n % 10
        if d in (0,2,4,5,6,8):
            return True
        n //= 10
    return False

def is_circular_prime(n: int, is_prime: list[bool]) -> bool:
    if n < 10:
        return is_prime[n]
    if has_forbidden_digits(n):
        return False
    k = digit_count(n)
    p10 = pow10(k - 1)
    r = n
    for _ in range(k):
        if not is_prime[r]:
            return False
        last = r % 10
        r = last * p10 + r // 10
    return True

def count_circular_primes(limit: int) -> int:
    primes = sieve(limit)
    return sum(1 for n in range(2, limit) if is_circular_prime(n, primes))

def main() -> None:
    print(f"below 100 = {count_circular_primes(100)} (should be 13)")
    cm = count_circular_primes(1_000_000)
    # print(cm)  # TODO: reveal locally
    print("below one million = <redacted>")

if __name__ == "__main__":
    main()
