// largest_product_in_a_series.cpp
// Zero-aware rolling-window product for Euler Problem 8.

#include <algorithm>
#include <cstdint>
#include <iostream>
#include <string>
#include <vector>

static const std::string NUM = R"(73167176531330624919225119674426574742355349194934
96983520312774506326239578318016984801869478851843
85861560789112949495459501737958331952853208805511
12540698747158523863050715693290963295227443043557
66896648950445244523161731856403098711121722383113
62229893423380308135336276614282806444486645238749
30358907296290491560440772390713810515859307960866
70172427121883998797908792274921901699720888093776
65727333001053367881220235421809751254540594752243
52584907711670556013604839586446706324415722155397
53697817977846174064955149290862569321978468622482
83972241375657056057490261407972968652414535100474
82166370484403199890008895243450658541227588666881
16427171479924442928230863465674813919123162824586
17866458359124566529476545682848912883142607690042
24219022671055626321111109370544217506941658960408
07198403850962455444362981230987879927244284909188
84580156166097919133875499200524063689912560717606
05886116467109405077541002256983155200055935729725
71636269561882670428252483600823257530420752963450)";

static std::vector<int> digits_from_block(const std::string& s) {
    std::vector<int> d;
    d.reserve(s.size());
    for (char c : s) {
        if (c >= '0' && c <= '9') {
            d.push_back(c - '0');
        }
    }
    return d;
}

static std::uint64_t max_product_k(const std::vector<int>& d, std::size_t k) {
    if (k == 0 || d.empty() || k > d.size()) {
        return 0;
    }

    __uint128_t prod = 1;
    std::size_t zeros = 0;
    std::uint64_t best = 0;

    for (std::size_t i = 0; i < d.size(); ++i) {
        int x = d[i];
        if (x == 0) {
            ++zeros;
        } else {
            prod *= static_cast<__uint128_t>(x);
        }

        if (i >= k) {
            int y = d[i - k];
            if (y == 0) {
                --zeros;
            } else {
                prod /= static_cast<__uint128_t>(y);
            }
        }

        if (i + 1 >= k && zeros == 0) {
            best = std::max(best, static_cast<std::uint64_t>(prod));
        }
    }

    return best;
}

int main() {
    const auto digits = digits_from_block(NUM);
    const auto best13 = max_product_k(digits, 13);
    std::cout << "best product (13) = " << best13 << '\n';

    if (max_product_k(digits, 4) != 5832ull) {
        std::cerr << "k=4 check failed" << std::endl;
        return 1;
    }

    return 0;
}
