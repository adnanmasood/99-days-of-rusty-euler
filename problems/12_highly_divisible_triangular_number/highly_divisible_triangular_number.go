// highly_divisible_triangular_number.go
// Go solution for Project Euler 12 using the split-factor trick.
package main

import "fmt"
func countDivisors(n int64) int64 {
    if n == 0 { return 0 }
    var cnt int64 = 1
    var e int64 = 0
    for n % 2 == 0 {
        n /= 2
        e++
    }
    if e > 0 { cnt *= (e + 1) }
    var p int64 = 3
    for p*p <= n {
        e = 0
        for n % p == 0 {
            n /= p
            e++
        }
        if e > 0 { cnt *= (e + 1) }
        p += 2
    }
    if n > 1 { cnt *= 2 }
    return cnt
}
func main() {
    const target int64 = 500
    var n int64 = 1
    for {
        var a, b int64
        if n % 2 == 0 { a, b = n/2, n+1 } else { a, b = n, (n+1)/2 }
        d := countDivisors(a) * countDivisors(b)
        if d > target {
            tri := n * (n + 1) / 2
            // fmt.Println(tri) // TODO: reveal locally
            fmt.Println("Triangle number found (redacted).")
            break
        }
        n++
    }
}
