// number_spiral_diagonals.cpp
// C++ reference solution for Project Euler 28 (Number Spiral Diagonals).
// Uses the closed form: S(n) = (4n^3 + 3n^2 + 8n - 9) / 6 for odd n.
// Final answer redacted.

#include <bits/stdc++.h>
using namespace std;

long long sum_diag(long long n) {
    // n is odd
    return (4*n*n*n + 3*n*n + 8*n - 9) / 6;
}

int main() {
    assert(sum_diag(1) == 1);
    assert(sum_diag(3) == 25);
    assert(sum_diag(5) == 101);
    long long n = 1001;
    long long ans = sum_diag(n);
    // cout << ans << "\n"; // TODO: reveal locally
    cout << "Sum of diagonals for " << n << "x" << n << " spiral = [redacted]\n";
    (void)ans;
    return 0;
}
