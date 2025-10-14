// distinct_powers.cpp
// C++ reference solution for Project Euler 29 (Distinct Powers).
// Represent a^b by its vector of prime exponents (no big integers).
// Final answer redacted.

#include <bits/stdc++.h>
using namespace std;

static vector<int> primes_up_to(int n) {
    vector<bool> isp(n+1, true);
    isp[0]=isp[1]=false;
    for (int p=2; p*p<=n; ++p) if (isp[p]) for (int m=p*p; m<=n; m+=p) isp[m]=false;
    vector<int> ps;
    for (int i=2;i<=n;++i) if (isp[i]) ps.push_back(i);
    return ps;
}

static vector<int> exps_of(int a, const vector<int>& ps) {
    vector<int> v(ps.size(), 0);
    for (size_t i=0;i<ps.size();++i) {
        int p = ps[i];
        if (p*p>a) break;
        while (a % p == 0) { v[i]++; a/=p; }
    }
    if (a>1) {
        auto it = find(ps.begin(), ps.end(), a);
        v[it-ps.begin()]++;
    }
    return v;
}

static long long distinct_count(int a_lo,int a_hi,int b_lo,int b_hi) {
    auto ps = primes_up_to(a_hi);
    set<vector<int>> S;
    for (int a=a_lo;a<=a_hi;++a) {
        auto base = exps_of(a, ps);
        for (int b=b_lo;b<=b_hi;++b) {
            auto v = base;
            for (auto &e: v) e *= b;
            S.insert(v);
        }
    }
    return (long long)S.size();
}

int main() {
    assert(distinct_count(2,5,2,5) == 15);
    long long total = distinct_count(2,100,2,100);
    // cout << total << "\n"; // TODO: reveal locally
    cout << "Distinct terms for 2<=a<=100, 2<=b<=100: [redacted]\n";
    (void)total;
    return 0;
}
