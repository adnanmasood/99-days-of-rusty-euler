// pandigital_products.cpp
// Project Euler 32 - Pandigital Products (C++)

#include <bits/stdc++.h>
using namespace std;

static inline bool is_pandigital_identity(int a, int b) {
    int p = a * b;
    string s = to_string(a) + to_string(b) + to_string(p);
    if ((int)s.size() != 9) return false;
    bool seen[10] = {false};
    for (char ch : s) {
        if (ch == '0') return false;
        int d = ch - '0';
        if (seen[d]) return false;
        seen[d] = true;
    }
    for (int d = 1; d <= 9; ++d) if (!seen[d]) return false;
    return true;
}

int main() {
    unordered_set<int> prods;

    for (int a = 1; a <= 9; ++a)
        for (int b = 1000; b <= 9999; ++b)
            if (is_pandigital_identity(a, b)) prods.insert(a * b);

    for (int a = 10; a <= 99; ++a)
        for (int b = 100; b <= 999; ++b)
            if (is_pandigital_identity(a, b)) prods.insert(a * b);

    long long sum = 0;
    for (int p : prods) sum += p;
    // cout << sum << "\n"; // TODO: reveal locally
    cout << "sum = <redacted>\n";
    return 0;
}
