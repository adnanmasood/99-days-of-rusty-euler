// non_abundant_sums.cpp
// C++ reference solution for Project Euler 23. Final answer redacted.

#include <bits/stdc++.h>
using namespace std;

static vector<int> sum_proper_divisors_table(int limit) {
    vector<int> spd(limit + 1, 0);
    for (int i = 1; i <= limit/2; ++i) {
        for (int j = i * 2; j <= limit; j += i) {
            spd[j] += i;
        }
    }
    return spd;
}

int main() {
    const int LIMIT = 28123;
    auto spd = sum_proper_divisors_table(LIMIT);

    vector<int> abundants;
    for (int n = 1; n <= LIMIT; ++n) if (spd[n] > n) abundants.push_back(n);

    vector<char> can(LIMIT + 1, 0);
    for (size_t i = 0; i < abundants.size(); ++i) {
        for (size_t j = i; j < abundants.size(); ++j) {
            int s = abundants[i] + abundants[j];
            if (s > LIMIT) break;
            can[s] = 1;
        }
    }

    long long total = 0;
    for (int n = 1; n <= LIMIT; ++n) if (!can[n]) total += n;

    // cout << total << "\n"; // TODO: reveal locally
    cout << "Sum (<= " << LIMIT << ") of non-abundant-sum numbers = [redacted]\n";
    return 0;
}
