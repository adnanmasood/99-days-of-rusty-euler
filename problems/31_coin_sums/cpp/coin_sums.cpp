// coin_sums.cpp
// Project Euler 31 - Coin sums (C++)
#include <bits/stdc++.h>
using namespace std;

long long count_ways(int target, const vector<int>& coins) {
    vector<long long> dp(target + 1, 0);
    dp[0] = 1;
    for (int c : coins) {
        for (int a = c; a <= target; ++a) {
            dp[a] += dp[a - c];
        }
    }
    return dp[target];
}

int main() {
    const int TARGET = 200;
    vector<int> coins = {1,2,5,10,20,50,100,200};
    auto ways = count_ways(TARGET, coins);
    // cout << ways << "\n"; // TODO: reveal locally
    cout << "ways[" << TARGET << "] = <redacted>\n";
    return 0;
}
