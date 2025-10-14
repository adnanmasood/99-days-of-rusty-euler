// digit_cancelling_fractions.cpp
// Project Euler 33 - Digit Cancelling Fractions (C++)

#include <bits/stdc++.h>
using namespace std;

static inline int gcd_int(long long a, long long b) {
    while (b != 0) { long long r = a % b; a = b; b = r; }
    return (int)a;
}

static inline bool is_curious(int n, int d) {
    int nt = n / 10, nu = n % 10;
    int dt = d / 10, du = d % 10;
    if (nt == 0 || nu == 0 || dt == 0 || du == 0) return false; // trivial-type zeros

    auto check = [&](int kn, int kd) {
        return kd != 0 && 1LL * n * kd == 1LL * d * kn;
    };

    if (nu == dt && check(nt, du)) return true;
    if (nt == du && check(nu, dt)) return true;
    if (nt == dt && check(nu, du)) return true;
    if (nu == du && check(nt, dt)) return true;
    return false;
}

int main() {
    long long num = 1, den = 1;
    for (int n = 10; n <= 99; ++n)
        for (int d = n + 1; d <= 99; ++d)
            if (is_curious(n, d)) { num *= n; den *= d; }

    int g = gcd_int(num, den);
    int ans = (int)(den / g);
    // cout << ans << "\n"; // TODO: reveal locally
    cout << "denominator (lowest terms) = <redacted>\n";
    return 0;
}
