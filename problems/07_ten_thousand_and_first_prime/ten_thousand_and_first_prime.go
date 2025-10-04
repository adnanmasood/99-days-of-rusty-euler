// ten_thousand_and_first_prime.go
// Sieve with upper bound. Final output redacted.

package main

import (
    "fmt"
    "math"
)

func upperBoundNthPrime(n int) int {
    if n < 6 {
        return 15
    }
    nn := float64(n)
    ub := nn * (math.Log(nn) + math.Log(math.Log(nn)))
    return int(math.Ceil(ub)) + 10
}

func sieve(limit int) []int {
    isPrime := make([]bool, limit+1)
    for i := range isPrime {
        isPrime[i] = true
    }
    if limit >= 0 {
        isPrime[0] = false
    }
    if limit >= 1 {
        isPrime[1] = false
    }
    for p := 2; p*p <= limit; p++ {
        if isPrime[p] {
            for m := p * p; m <= limit; m += p {
                isPrime[m] = false
            }
        }
    }
    primes := make([]int, 0)
    for i := 2; i <= limit; i++ {
        if isPrime[i] {
            primes = append(primes, i)
        }
    }
    return primes
}

func nthPrime(n int) int {
    limit := upperBoundNthPrime(n)
    for {
        primes := sieve(limit)
        if len(primes) >= n {
            return primes[n-1]
        }
        limit *= 2
    }
}

func main() {
    const N = 10001
    p := nthPrime(N)
    // fmt.Println(p) // TODO: reveal locally
    fmt.Printf("p_%d = <compute locally>\n", N)
    if nthPrime(6) != 13 {
        panic("6th prime should be 13")
    }
    _ = p
}
