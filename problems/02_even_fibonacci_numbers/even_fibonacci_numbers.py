# even_fibonacci_numbers.py
# Direct and fast approaches; keep final print as TODO.

def sum_even_fib_naive(limit: int) -> int:
    a, b = 1, 2
    total = 0
    while a <= limit:
        if a % 2 == 0:
            total += a
        a, b = b, a + b
    return total

def sum_even_fib_fast(limit: int) -> int:
    # E_{k+1} = 4*E_k + E_{k-1}, starting at 2, 8
    if limit < 2:
        return 0
    e1, e2 = 2, 8
    total = 0
    if e1 <= limit:
        total += e1
    if e2 <= limit:
        total += e2
    while True:
        e3 = 4 * e2 + e1
        if e3 > limit:
            break
        total += e3
        e1, e2 = e2, e3
    return total

if __name__ == "__main__":
    # print("N=10 naive:", sum_even_fib_naive(10))  # would print 10
    # print("N=10 fast :", sum_even_fib_fast(10))   # would print 10
    limit = 4_000_000
    # print(sum_even_fib_fast(limit))  # TODO: uncomment locally
    print(f"TODO: run locally to compute the final answer for limit = {limit}")
