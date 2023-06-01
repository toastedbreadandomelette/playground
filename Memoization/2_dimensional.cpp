#include <iostream>
#include <vector>
#include <algorithm>
#include <cassert>

using namespace std;

int lowest_common_subsequence(const string &s, const string &t) {
    vector<vector<int>> memo(s.size() + 1, vector<int>(t.size() + 1, 0));

    for (int i = 1; i <= s.size(); ++i) {
        for (int j = 1; j <= t.size(); ++j) {
            if (s[i-1] == t[j-1]) {
                memo[i][j] = memo[i-1][j-1] + 1;
            } else {
                memo[i][j] = max(memo[i-1][j], memo[i][j-1]);
            }
        }
    }
    return memo[s.size()][t.size()];
}

int edit_distance(const string &s, const string &t) {
    vector<vector<int>> memo(s.size() + 1, vector<int>(t.size() + 1, 0));
    for (int i = 0; i <= s.size(); ++i) {
        memo[i][0] = i;
    }
    for (int j = 0; j <= s.size(); ++j) {
        memo[0][j] = j;
    }
    for (int i = 1; i <= s.size(); ++i) {
        for (int j = 1; j <= t.size(); ++j) {
            if (s[i-1] == t[j-1]) {
                memo[i][j] = memo[i-1][j-1];
            } else {
                // Take min of three states mentioned in order:
                // 1. Removing char from string s
                // 2. Add char to string s
                // 3. Replace with the char t at pos j
                memo[i][j] = 1 + min({memo[i-1][j], memo[i][j-1], memo[i-1][j-1]});
            }
        }
    }
    return memo[s.size()][t.size()];
}

void testing() {
    assert(lowest_common_subsequence("AGACATAGA", "GACATCGA") == 7);
    assert(lowest_common_subsequence("ABCDEFGH", "LMNOPQRS") == 0);

    assert(edit_distance("AGACATAGA", "GACATCGA") == 2);
}

int main () {
    testing();
    return 0;
}
