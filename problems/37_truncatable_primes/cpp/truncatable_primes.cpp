// truncatable_primes.cpp
// Project Euler 37 - Truncatable Primes (C++)

#include <bits/stdc++.h>
using namespace std;

static bool is_prime(long long n) {
    if (n < 2) return false;
    if (n % 2 == 0) return n == 2;
    if (n % 3 == 0) return n == 3;
    for (long long f = 5; f * f <= n; f += 6)
        if (n % f == 0 || n % (f + 2) == 0) return false;
    return true;
}

static int digits_count(long long n) {
    if (n == 0) return 1;
    int k = 0;
    while (n > 0) { n /= 10; ++k; }
    return k;
}

static long long pow10i(int e) {
    long long r = 1;
    while (e--) r *= 10;
    return r;
}

static vector<long long> grow_right(const vector<long long>& layer) {
    vector<long long> next;
    for (auto p : layer) {
        for (int d : {1,3,7,9}) {
            long long n = p * 10 + d;
            if (is_prime(n)) next.push_back(n);
        }
    }
    return next;
}

static vector<long long> grow_left(const vector<long long>& layer) {
    vector<long long> next;
    for (auto p : layer) {
        long long base = pow10i(digits_count(p));
        for (int d : {1,2,3,5,7,9}) {
            long long n = d * base + p;
            if (is_prime(n)) next.push_back(n);
        }
    }
    return next;
}

static unordered_set<long long> build(function<vector<long long>(const vector<long long>&)> grow) {
    unordered_set<long long> result;
    vector<long long> layer = {2,3,5,7};
    while (true) {
        auto next = grow(layer);
        for (auto n : next) if (n >= 10) result.insert(n);
        if (next.empty()) break;
        layer.swap(next);
    }
    return result;
}

int main() {
    auto right = build(grow_right);
    auto left  = build(grow_left);

    long long total = 0;
    int cnt = 0;
    for (auto n : right) {
        if (left.count(n) && n >= 10) {
            total += n;
            ++cnt;
        }
    }
    cout << "found " << cnt << " truncatable primes (should be 11)\n";
    // cout << total << "\n"; // TODO: reveal locally
    cout << "sum of the eleven truncatable primes = <redacted>\n";
    return 0;
}
