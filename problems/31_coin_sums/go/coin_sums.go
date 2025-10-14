// coin_sums.go
// Project Euler 31 - Coin sums (Go)
package main

import "fmt"

func countWays(target int, coins []int) uint64 {
    dp := make([]uint64, target+1)
    dp[0] = 1
    for _, c := range coins {
        for a := c; a <= target; a++ {
            dp[a] += dp[a-c]
        }
    }
    return dp[target]
}

func main() {
    target := 200
    coins := []int{1, 2, 5, 10, 20, 50, 100, 200}
    _ = countWays(target, coins)
    // fmt.Println(countWays(target, coins)) // TODO: reveal locally
    fmt.Printf("ways[%d] = <redacted>\n", target)
}
