// largest_prime_factor.cpp
// Trial division up to sqrt(n); final print redacted.
#include <cstdint>
#include <iostream>

static unsigned long long largest_prime_factor(unsigned long long n) {
    unsigned long long best = 0ull;

    while (n % 2ull == 0ull) {
        best = 2ull;
        n /= 2ull;
    }

    unsigned long long f = 3ull;
    while (f <= n / f) {            // avoid f*f overflow
        if (n % f == 0ull) {
            best = f;
            while (n % f == 0ull) {
                n /= f;
            }
        }
        f += 2ull;
    }

    if (n > 1ull) {
        best = (n > best ? n : best);
    }

    return best;
}

int main() {
    const unsigned long long N = 600'851'475'143ull;
    auto ans = largest_prime_factor(N);
    // std::cout << "Largest prime factor of " << N << " is " << ans << "\n"; // TODO
    std::cout << "Largest prime factor of " << N << " is <compute locally>\n";
    (void)ans;

    // Statement check:
    if (largest_prime_factor(13'195ull) != 29ull) {
        return 1;
    }
    return 0;
}
