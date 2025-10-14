# distinct_powers.py
# Python reference solution for Project Euler 29.
# Python integers are arbitrary precision: we can directly compute {a**b}.
# Final answer redacted.

def distinct_count(a_lo, a_hi, b_lo, b_hi):
    return len({pow(a, b) for a in range(a_lo, a_hi+1) for b in range(b_lo, b_hi+1)})

def main() -> None:
    assert distinct_count(2, 5, 2, 5) == 15
    total = distinct_count(2, 100, 2, 100)
    # print(total)  # TODO: reveal locally
    print("Distinct terms for 2<=a<=100, 2<=b<=100: [redacted]")

if __name__ == "__main__":
    main()
