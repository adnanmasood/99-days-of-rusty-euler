# coin_sums.py
# Project Euler 31 - Coin sums (Python)

from typing import List

def count_ways(target: int, coins: List[int]) -> int:
    dp = [0] * (target + 1)
    dp[0] = 1
    for c in coins:
        for a in range(c, target + 1):
            dp[a] += dp[a - c]
    return dp[target]

def main() -> None:
    TARGET = 200
    COINS = [1, 2, 5, 10, 20, 50, 100, 200]
    _ = count_ways(TARGET, COINS)
    # print(count_ways(TARGET, COINS))  # TODO: reveal locally
    print(f"ways[{TARGET}] = <redacted>")

if __name__ == "__main__":
    main()
