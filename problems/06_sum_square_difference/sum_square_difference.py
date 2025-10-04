# sum_square_difference.py
# Closed-form formulas; final print redacted.


def sum_to(n: int) -> int:
    return n * (n + 1) // 2


def sum_of_squares(n: int) -> int:
    return n * (n + 1) * (2 * n + 1) // 6


def square(x: int) -> int:
    return x * x


def difference(n: int) -> int:
    return square(sum_to(n)) - sum_of_squares(n)


def main() -> None:
    n = 100
    diff = difference(n)
    # print(diff)  # TODO: reveal locally
    print(f"difference for {n} = <compute locally>")
    # Statement check:
    assert sum_of_squares(10) == 385
    assert square(sum_to(10)) == 3025
    assert difference(10) == 3025 - 385
    _ = diff


if __name__ == "__main__":
    main()
