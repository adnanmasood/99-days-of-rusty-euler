// double_base_palindromes.go
// Project Euler 36 - Double-base Palindromes (Go)
package main

import "fmt"

func isPalBase(n, base int) bool {
    if n == 0 { return true }
    var d []int
    for x := n; x > 0; x /= base {
        d = append(d, x%base)
    }
    for i, j := 0, len(d)-1; i < j; i, j = i+1, j-1 {
        if d[i] != d[j] { return false }
    }
    return true
}

func main() {
    const LIMIT = 1_000_000
    total := 0
    for n := 1; n < LIMIT; n += 2 { // skip evens
        if isPalBase(n, 10) && isPalBase(n, 2) {
            total += n
        }
    }
    // fmt.Println(total) // TODO: reveal locally
    fmt.Println("sum of double-base palindromes below one million = <redacted>")
}
