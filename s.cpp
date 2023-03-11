#include <bits/stdc++.h>

using namespace std;
using ll = long long;

int main()
{
    int n = 10;
    int a[n][n][n];
    for (int i = 0; i < n; ++i)
    {
        for (int j = 0; j < n; ++j)
        {
            for (int k = 0; k < n; ++k)
            {
                a[i][j][k] = 0;
            }
        }
    }
    a[0][0][0] = 1;
    for (int i = 0; i < n; ++i)
    {
        a[i][0][0] = a[0][i][0] = a[0][0][i] = 1;
    }
    for (int i = 0; i < n; ++i)
    {
        for (int j = 0; j < n; ++j)
        {
            for (int k = 0; k < n; ++k)
            {
                if (!a[i][j][k])
                {
                    a[i][j][k] = (i > 0 ? a[i - 1][j][k] : 0) + (j > 0 ? a[i][j - 1][k] : 0) + (k > 0 ? a[i][j][k - 1] : 0);
                }
            }
        }
    }
    for (int i = 0; i < 3; ++i)
    {
        for (int j = 0; j < 3; ++j)
        {
            for (int k = 0; k < 3; ++k)
            {
                cout << a[i][j][k] << " ";
            }
            cout << endl;
        }
        cout << endl;
    }
}
