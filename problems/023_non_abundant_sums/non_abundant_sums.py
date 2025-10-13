# non_abundant_sums.py
# Python reference solution for Project Euler 23. Final answer redacted.

def spd_table(limit: int) -> list[int]:
    spd = [0] * (limit + 1)
    for i in range(1, limit // 2 + 1):
        for j in range(i * 2, limit + 1, i):
            spd[j] += i
    return spd

def main() -> None:
    LIMIT = 28123
    spd = spd_table(LIMIT)
    abundants = [n for n in range(1, LIMIT + 1) if spd[n] > n]

    can = [False] * (LIMIT + 1)
    for i, a in enumerate(abundants):
        for b in abundants[i:]:
            s = a + b
            if s > LIMIT:
                break
            can[s] = True

    total = sum(n for n in range(1, LIMIT + 1) if not can[n])
    # print(total)  # TODO: reveal locally
    print(f"Sum (<= {LIMIT}) of non-abundant-sum numbers = [redacted]")

if __name__ == "__main__":
    main()
