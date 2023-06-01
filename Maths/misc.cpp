#include <iostream>
#include <vector>

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

/**
 * @brief Solves the pascal pyramid problem
 * p(n,i,j,k) = p(n,i-1,j,k)+p(n,i-1,j-1,k)+p(n,i-1,j-1,k-1).
 * 
 * @param pyramid 3d object
 * @param n total pyramid height
 * @param i layer of the pyramid
 * @param j width
 * @param k height
 * @return int the value nC(i,j,k).
 */
int solve_pascal_pyramid(vec3d<ll>&pyramid, int n, int i, int j, int k) {
    if (ir(i, n) || ir(j, i) || ir(k, j)) {
        return 0;
    } else if (pyramid[i][j][k] != 0) {
        return pyramid[i][j][k];
    } else if ((i <= 1) || (j == 0) || (j == i && k == 0) || (j == i && k == j)) {
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
