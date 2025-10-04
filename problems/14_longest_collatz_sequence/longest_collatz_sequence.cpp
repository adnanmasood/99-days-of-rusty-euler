// longest_collatz_sequence.cpp
// Project Euler 14 - Longest Collatz Sequence (C++)
// Memoized iterative approach. Prints placeholder to avoid spoilers.
#include <bits/stdc++.h>
using namespace std;

static inline unsigned long long next_collatz(unsigned long long n) {
    return (n % 2 == 0) ? (n / 2) : (3 * n + 1);
}

int main() {
    const size_t LIMIT = 1'000'000;
    vector<unsigned int> small(LIMIT + 1, 0);
    small[1] = 1;
    unordered_map<unsigned long long, unsigned int> big;
    unsigned long long best_start = 1;
    unsigned int best_len = 1;

    for (size_t s = 1; s < LIMIT; ++s) {
        unsigned long long x = s;
        vector<unsigned long long> trail;
        unsigned int known = 0;

        while (true) {
            if (x == 1) { known = 1; break; }
            if (x <= LIMIT && small[x] != 0) { known = small[x]; break; }
            auto it = big.find(x);
            if (it != big.end()) { known = it->second; break; }
            trail.push_back(x);
            x = next_collatz(x);
        }

        unsigned int len = known;
        for (auto it = trail.rbegin(); it != trail.rend(); ++it) {
            ++len;
            if (*it <= LIMIT) small[*it] = len;
            else big[*it] = len;
        }

        if (len > best_len) { best_len = len; best_start = s; }
    }

    // cout << best_start << " " << best_len << "\n"; // TODO: reveal locally
    cout << "Computed longest chain under 1e6 (redacted)." << "\n";
    return 0;
}
