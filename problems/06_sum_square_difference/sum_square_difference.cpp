// sum_square_difference.cpp
// Use closed forms; final print redacted.

#include <cstdint>
#include <iostream>

static unsigned long long sum_to(unsigned long long n) {
    return n * (n + 1) / 2ull;
}

static unsigned long long sum_of_squares(unsigned long long n) {
    return n * (n + 1) * (2ull * n + 1) / 6ull;
}

static unsigned long long square(unsigned long long x) {
    return x * x;
}

int main() {
    const unsigned long long N = 100ull;
    auto diff = square(sum_to(N)) - sum_of_squares(N);
    // std::cout << diff << "\n"; // TODO: reveal locally
    std::cout << "difference for 100 = <compute locally>\n";
    // Statement check for n=10:
    if (sum_of_squares(10) != 385ull) return 1;
    if (square(sum_to(10)) != 3025ull) return 1;
    if (square(sum_to(10)) - sum_of_squares(10) != (3025ull - 385ull)) return 1;
    (void)diff;
    return 0;
}
