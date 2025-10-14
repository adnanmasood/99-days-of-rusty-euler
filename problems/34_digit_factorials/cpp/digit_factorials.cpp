// digit_factorials.cpp
// Project Euler 34 - Digit Factorials (C++)

#include <bits/stdc++.h>
using namespace std;

static const int FACT[10] = {1,1,2,6,24,120,720,5040,40320,362880};
static const int UPPER = 7 * 362880; // 2,540,160

static inline int digit_fact_sum(int n) {
    if (n == 0) return FACT[0];
    int s = 0;
    while (n > 0) {
        s += FACT[n % 10];
        n /= 10;
    }
    return s;
}

int main() {
    long long total = 0;
    for (int n = 10; n <= UPPER; ++n) {
        if (n == digit_fact_sum(n)) total += n;
    }
    // cout << total << "\n"; // TODO: reveal locally
    cout << "sum of curious numbers = <redacted>\n";
    return 0;
}
