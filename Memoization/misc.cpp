#include <iostream>
#include <vector>

using namespace std;
using ll = long long;
using namespace std;
template <typename t>
using vec = vector<t>;
template <typename t>
using vec2d = vec<vec<t>>;
template <typename t>
using vec3d = vec2d<vec<t>>;

using ll = long long;

int ir(int i, int n) {
    return i < 0 || i > n;
}

int solve_pascal_pyramid(vec3d<ll>&pyramid, int n, int i, int j, int k) {
    if (ir(i, n) || ir(j, i) || ir(k, j)) {
        return 0;
    } else if (pyramid[i][j][k] != 0) {
        return pyramid[i][j][k];
    } else if ((i <= 1) || (j == 0) || (j == i && k == 0) || (j == i && k == j)) {
        /// Use case: it's any one corner vertex of a tertrahedron
        pyramid[i][j][k] = 1;
        return pyramid[i][j][k];
    } else {
        pyramid[i][j][k] = solve_pascal_pyramid(pyramid, n, i-1, j, k) + solve_pascal_pyramid(pyramid, n, i-1, j-1, k) + solve_pascal_pyramid(pyramid, n, i-1, j-1, k-1);
        return pyramid[i][j][k];
    }
}

void test_pyramid () {
    int n = 20;
    vec3d<ll>pyramid(n, vec2d<ll>(n, vec<ll>(n, 0)));
    for (int i = 0; i < n; ++i) {
        for (int j = 0; j <= i; ++j) {
            for (int k = 0; k <= j; ++k) {
                solve_pascal_pyramid(pyramid, i, i, j, k);
            }
        }
    }
    for (int i = 0; i < n; ++i) {
        for (int j = 0; j <= i; ++j) {
            for (int k = 0; k <= j; ++k) {
                cout << pyramid[i][j][k] << " ";
            }
            cout << endl;
        }
        cout << endl;
    }
}


void maze3d() {
    int n = 10;
    vec3d<int> a(n, vec2d<int>(n, vec<int>(n, 0)));
    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < n; ++j) {
            for (int k = 0; k < n; ++k) {
                a[i][j][k] = 0;
            }
        }
    }
    a[0][0][0] = 1;
    for (int i = 0; i < n; ++i) {
        a[i][0][0] = a[0][i][0] = a[0][0][i] = 1;
    }
    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < n; ++j) {
            for (int k = 0; k < n; ++k) {
                if (!a[i][j][k]) {
                    a[i][j][k] = (i > 0 ? a[i-1][j][k] : 0) + (j > 0 ? a[i][j-1][k] : 0) + (k > 0 ? a[i][j][k-1] : 0);
                }
            }    
        }
    }
    for (int i = 0; i < 3; ++i) {
        for (int j = 0; j < 3; ++j) {
            for (int k = 0; k < 3; ++k) {
                cout << a[i][j][k] << " ";
            }
            cout << endl;
        }
        cout << endl;
    }
}

int main () {
    maze3d();
}
