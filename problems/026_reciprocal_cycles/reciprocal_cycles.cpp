// reciprocal_cycles.cpp
// C++ reference solution for Project Euler 26. Final answer redacted.

#include <bits/stdc++.h>
using namespace std;

static int strip_twos_fives(int n) {
    while (n % 2 == 0) n /= 2;
    while (n % 5 == 0) n /= 5;
    return n;
}

static int cycle_len(int d) {
    int n = strip_twos_fives(d);
    if (n == 1) return 0;
    int k = 1;
    int rem = 10 % n;
    while (rem != 1) {
        rem = (rem * 10) % n;
        ++k;
    }
    return k;
}

int main() {
    // examples
    assert(cycle_len(2) == 0);
    assert(cycle_len(3) == 1);
    assert(cycle_len(7) == 6);

    int best_d = 0, best_len = 0;
    for (int d = 2; d < 1000; ++d) {
        int len = cycle_len(d);
        if (len > best_len) {
            best_len = len;
            best_d = d;
        }
    }

    // cout << best_d << " " << best_len << "\n"; // TODO: reveal locally
    cout << "d<1000 with longest recurring cycle: d=[redacted], len=[redacted]\n";
    return 0;
}
