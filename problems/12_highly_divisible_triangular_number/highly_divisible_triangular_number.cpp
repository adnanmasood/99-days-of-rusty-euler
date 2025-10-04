// highly_divisible_triangular_number.cpp
// C++ solution for Project Euler 12 with factor-splitting trick.
#include <bits/stdc++.h>
using namespace std;
static long long count_divisors(long long n) {
    if (n == 0) return 0;
    long long cnt = 1;
    long long e = 0;
    while (n % 2 == 0) { n /= 2; ++e; }
    if (e) cnt *= (e + 1);
    for (long long p = 3; p * p <= n; p += 2) {
        e = 0;
        while (n % p == 0) { n /= p; ++e; }
        if (e) cnt *= (e + 1);
    }
    if (n > 1) cnt *= 2;
    return cnt;
}
int main() {
    const long long TARGET = 500;
    long long n = 1;
    while (true) {
        long long a, b;
        if (n % 2 == 0) { a = n/2; b = n+1; }
        else { a = n; b = (n+1)/2; }
        long long d = count_divisors(a) * count_divisors(b);
        if (d > TARGET) {
            long long tri = n * (n + 1) / 2;
            // cout << tri << "\n"; // TODO: reveal locally
            cout << "Triangle number found (redacted)." << "\n";
            break;
        }
        ++n;
    }
    return 0;
}
