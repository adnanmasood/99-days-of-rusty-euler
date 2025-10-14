// quadratic_primes.cpp
// C++ reference solution for Project Euler 27 (Quadratic Primes).
// Final answer redacted.

#include <bits/stdc++.h>
using namespace std;

static bool is_prime(long long n) {
    if (n <= 1) return false;
    if (n % 2 == 0) return n == 2;
    for (long long d = 3; d * d <= n; d += 2)
        if (n % d == 0) return false;
    return true;
}

static long long run_len(long long a, long long b) {
    long long n = 0;
    while (true) {
        long long val = n*n + a*n + b;
        if (!is_prime(val)) break;
        ++n;
    }
    return n;
}

int main() {
    // Sanity
    assert(run_len(1, 41) == 40);
    assert(run_len(-79, 1601) == 80);

    long long bestA=0, bestB=0, bestLen=0;
    for (long long a = -999; a <= 999; ++a) {
        for (long long b = 2; b <= 1000; ++b) {
            if (!is_prime(b)) continue;
            long long len = run_len(a, b);
            if (len > bestLen) {
                bestLen = len; bestA = a; bestB = b;
            }
        }
    }

    long long product = bestA * bestB;
    // cout << product << "\n"; // TODO: reveal locally
    cout << "Best (a,b,len,product) = [redacted]\n";
    (void)product;
    return 0;
}
