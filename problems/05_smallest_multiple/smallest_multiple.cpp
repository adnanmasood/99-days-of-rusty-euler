// smallest_multiple.cpp
// Compute LCM(1..=20) using gcd/lcm folding; final output redacted.

#include <cstdint>
#include <iostream>

namespace {

unsigned long long gcd(unsigned long long a, unsigned long long b) {
    while (b != 0ull) {
        const auto r = a % b;
        a = b;
        b = r;
    }
    return a;
}

unsigned long long lcm(unsigned long long a, unsigned long long b) {
    if (a == 0ull || b == 0ull) {
        return 0ull;
    }
    return (a / gcd(a, b)) * b;
}

unsigned long long lcm_range(unsigned long long n) {
    unsigned long long acc = 1ull;
    for (unsigned long long x = 1ull; x <= n; ++x) {
        acc = lcm(acc, x);
    }
    return acc;
}

} // namespace

int main() {
    const unsigned long long limit = 20ull;
    const auto result = lcm_range(limit);
    // std::cout << "LCM(1..=" << limit << ") = " << result << '\n'; // TODO: reveal locally
    std::cout << "LCM(1..=" << limit << ") = <compute locally>\n";

    if (lcm_range(10) != 2520ull) {
        return 1; // sanity check failed
    }
    return 0;
}
