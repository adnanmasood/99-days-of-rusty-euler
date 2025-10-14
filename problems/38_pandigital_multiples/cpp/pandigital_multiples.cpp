// pandigital_multiples.cpp
// Project Euler 38 - Pandigital Multiples (C++)

#include <bits/stdc++.h>
using namespace std;

static unsigned digits(unsigned n) {
    if (n == 0) return 1;
    unsigned k = 0;
    while (n > 0) { n /= 10; ++k; }
    return k;
}

static unsigned pow10u(unsigned e) {
    unsigned r = 1;
    while (e--) r *= 10;
    return r;
}

static bool is_pandigital_1_to_9(unsigned n) {
    unsigned short seen = 0;
    int count = 0;
    while (n > 0) {
        unsigned d = n % 10;
        if (d == 0) return false;
        unsigned short bit = 1u << d;
        if (seen & bit) return false;
        seen |= bit;
        ++count;
        n /= 10;
    }
    return count == 9 && seen == 0b1111111110;
}

static optional<unsigned> concat_prod(unsigned x) {
    unsigned v = 0, len = 0, k = 1;
    while (len < 9) {
        unsigned p = x * k;
        unsigned dp = digits(p);
        v = v * pow10u(dp) + p;
        len += dp;
        ++k;
    }
    if (len == 9 && k - 1 >= 2) return v;
    return {};
}

int main() {
    unsigned best = 0;
    for (unsigned x = 1; x <= 9999; ++x) {
        if (auto val = concat_prod(x)) {
            if (is_pandigital_1_to_9(*val)) best = max(best, *val);
        }
    }
    // cout << best << "\n"; // TODO: reveal locally
    cout << "largest pandigital concatenated product = <redacted>\n";
    return 0;
}
