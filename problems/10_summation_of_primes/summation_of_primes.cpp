// summation_of_primes.cpp
// Sieve of Eratosthenes; final numeric print redacted.

#include <bits/stdc++.h>

static unsigned long long sieve_sum(unsigned int limit) {
    if (limit <= 2) {
        return 0ULL;
    }
    std::vector<char> is_prime(limit, true);
    is_prime[0] = false;
    is_prime[1] = false;
    for (unsigned int p = 2; p * p < limit; ++p) {
        if (is_prime[p]) {
            for (unsigned int m = p * p; m < limit; m += p) {
                is_prime[m] = false;
            }
        }
    }
    unsigned long long sum = 0;
    for (unsigned int i = 2; i < limit; ++i) {
        if (is_prime[i]) {
            sum += i;
        }
    }
    return sum;
}

int main() {
    const unsigned int LIMIT = 2'000'000;
    auto total = sieve_sum(LIMIT);
    // std::cout << total << "\n"; // TODO: reveal locally
    std::cout << "sum(primes < " << LIMIT << ") = <compute locally>\n";
    if (sieve_sum(10) != 17ULL) {
        return 1;
    }
    return 0;
}
