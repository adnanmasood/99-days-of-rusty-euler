// integer_right_triangles.cpp
// Project Euler 39 - Integer Right Triangles (C++)

#include <bits/stdc++.h>
using namespace std;

static unsigned gcd_u(unsigned a, unsigned b) {
    while (b) { unsigned t = a % b; a = b; b = t; }
    return a;
}

int main() {
    const unsigned LIMIT = 1000;
    vector<unsigned> counts(LIMIT + 1, 0);
    for (unsigned m = 2; 2*m*(m+1) <= LIMIT; ++m) {
        for (unsigned n = 1; n < m; ++n) {
            if (((m - n) & 1u) && gcd_u(m, n) == 1) {
                unsigned p0 = 2 * m * (m + n);
                for (unsigned k = 1; k * p0 <= LIMIT; ++k) {
                    counts[k * p0]++;
                }
            }
        }
    }
    unsigned best_p = 0, best_c = 0;
    for (unsigned p = 0; p <= LIMIT; ++p) {
        if (counts[p] > best_c) { best_c = counts[p]; best_p = p; }
    }
    // cout << "p = " << best_p << ", solutions = " << best_c << "\n"; // TODO: reveal locally
    cout << "maximizing perimeter (p <= 1000) = <redacted>\n";
    return 0;
}
