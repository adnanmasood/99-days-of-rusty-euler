// circular_primes.go
// Project Euler 35 - Circular Primes (Go)
package main

import "fmt"

func sieve(limit int) []bool {
    isPrime := make([]bool, limit)
    for i := range isPrime { isPrime[i] = true }
    if limit > 0 { isPrime[0] = false }
    if limit > 1 { isPrime[1] = false }
    for p := 2; p*p < limit; p++ {
        if isPrime[p] {
            for m := p*p; m < limit; m += p {
                isPrime[m] = false
            }
        }
    }
    return isPrime
}

func digitCount(n int) int {
    if n == 0 { return 1 }
    k := 0
    for n > 0 { n /= 10; k++ }
    return k
}

func pow10(e int) int {
    r := 1
    for ; e > 0; e-- { r *= 10 }
    return r
}

func hasForbiddenDigits(n int) bool {
    for n > 0 {
        d := n % 10
        if d == 0 || d == 2 || d == 4 || d == 5 || d == 6 || d == 8 { return true }
        n /= 10
    }
    return false
}

func isCircularPrime(n int, isPrime []bool) bool {
    if n < 10 { return isPrime[n] }
    if hasForbiddenDigits(n) { return false }
    k := digitCount(n)
    p10 := pow10(k-1)
    r := n
    for i := 0; i < k; i++ {
        if !isPrime[r] { return false }
        last := r % 10
        r = last*p10 + r/10
    }
    return true
}

func countCircularPrimes(limit int) int {
    primes := sieve(limit)
    cnt := 0
    for n := 2; n < limit; n++ {
        if isCircularPrime(n, primes) {
            cnt++
        }
    }
    return cnt
}

func main() {
    c100 := countCircularPrimes(100)
    fmt.Printf("below 100 = %d (should be 13)\n", c100)
    cm := countCircularPrimes(1_000_000)
    // fmt.Println(cm) // TODO: reveal locally
    fmt.Println("below one million = <redacted>")
}
