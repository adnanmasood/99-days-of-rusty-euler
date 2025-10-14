# quadratic_primes.py
# Python reference solution for Project Euler 27. Final answer redacted.

def is_prime(n: int) -> bool:
    if n <= 1:
        return False
    if n % 2 == 0:
        return n == 2
    d = 3
    while d * d <= n:
        if n % d == 0:
            return False
        d += 2
    return True

def run_len(a: int, b: int) -> int:
    n = 0
    while True:
        val = n*n + a*n + b
        if not is_prime(val):
            break
        n += 1
    return n

def main() -> None:
    assert run_len(1, 41) == 40
    assert run_len(-79, 1601) == 80

    best_a = best_b = best_len = 0
    for a in range(-999, 1000):
        for b in range(2, 1001):
            if not is_prime(b):
                continue
            l = run_len(a, b)
            if l > best_len:
                best_len, best_a, best_b = l, a, b

    product = best_a * best_b
    # print(product)  # TODO: reveal locally
    print("Best (a,b,len,product) = [redacted]")

if __name__ == "__main__":
    main()
