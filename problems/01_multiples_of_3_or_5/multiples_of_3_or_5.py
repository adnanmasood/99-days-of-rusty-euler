# multiples_of_3_or_5.py
# Straightforward scan; final print redacted.


def is_multiple_of_3_or_5(n: int) -> bool:
    return n % 3 == 0 or n % 5 == 0


def sum_scan(limit: int) -> int:
    total = 0
    for k in range(1, limit):  # exclusive upper bound
        if is_multiple_of_3_or_5(k):
            total += k
    return total


def main() -> None:
    limit = 1000
    result = sum_scan(limit)
    # print(result)  # TODO: reveal locally
    print(f"Sum for {limit} = <compute locally>")
    # Example from the statement:
    assert sum_scan(10) == 23
    _ = result


if __name__ == "__main__":
    main()
