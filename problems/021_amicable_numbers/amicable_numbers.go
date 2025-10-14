// amicable_numbers.go
// Go reference solution (sqrt divisor scan).
// Final Euler answer intentionally not printed.

package main

import (
    "fmt"
)

func sumProperDivisors(n int) int {
    if n <= 1 {
        return 0
    }
    sum := 1
    for i := 2; i*i <= n; i++ {
        if n%i == 0 {
            sum += i
            d := n / i
            if d != i {
                sum += d
            }
        }
    }
    return sum
}

func main() {
    const LIMIT = 10000
    total := 0
    for a := 2; a < LIMIT; a++ {
        b := sumProperDivisors(a)
        if b != a && sumProperDivisors(b) == a {
            total += a
        }
    }
    // fmt.Println(total) // TODO: reveal locally
    fmt.Printf("Sum under %d = [redacted]\n", LIMIT)
}
