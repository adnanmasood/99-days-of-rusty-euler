# number_spiral_diagonals.py
# Python reference for Project Euler 28 using closed form.
# Final answer redacted.

def sum_diag(n: int) -> int:
    return (4*n*n*n + 3*n*n + 8*n - 9) // 6

def main() -> None:
    assert sum_diag(1) == 1
    assert sum_diag(3) == 25
    assert sum_diag(5) == 101

    n = 1001
    ans = sum_diag(n)
    # print(ans)  # TODO: reveal locally
    print(f"Sum of diagonals for {n}x{n} spiral = [redacted]")
    _ = ans

if __name__ == "__main__":
    main()
