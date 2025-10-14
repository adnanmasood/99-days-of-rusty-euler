// champernownes_constant.cpp
// Project Euler 40 - Champernowne's Constant (C++)

#include <bits/stdc++.h>
using namespace std;

static unsigned pow10u(unsigned e) {
    unsigned r = 1;
    while (e--) r *= 10;
    return r;
}

static unsigned digit_at(unsigned n) {
    unsigned d = 1;
    unsigned block = 9u * pow10u(d - 1) * d;
    while (n > block) {
        n -= block;
        d += 1;
        block = 9u * pow10u(d - 1) * d;
    }
    unsigned idx = n - 1;
    unsigned num = pow10u(d - 1) + idx / d;
    unsigned digit_index = idx % d;
    string s = to_string(num);
    return (unsigned)(s[digit_index] - '0');
}

int main() {
    unsigned targets[] = {1u,10u,100u,1000u,10000u,100000u,1000000u};
    unsigned long long prod = 1;
    for (auto t : targets) prod *= digit_at(t);
    // cout << prod << "\n"; // TODO: reveal locally
    cout << "product d1*d10*...*d1000000 = <redacted>\n";

    // Sanity
    assert(digit_at(12) == 1);
    return 0;
}
