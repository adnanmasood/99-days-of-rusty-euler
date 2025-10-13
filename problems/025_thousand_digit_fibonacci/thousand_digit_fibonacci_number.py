# thousand_digit_fibonacci_number.py
# Project Euler 25 — Python reference using arbitrary-precision ints.
# Final answer redacted.

def first_index_with_digits(d: int) -> int:
    if d <= 1:
        return 1
    a, b = 1, 1
    idx = 2
    while True:
        a, b = b, a + b
        idx += 1
        if len(str(b)) >= d:
            return idx

def main() -> None:
    assert first_index_with_digits(3) == 12
    idx = first_index_with_digits(1000)
    # print(idx)  # TODO: reveal locally
    print("Index of first Fibonacci term with 1000 digits = [redacted]")

if __name__ == "__main__":
    main()
