// double_base_palindromes.cpp
// Project Euler 36 - Double-base Palindromes (C++)

#include <bits/stdc++.h>
using namespace std;

static bool is_pal_base(unsigned n, unsigned base) {
    if (n == 0) return true;
    vector<unsigned> d;
    while (n > 0) { d.push_back(n % base); n /= base; }
    for (size_t i = 0, j = d.size() - 1; i < j; ++i, --j)
        if (d[i] != d[j]) return false;
    return true;
}

int main() {
    const unsigned LIMIT = 1'000'000;
    unsigned long long total = 0;
    for (unsigned n = 1; n < LIMIT; n += 2) { // skip evens
        if (is_pal_base(n, 10) && is_pal_base(n, 2)) total += n;
    }
    // cout << total << "\n"; // TODO: reveal locally
    cout << "sum of double-base palindromes below one million = <redacted>\n";
    return 0;
}
