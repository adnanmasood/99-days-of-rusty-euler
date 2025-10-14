// circular_primes.cpp
// Project Euler 35 - Circular Primes (C++)

#include <bits/stdc++.h>
using namespace std;

static vector<bool> sieve(int limit) {
    vector<bool> is_prime(limit, true);
    if (limit > 0) is_prime[0] = false;
    if (limit > 1) is_prime[1] = false;
    for (int p = 2; 1LL*p*p < limit; ++p) if (is_prime[p]) {
        for (long long m = 1LL*p*p; m < limit; m += p) is_prime[(int)m] = false;
    }
    return is_prime;
}

static int digit_count(int n) {
    if (n == 0) return 1;
    int k = 0;
    while (n > 0) { n /= 10; ++k; }
    return k;
}

static int pow10i(int e) {
    int r = 1;
    while (e--) r *= 10;
    return r;
}

static bool has_forbidden_digits(int n) {
    while (n > 0) {
        int d = n % 10;
        if (d == 0 || d == 2 || d == 4 || d == 5 || d == 6 || d == 8) return true;
        n /= 10;
    }
    return false;
}

static bool is_circular_prime(int n, const vector<bool>& is_prime) {
    if (n < 10) return is_prime[n];
    if (has_forbidden_digits(n)) return false;
    int k = digit_count(n);
    int p10 = pow10i(k - 1);
    int r = n;
    for (int i = 0; i < k; ++i) {
        if (!is_prime[r]) return false;
        int last = r % 10;
        r = last * p10 + r / 10;
    }
    return true;
}

static int count_circular_primes(int limit) {
    auto primes = sieve(limit);
    int cnt = 0;
    for (int n = 2; n < limit; ++n) if (is_circular_prime(n, primes)) ++cnt;
    return cnt;
}

int main() {
    int c100 = count_circular_primes(100);
    cout << "below 100 = " << c100 << " (should be 13)\n";
    int cm = count_circular_primes(1'000'000);
    // cout << cm << "\n"; // TODO: reveal locally
    cout << "below one million = <redacted>\n";
    return 0;
}
