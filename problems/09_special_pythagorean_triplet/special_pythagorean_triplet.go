// special_pythagorean_triplet.go
// Euclid parameterization; final numeric print redacted.
package main

import "fmt"

func gcd(a, b int64) int64 {
    if a < 0 {
        a = -a
    }
    if b < 0 {
        b = -b
    }
    for b != 0 {
        a, b = b, a%b
    }
    return a
}

func findTripletSum(s int64) (int64, int64, int64, bool) {
    for m := int64(2); 2*m*(m+1) <= s; m++ {
        for n := int64(1); n < m; n++ {
            if ((m-n)&1) == 1 && gcd(m, n) == 1 {
                den := 2 * m * (m + n)
                if s%den == 0 {
                    k := s / den
                    a := k * (m*m - n*n)
                    b := k * (2 * m * n)
                    c := k * (m*m + n*n)
                    if a > b {
                        a, b = b, a
                    }
                    return a, b, c, true
                }
            }
        }
    }
    return 0, 0, 0, false
}

func main() {
    const S int64 = 1000
    if a, b, c, ok := findTripletSum(S); ok {
        _ = a * b * c
        // fmt.Printf("a=%d b=%d c=%d product=%d\n", a, b, c, a*b*c) // TODO
        fmt.Println("Product = <compute locally>")
    }
    if a, b, c, ok := findTripletSum(12); !ok || a*a+b*b != c*c {
        panic("sanity failed")
    }
}
