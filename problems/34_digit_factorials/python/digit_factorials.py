# digit_factorials.py
# Project Euler 34 - Digit Factorials (Python)

FACT = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880]
UPPER = 7 * 362880  # 2,540,160

def digit_fact_sum(n: int) -> int:
    if n == 0:
        return FACT[0]
    s = 0
    while n > 0:
        s += FACT[n % 10]
        n //= 10
    return s

def main() -> None:
    total = 0
    for n in range(10, UPPER + 1):
        if n == digit_fact_sum(n):
            total += n
    # print(total)  # TODO: reveal locally
    print("sum of curious numbers = <redacted>")

if __name__ == "__main__":
    main()
