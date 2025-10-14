# integer_right_triangles.py
# Project Euler 39 - Integer Right Triangles (Python)

from math import gcd

def solve(limit: int = 1000) -> tuple[int, int]:
    counts = [0] * (limit + 1)
    m = 2
    while 2*m*(m+1) <= limit:
        n = 1
        while n < m:
            if ((m - n) & 1) == 1 and gcd(m, n) == 1:
                p0 = 2 * m * (m + n)
                k = 1
                while k * p0 <= limit:
                    counts[k * p0] += 1
                    k += 1
            n += 1
        m += 1
    best_p = max(range(limit+1), key=lambda p: counts[p])
    return best_p, counts[best_p]

def main() -> None:
    p, c = solve(1000)
    # print(f"p = {p}, solutions = {c}")  # TODO: reveal locally
    print("maximizing perimeter (p <= 1000) = <redacted>")

if __name__ == "__main__":
    main()
