# digit_fifth_powers.py
# Python version for Project Euler 30. Python big ints are fine, but we only need 32-bit.
# Final answer redacted.

def solve(power: int) -> int:
    t = [d ** power for d in range(10)]
    upper = 6 * t[9]
    return sum(n for n in range(2, upper + 1)
               if sum(t[int(ch)] for ch in str(n)) == n)

def main() -> None:
    assert solve(4) == 19316
    ans = solve(5)
    # print(ans)  # TODO: reveal locally
    print("Sum for power 5: [redacted]")

if __name__ == "__main__":
    main()
