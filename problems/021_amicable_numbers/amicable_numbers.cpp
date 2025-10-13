// amicable_numbers.cpp
// C++ reference solution (sqrt divisor scan).
// Final Euler answer suppressed for the series.

#include <bits/stdc++.h>
using namespace std;

static long long sum_proper_divisors(long long n) {
    if (n <= 1) return 0;
    long long sum = 1;
    for (long long i = 2; i * i <= n; ++i) {
        if (n % i == 0) {
            sum += i;
            long long d = n / i;
            if (d != i) sum += d;
        }
    }
    return sum;
}

int main() {
    const long long LIMIT = 10000;
    long long total = 0;
    for (long long a = 2; a < LIMIT; ++a) {
        long long b = sum_proper_divisors(a);
        if (b != a && sum_proper_divisors(b) == a) {
            total += a;
        }
    }
    // cout << total << "\n"; // TODO: reveal locally
    cout << "Sum under " << LIMIT << " = [redacted]\n";
    return 0;
}
