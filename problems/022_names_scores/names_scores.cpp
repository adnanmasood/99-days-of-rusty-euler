// names_scores.cpp
// C++ reference solution for Project Euler 22. Final answer redacted.

#include <bits/stdc++.h>
using namespace std;

static long long alpha_value(const string& s) {
    long long v = 0;
    for (unsigned char c : s) {
        if (isalpha(c)) {
            v += toupper(c) - 'A' + 1;
        }
    }
    return v;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    const string file = "names.txt";
    string raw;
    ifstream fin(file);
    if (fin) {
        ostringstream ss;
        ss << fin.rdbuf();
        raw = ss.str();
    } else {
        raw = "\"MARY\",\"PATRICIA\",\"LINDA\",\"BARBARA\",\"COLIN\"";
    }

    vector<string> names;
    string token;
    string cur;
    // Simple split by comma
    stringstream in(raw);
    while (getline(in, token, ',')) {
        // trim surrounding quotes and spaces
        while (!token.empty() && (token.front()=='\"' || isspace((unsigned char)token.front()))) token.erase(token.begin());
        while (!token.empty() && (token.back()=='\"' || isspace((unsigned char)token.back()))) token.pop_back();
        if (!token.empty()) names.push_back(token);
    }

    sort(names.begin(), names.end());

    long long total = 0;
    for (size_t i = 0; i < names.size(); ++i) {
        total += (long long)(i + 1) * alpha_value(names[i]);
    }

    // cout << total << "\n"; // TODO: reveal locally
    cout << "Total of name scores = [redacted]; computed " << names.size() << " names\n";
    return 0;
}
