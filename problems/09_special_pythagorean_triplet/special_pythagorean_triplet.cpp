// special_pythagorean_triplet.cpp
// Euclid parameterization; final numeric print redacted.

#include <bits/stdc++.h>

using namespace std;

static long long gcdll(long long a, long long b) {
    while (b) {
        long long t = a % b;
        a = b;
        b = t;
    }
    return llabs(a);
}

static optional<tuple<long long, long long, long long>> find_triplet_sum(long long s) {
    for (long long m = 2; 2 * m * (m + 1) <= s; ++m) {
        for (long long n = 1; n < m; ++n) {
            if (((m - n) & 1) == 1 && gcdll(m, n) == 1) {
                long long denom = 2 * m * (m + n);
                if (s % denom == 0) {
                    long long k = s / denom;
                    long long a = k * (m * m - n * n);
                    long long b = k * (2 * m * n);
                    long long c = k * (m * m + n * n);
                    if (a > b) {
                        swap(a, b);
                    }
                    return {{a, b, c}};
                }
            }
        }
    }
    return nullopt;
}

int main() {
    const long long S = 1000;
    auto t = find_triplet_sum(S);
    if (t) {
        auto [a, b, c] = *t;
        long long prod = a * b * c;
        (void)prod;
        // cout << a << " " << b << " " << c << " -> " << prod << "\n"; // TODO: reveal locally
        cout << "Product = <compute locally>\n";
    }
    // Sanity: S=12 -> 3,4,5
    auto t2 = find_triplet_sum(12);
    if (!t2) {
        return 1;
    }
    auto [a, b, c] = *t2;
    if (a * a + b * b != c * c) {
        return 1;
    }
    return 0;
}
