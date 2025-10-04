# highly_divisible_triangular_number.py
# Python solution for Project Euler 12 using the split-factor trick.
# Prints a placeholder instead of the final answer.
def count_divisors(n: int) -> int:
    if n == 0:
        return 0
    cnt = 1
    e = 0
    while n % 2 == 0:
        n //= 2
        e += 1
    if e:
        cnt *= (e + 1)
    p = 3
    while p * p <= n:
        e = 0
        while n % p == 0:
            n //= p
            e += 1
        if e:
            cnt *= (e + 1)
        p += 2
    if n > 1:
        cnt *= 2
    return cnt

def main():
    target = 500
    n = 1
    while True:
        if n % 2 == 0:
            a, b = n // 2, n + 1
        else:
            a, b = n, (n + 1) // 2
        d = count_divisors(a) * count_divisors(b)
        if d > target:
            tri = n * (n + 1) // 2
            # print(tri)  # TODO: reveal locally
            print("Triangle number found (redacted).")
            break
        n += 1

if __name__ == "__main__":
    main()
