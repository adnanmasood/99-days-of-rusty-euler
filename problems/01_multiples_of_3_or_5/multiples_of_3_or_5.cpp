// multiples_of_3_or_5.cpp
// Simple readable implementation; final print redacted.

#include <cstdint>
#include <iostream>

static bool is_multiple_of_3_or_5(std::uint64_t n) {
    return (n % 3 == 0) || (n % 5 == 0);
}

static std::uint64_t sum_scan(std::uint64_t limit) {
    std::uint64_t total = 0;
    for (std::uint64_t k = 1; k < limit; ++k) {
        if (is_multiple_of_3_or_5(k)) {
            total += k;
        }
    }
    return total;
}

int main() {
    const std::uint64_t limit = 1000;
    const auto result = sum_scan(limit);
    // std::cout << "Sum for " << limit << " = " << result << "\n"; // TODO: reveal locally
    std::cout << "Sum for " << limit << " = <compute locally>\n";
    // Example check: below 10 -> 23
    if (sum_scan(10) != 23) {
        return 1;
    }
    (void)result;
    return 0;
}
