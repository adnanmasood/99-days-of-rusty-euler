# longest_collatz_sequence.py
# Project Euler 14 - Longest Collatz Sequence (Python)
# Memoized iterative approach; prints placeholder.

def next_collatz(n: int) -> int:
    return n // 2 if n % 2 == 0 else 3 * n + 1

def main():
    LIMIT = 1_000_000
    small = [0] * (LIMIT + 1)
    small[1] = 1
    big = {}
    best_start, best_len = 1, 1

    for s in range(1, LIMIT):
        x = s
        trail = []
        while True:
            if x == 1:
                known = 1; break
            if x <= LIMIT and small[x] != 0:
                known = small[x]; break
            if x in big:
                known = big[x]; break
            trail.append(x)
            x = next_collatz(x)

        l = known
        for y in reversed(trail):
            l += 1
            if y <= LIMIT: small[y] = l
            else: big[y] = l

        if l > best_len:
            best_len, best_start = l, s

    # print(best_start, best_len)  # TODO: reveal locally
    print("Computed longest chain under 1e6 (redacted).")

if __name__ == "__main__":
    main()
