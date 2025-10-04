// lattice_paths.cpp
// Project Euler 15 - Lattice Paths (C++)
// Compute paths(n) = C(2n, n) with integer-safe multiplicative formula.
#include <bits/stdc++.h>

using namespace std;

static unsigned long long gcd_ull(unsigned long long a, unsigned long long b) {
    while (b) {
        unsigned long long t = a % b;
        a = b;
        b = t;
    }
    return a;
}

static unsigned long long binom(unsigned int n, unsigned int k) {
    if (k == 0 || k == n) {
        return 1ULL;
    }
    k = min(k, n - k);
    vector<unsigned long long> numer;
    vector<unsigned long long> denom;
    for (unsigned long long x = n - k + 1; x <= n; ++x) {
        numer.push_back(x);
    }
    for (unsigned long long x = 1; x <= k; ++x) {
        denom.push_back(x);
    }

    for (auto &d : denom) {
        if (d == 1) {
            continue;
        }
        for (auto &num : numer) {
            if (num == 1) {
                continue;
            }
            unsigned long long g = gcd_ull(num, d);
            if (g > 1) {
                num /= g;
                d /= g;
                if (d == 1) {
                    break;
                }
            }
        }
    }
    unsigned long long res = 1ULL;
    for (auto v : numer) {
        res *= v;
    }
    return res;
}

int main() {
    // 2x2 -> 6
    if (binom(4, 2) != 6ULL) {
        return 1;
    }
    unsigned long long ways = binom(40, 20);
    (void)ways; // redacted
    // cout << ways << "\n"; // TODO: reveal locally
    cout << "Computed C(40,20) (redacted)." << "\n";
    return 0;
}
