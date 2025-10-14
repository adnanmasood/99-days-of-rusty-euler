// pandigital_products.go
// Project Euler 32 - Pandigital Products (Go)
package main

import "fmt"

func isPandigital(a, b int) bool {
    p := a * b
    s := fmt.Sprintf("%d%d%d", a, b, p)
    if len(s) != 9 {
        return false
    }
    seen := [10]bool{}
    for i := 0; i < len(s); i++ {
        d := int(s[i] - '0')
        if d == 0 || seen[d] {
            return false
        }
        seen[d] = true
    }
    for d := 1; d <= 9; d++ {
        if !seen[d] {
            return false
        }
    }
    return true
}

func main() {
    products := map[int]struct{}{}

    for a := 1; a <= 9; a++ {
        for b := 1000; b <= 9999; b++ {
            if isPandigital(a, b) {
                products[a*b] = struct{}{}
            }
        }
    }

    for a := 10; a <= 99; a++ {
        for b := 100; b <= 999; b++ {
            if isPandigital(a, b) {
                products[a*b] = struct{}{}
            }
        }
    }

    sum := 0
    for p := range products {
        sum += p
    }
    // fmt.Println(sum) // TODO: reveal locally
    fmt.Println("sum = <redacted>")
}
