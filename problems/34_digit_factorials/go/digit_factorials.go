// digit_factorials.go
// Project Euler 34 - Digit Factorials (Go)
package main

import "fmt"

var FACT = [10]int{1,1,2,6,24,120,720,5040,40320,362880}
const UPPER = 7 * 362880 // 2,540,160

func digitFactSum(n int) int {
    if n == 0 { return FACT[0] }
    s := 0
    for n > 0 {
        s += FACT[n%10]
        n /= 10
    }
    return s
}

func main() {
    total := 0
    for n := 10; n <= UPPER; n++ {
        if n == digitFactSum(n) {
            total += n
        }
    }
    // fmt.Println(total) // TODO: reveal locally
    fmt.Println("sum of curious numbers = <redacted>")
}
