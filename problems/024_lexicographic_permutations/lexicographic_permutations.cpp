// lexicographic_permutations.cpp
// C++ reference solution using factoradic indexing. Final answer redacted.
#include <bits/stdc++.h>
using namespace std;

static vector<unsigned long long> build_fact(size_t n) {
    vector<unsigned long long> f(n + 1, 1);
    for (size_t i = 1; i <= n; ++i) f[i] = f[i-1] * i;
    return f;
}

static string nth_perm(vector<char> items, unsigned long long n1) {
    size_t m = items.size();
    auto fact = build_fact(m);
    unsigned long long k = n1 - 1; // zero-based
    string out; out.reserve(m);
    for (size_t r = m; r >= 1; --r) {
        unsigned long long f = fact[r - 1];
        size_t idx = static_cast<size_t>(k / f);
        k %= f;
        out.push_back(items[idx]);
        items.erase(items.begin() + idx);
    }
    return out;
}

int main() {
    // sanity
    vector<char> small = {'0','1','2'};
    if (nth_perm(small, 3) != string("102")) return 1;

    vector<char> digits;
    for (char c='0'; c<='9'; ++c) digits.push_back(c);
    string millionth = nth_perm(digits, 1'000'000ULL);

    // cout << millionth << "\n"; // TODO: reveal locally
    cout << "Millionth permutation of 0..9 = [redacted]\n";
    (void)millionth;
    return 0;
}
