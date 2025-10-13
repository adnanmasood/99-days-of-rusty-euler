// digit_fifth_powers.cpp
// C++ reference solution for Project Euler 30.
// Precompute fifth powers of digits and scan up to 6*9^5. Final answer redacted.
#include <bits/stdc++.h>
using namespace std;

static unsigned powu(unsigned b, unsigned e) {
    unsigned r = 1;
    for (unsigned i=0;i<e;++i) r *= b;
    return r;
}

static unsigned sum_digit_powers(unsigned n, const array<unsigned,10>& t) {
    unsigned s = 0;
    if (n==0) return t[0];
    while (n) { s += t[n%10]; n/=10; }
    return s;
}

static unsigned solve(unsigned p) {
    array<unsigned,10> t{};
    for (unsigned d=0; d<=9; ++d) t[d] = powu(d,p);
    unsigned upper = 6 * t[9];
    unsigned total = 0;
    for (unsigned n=2; n<=upper; ++n) {
        if (sum_digit_powers(n, t) == n) total += n;
    }
    return total;
}

int main(){
    assert(solve(4) == 19316);
    unsigned ans = solve(5);
    // cout << ans << "\n"; // TODO: reveal locally
    cout << "Sum for power 5: [redacted]\n";
    (void)ans;
}
