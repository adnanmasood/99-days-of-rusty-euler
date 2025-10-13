// non_abundant_sums.go
// Go reference solution for Project Euler 23. Final answer redacted.
package main

import "fmt"

func spdTable(limit int) []int {
    spd := make([]int, limit+1)
    for i := 1; i <= limit/2; i++ {
        for j := i * 2; j <= limit; j += i {
            spd[j] += i
        }
    }
    return spd
}

func main() {
    const LIMIT = 28123
    spd := spdTable(LIMIT)

    abundants := make([]int, 0)
    for n := 1; n <= LIMIT; n++ {
        if spd[n] > n {
            abundants = append(abundants, n)
        }
    }

    can := make([]bool, LIMIT+1)
    for i := 0; i < len(abundants); i++ {
        for j := i; j < len(abundants); j++ {
            s := abundants[i] + abundants[j]
            if s > LIMIT { break }
            can[s] = true
        }
    }

    var total int64 = 0
    for n := 1; n <= LIMIT; n++ {
        if !can[n] { total += int64(n) }
    }

    // fmt.Println(total) // TODO: reveal locally
    fmt.Printf("Sum (<= %d) of non-abundant-sum numbers = [redacted]\n", LIMIT)
}
