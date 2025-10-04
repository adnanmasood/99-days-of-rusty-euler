// ten_thousand_and_first_prime.cpp
// Sieve of Eratosthenes with a PNT-based upper bound. Final output redacted.

#include <cmath>
#include <cstdint>
#include <iostream>
#include <vector>

static size_t upper_bound_nth_prime(size_t n) {
    if (n < 6) return 15;
    double nn = static_cast<double>(n);
    double ub = nn * (std::log(nn) + std::log(std::log(nn)));
    return static_cast<size_t>(std::ceil(ub)) + 10;
}

static std::vector<size_t> sieve(size_t limit) {
    std::vector<bool> is_prime(limit + 1, true);
    if (limit >= 0) is_prime[0] = false;
    if (limit >= 1) is_prime[1] = false;
    for (size_t p = 2; p * p <= limit; ++p) {
        if (is_prime[p]) {
            for (size_t m = p * p; m <= limit; m += p) is_prime[m] = false;
        }
    }
    std::vector<size_t> primes;
    for (size_t i = 2; i <= limit; ++i) if (is_prime[i]) primes.push_back(i);
    return primes;
}

static size_t nth_prime(size_t n) {
    size_t limit = upper_bound_nth_prime(n);
    for (;;) {
        auto primes = sieve(limit);
        if (primes.size() >= n) return primes[n - 1];
        limit *= 2;
    }
}

int main() {
    const size_t N = 10'001;
    auto p = nth_prime(N);
    // std::cout << "p_" << N << " = " << p << "\n"; // TODO: reveal locally
    std::cout << "p_" << N << " = <compute locally>\n";
    // Statement check:
    if (nth_prime(6) != 13) return 1;
    (void)p;
    return 0;
}
