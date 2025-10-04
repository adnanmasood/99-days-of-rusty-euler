# lattice_paths.py
# Project Euler 15 - Lattice Paths (Python)
# Simple: use math.comb for C(2n, n); print placeholder.
import math

def paths(n: int) -> int:
    return math.comb(2 * n, n)

if __name__ == "__main__":
    assert paths(2) == 6
    ans = paths(20)
    _ = ans
    # print(ans)  # TODO: reveal locally
    print("Computed C(40,20) (redacted).")
