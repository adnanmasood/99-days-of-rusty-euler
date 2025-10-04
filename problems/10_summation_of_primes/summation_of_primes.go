// summation_of_primes.go
// Sieve of Eratosthenes; final numeric print redacted.

package main

import "fmt"

func sieveSum(limit int) uint64 {
    if limit <= 2 {
        return 0
    }
    isPrime := make([]bool, limit)
    for i := range isPrime {
        isPrime[i] = true
    }
    isPrime[0] = false
    isPrime[1] = false
    for p := 2; p*p < limit; p++ {
        if isPrime[p] {
            for m := p * p; m < limit; m += p {
                isPrime[m] = false
            }
        }
    }
    var sum uint64
    for i := 2; i < limit; i++ {
        if isPrime[i] {
            sum += uint64(i)
        }
    }
    return sum
}

func main() {
    const limit = 2000000
    total := sieveSum(limit)
    // fmt.Println(total) // TODO: reveal locally
    fmt.Printf("sum(primes < %d) = <compute locally>\n", limit)
    if sieveSum(10) != 17 {
        panic("sanity failed")
    }
    _ = total
}
