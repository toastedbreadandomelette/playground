#include <bits/stdc++.h>

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

int main () {
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
    return 0;
}
/*
def test(n,r,m):
    return (fac(n)//(fac(r)*fac(m)*fac(n-r-m))), (fac(n-1)//(fac(r-1)*fac(m)*fac(n-r-m))+fac(n-1)//(fac(r)*fac(m-1)*fac(n-m-r))+fac(n-1)//(fac(r)*fac(m)*fac(n-r-m-1)))
*/
