#include <iostream>
#include <chrono>
#include <vector>
#include "./lib/math.hpp"

using u8 = unsigned char;
using u16 = unsigned short int;
using u32 = unsigned;
using u64 = unsigned long long;
using u128 = __uint128_t;
using usize = size_t;

using i8 = char;
using i16 = short int;
using i32 = int;
using i64 = long long;
using i128 = __int128_t;
using f32 = float;
using f64 = double;
using f128 = long double;

int main () {
    f32 n = 0.3312321;
    int sz = 10000000;
    std::vector<f32> d(sz);
    auto start = std::chrono::system_clock::now();
    for (int i = 0; i < sz; ++i) {
        d[i] = math::exp(i * n);
    }
    auto end = std::chrono::system_clock::now();
    std::chrono::duration<f64> time = end - start;
    std::cout << "Custom Time: " << time.count() << '\n';

    start = std::chrono::system_clock::now();
    for (int i = 0; i < sz; ++i) {
        if (i < 100) {
            std::cout << d[i] << ' ' << ::exp(i * n) << '\n';
        }
        d[i] = ::exp(i * n);
    }
    end = std::chrono::system_clock::now();
    time = end - start;
    std::cout << "Time: " << time.count() << '\n';
    return 0;
}
