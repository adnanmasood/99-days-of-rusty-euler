// even_fibonacci_numbers.cpp
// Direct and fast approaches; keep final print as TODO.

#include <iostream>
using namespace std;

long long sum_even_fib_naive(int limit) {
    long long total = 0;
    long long a = 1, b = 2;
    while (a <= limit) {
        if (a % 2 == 0) total += a;
        long long next = a + b;
        a = b;
        b = next;
    }
    return total;
}

// Recurrence over even Fibonacci numbers: E_{k+1} = 4*E_k + E_{k-1}
long long sum_even_fib_fast(int limit) {
    if (limit < 2) return 0;
    long long e1 = 2, e2 = 8, total = 0;
    if (e1 <= limit) total += e1;
    if (e2 <= limit) total += e2;
    while (true) {
        long long e3 = 4 * e2 + e1;
        if (e3 > limit) break;
        total += e3;
        e1 = e2;
        e2 = e3;
    }
    return total;
}

int main() {
    // cout << "N=10 naive -> " << sum_even_fib_naive(10) << "\n"; // would print 10
    // cout << "N=10 fast  -> " << sum_even_fib_fast(10)  << "\n"; // would print 10

    int limit = 4'000'000;
    // cout << sum_even_fib_fast(limit) << "\n"; // TODO: uncomment locally
    cout << "TODO: run locally to compute the final answer for limit = " << limit << "\n";
}
