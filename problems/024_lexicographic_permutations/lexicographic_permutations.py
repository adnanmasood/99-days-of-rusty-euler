# lexicographic_permutations.py
# Python reference solution using the factoradic indexing method. Final answer redacted.

from typing import List

def build_fact(n: int) -> List[int]:
    f = [1] * (n + 1)
    for i in range(1, n + 1):
        f[i] = f[i-1] * i
    return f

def nth_perm(items: List[str], n1: int) -> str:
    m = len(items)
    fact = build_fact(m)
    k = n1 - 1  # zero-based
    out = []
    for r in range(m, 0, -1):
        f = fact[r - 1]
        idx = k // f
        k %= f
        out.append(items.pop(idx))
    return ''.join(out)

def main() -> None:
    assert nth_perm(list("012"), 3) == "102"
    millionth = nth_perm(list("0123456789"), 1_000_000)
    # print(millionth)  # TODO: reveal locally
    print("Millionth permutation of 0..9 = [redacted]")

if __name__ == "__main__":
    main()
