# pandigital_products.py
# Project Euler 32 - Pandigital Products (Python)

def is_pandigital_identity(a: int, b: int) -> bool:
    p = a * b
    s = f"{a}{b}{p}"
    if len(s) != 9:
        return False
    seen = [False] * 10
    for ch in s:
        if ch == '0':
            return False
        d = ord(ch) - ord('0')
        if seen[d]:
            return False
        seen[d] = True
    return all(seen[d] for d in range(1, 10))

def main() -> None:
    products = set()

    for a in range(1, 10):
        for b in range(1000, 10000):
            if is_pandigital_identity(a, b):
                products.add(a * b)

    for a in range(10, 100):
        for b in range(100, 1000):
            if is_pandigital_identity(a, b):
                products.add(a * b)

    total = sum(products)
    # print(total)  # TODO: reveal locally
    print("sum = <redacted>")

if __name__ == "__main__":
    main()
