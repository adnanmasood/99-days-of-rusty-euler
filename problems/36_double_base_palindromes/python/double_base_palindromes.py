# double_base_palindromes.py
# Project Euler 36 - Double-base Palindromes (Python)

def is_pal_base(n: int, base: int) -> bool:
    if n == 0:
        return True
    digits = []
    x = n
    while x > 0:
        digits.append(x % base)
        x //= base
    i, j = 0, len(digits) - 1
    while i < j:
        if digits[i] != digits[j]:
            return False
        i += 1
        j -= 1
    return True

def main() -> None:
    LIMIT = 1_000_000
    total = 0
    for n in range(1, LIMIT, 2):  # skip evens
        if is_pal_base(n, 10) and is_pal_base(n, 2):
            total += n
    # print(total)  # TODO: reveal locally
    print("sum of double-base palindromes below one million = <redacted>")

if __name__ == "__main__":
    main()
