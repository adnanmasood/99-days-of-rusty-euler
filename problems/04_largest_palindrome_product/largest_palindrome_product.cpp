// largest_palindrome_product.cpp
// Readable implementation with pruning and the factor-11 trick. Final print redacted.

#include <cstdint>
#include <iostream>

namespace {

bool is_palindrome(std::uint32_t n) {
    std::uint32_t orig = n;
    std::uint32_t rev = 0;
    while (n > 0) {
        rev = rev * 10 + (n % 10);
        n /= 10;
    }
    return orig == rev;
}

std::uint32_t largest_palindrome_product_3digits() {
    std::uint32_t best = 0;
    for (int i = 999; i >= 100; --i) {
        int step = (i % 11 == 0) ? 1 : 11;
        int j = (step == 11) ? 990 : 999;
        if (static_cast<std::uint32_t>(i * j) <= best) {
            continue;
        }
        for (; j >= i; j -= step) {
            std::uint32_t prod = static_cast<std::uint32_t>(i) * static_cast<std::uint32_t>(j);
            if (prod <= best) {
                break;
            }
            if (is_palindrome(prod)) {
                best = prod;
            }
        }
    }
    return best;
}

}  // namespace

int main() {
    [[maybe_unused]] const auto best = largest_palindrome_product_3digits();
    // std::cout << best << "\n"; // TODO: reveal locally
    std::cout << "largest pal product (3 digits) = <compute locally>\n";

    std::uint32_t best2 = 0;
    for (std::uint32_t a = 10; a <= 99; ++a) {
        for (std::uint32_t b = a; b <= 99; ++b) {
            std::uint32_t p = a * b;
            if (p > best2 && is_palindrome(p)) {
                best2 = p;
            }
        }
    }
    if (best2 != 9009u) {
        return 1;
    }
    return 0;
}
