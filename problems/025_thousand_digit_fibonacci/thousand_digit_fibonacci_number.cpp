// thousand_digit_fibonacci_number.cpp
// Project Euler 25 — C++ reference using string-based big integer addition.
// Final answer redacted.

#include <bits/stdc++.h>
using namespace std;

static string add_str(const string& a, const string& b) {
    int i = (int)a.size() - 1;
    int j = (int)b.size() - 1;
    int carry = 0;
    string out;
    out.reserve(max(a.size(), b.size()) + 1);
    while (i >= 0 || j >= 0 || carry) {
        int da = (i >= 0 ? a[i] - '0' : 0);
        int db = (j >= 0 ? b[j] - '0' : 0);
        int s = da + db + carry;
        out.push_back(char('0' + (s % 10)));
        carry = s / 10;
        --i; --j;
    }
    reverse(out.begin(), out.end());
    return out;
}

static int first_index_with_digits(int d) {
    if (d <= 1) return 1;
    string a = "1", b = "1";
    int idx = 2;
    while (true) {
        string c = add_str(a, b);
        ++idx;
        if ((int)c.size() >= d) return idx;
        a = move(b);
        b = move(c);
    }
}

int main() {
    // sanity
    if (first_index_with_digits(3) != 12) return 1;
    int idx = first_index_with_digits(1000);
    // cout << idx << "\n"; // TODO: reveal locally
    cout << "Index of first Fibonacci term with 1000 digits = [redacted]\n";
    (void)idx;
    return 0;
}
