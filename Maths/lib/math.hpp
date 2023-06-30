#pragma once
#ifndef __MATH_HPP__
#define __MATH_HPP__

#include "./utils.hpp"
#include <stdint.h>
#include <limits>
#include <cmath>

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


namespace Usize {
constexpr usize min = 0;
constexpr usize max = ~0;
}  // namespace Usize

namespace I8 {
constexpr i8 min = -128;
constexpr i8 max = 127;
}  // namespace I8

namespace I16 {
constexpr i16 min = -32768;
constexpr i16 max = 32767;
}  // namespace I16

namespace I32 {
constexpr i32 min = -2147483648;
constexpr i32 max = 2147483647;
}  // namespace I32

namespace I64 {
constexpr i64 min = -9223372036854775807L - 1;
constexpr i64 max = 9223372036854775807L;
}  // namespace I64

namespace U8 {
constexpr u8 min = 0U;
constexpr u8 max = 255U;
}  // namespace U8

namespace U16 {
constexpr u16 min = 0U;
constexpr u16 max = 65535U;
}  // namespace U16

namespace U32 {
constexpr u32 min = 0U;
constexpr u32 max = 4294967295U;
}  // namespace U32

namespace U64 {
constexpr u64 min = 0U;
constexpr u64 max = 18446744073709551615ULL;
}  // namespace U64

namespace F32 {
constexpr bool isnan(f32 v) { return isnanf(v); };
constexpr f32 nan = std::numeric_limits<f32>::quiet_NaN();
constexpr f32 inf = std::numeric_limits<f32>::infinity();
constexpr f32 ninf = inf * -1;
}  // namespace F32

namespace F64 {
constexpr bool isnan(f64 v) { return std::isnan(v); };
constexpr f64 nan = std::numeric_limits<f64>::quiet_NaN();
constexpr f64 inf = std::numeric_limits<f64>::infinity();
constexpr f64 ninf = inf * -1;
}  // namespace F64

namespace F128 {
constexpr bool isnan(f128 v) { return std::isnan(v); };
constexpr f128 nan = std::numeric_limits<f128>::quiet_NaN();
constexpr f128 inf = std::numeric_limits<f128>::infinity();
constexpr f128 ninf = inf * -1;
}  // namespace F128

namespace math {

/**
 * @brief Reverse at max 64-bits of a 64 bit number
 * @param n unsigned number
 * @param bit_size size to be reversed
 * @return reversed bit number
 */
constexpr inline usize reverse_bits(const usize n, const usize bit_size = 64) {
    usize rn = (n << 32) | (n >> 32);
    rn = ((rn << 16) & 0xFFFF0000FFFF0000) | ((rn >> 16) & 0x0000FFFF0000FFFF);
    rn = ((rn << 8) & 0xFF00FF00FF00FF00) | ((rn >> 8) & 0x00FF00FF00FF00FF);
    rn = ((rn << 4) & 0xF0F0F0F0F0F0F0F0) | ((rn >> 4) & 0x0F0F0F0F0F0F0F0F);
    rn = ((rn << 2) & 0xCCCCCCCCCCCCCCCC) | ((rn >> 2) & 0x3333333333333333);
    rn = ((rn << 1) & 0xAAAAAAAAAAAAAAAA) | ((rn >> 1) & 0x5555555555555555);
    return (rn >> (64 - bit_size));
}

/**
 * @brief Reverse at max 32-bits of a 32 bit number
 * @param n unsigned number
 * @param bit_size size to be reversed
 * @return reversed bit number
 */
constexpr inline u32 reverse_bits_32(const u32 n, const u32 bit_size = 32) {
    u32 rn = (n << 16) | (n >> 16);
    rn = ((rn << 8) & 0xFF00FF00) | ((rn >> 8) & 0x00FF00FF);
    rn = ((rn << 4) & 0xF0F0F0F0) | ((rn >> 4) & 0x0F0F0F0F);
    rn = ((rn << 2) & 0xCCCCCCCC) | ((rn >> 2) & 0x33333333);
    rn = ((rn << 1) & 0xAAAAAAAA) | ((rn >> 1) & 0x55555555);
    return (rn >> (32 - bit_size));
}

/**
 * @brief Reverse at max 16-bits of a 16 bit number
 * @param n unsigned number
 * @param bit_size size to be reversed
 * @return reversed bit number
 */
constexpr inline usize reverse_bits_16(const u16 n, const u16 bit_size = 16) {
    u16 rn = (rn << 8) | (rn >> 8);
    rn = ((rn << 4) & 0xF0F0) | ((rn >> 4) & 0x0F0F);
    rn = ((rn << 2) & 0xCCCC) | ((rn >> 2) & 0x3333);
    rn = ((rn << 1) & 0xAAAA) | ((rn >> 1) & 0x5555);
    return (rn >> (16 - bit_size));
}

/**
 * @brief Reverse at max 8-bits of a 8 bit number
 * @param n unsigned number
 * @param bit_size size to be reversed
 * @return reversed bit number
 */
constexpr inline u8 reverse_bits_8(const u8 n, const u8 bit_size = 8) {
    u8 rn = (rn << 4) | (rn >> 4);
    rn = ((rn << 2) & 0xCC) | ((rn >> 2) & 0x33);
    rn = ((rn << 1) & 0xAA) | ((rn >> 1) & 0x55);
    return (rn >> (8 - bit_size));
}


template <usize...>
struct reversed_bits_32 {};

/**
 * @brief Evaluate the reversed bits at compilation time.
 */
template <usize N, usize bit_size>
struct reversed_bits_32<N, bit_size> {
    static constexpr u32 value = reverse_bits_32(N, bit_size);
};


template <usize...>
struct reversed_bits {};

/**
 * @brief Evaluate the reversed bits at compilation time.
 */
template <usize N, usize bit_size>
struct reversed_bits<N, bit_size> {
    static constexpr usize value = reverse_bits(N, bit_size);
};

template <usize...>
struct reversed_bits_16 {};

/**
 * @brief Evaluate the reversed bits at compilation time.
 */
template <usize N, usize bit_size>
struct reversed_bits_16<N, bit_size> {
    static constexpr u16 value = reverse_bits_16(N, bit_size);
};

template <usize...>
struct reversed_bits_8 {};

/**
 * @brief Evaluate the reversed bits at compilation time.
 */
template <usize N, usize bit_size>
struct reversed_bits_8<N, bit_size> {
    static constexpr u8 value = reverse_bits_8(N, bit_size);
};


/**
 * @brief Evaluate the factorial of a number during compilation time
 */
template <usize N>
struct factorial {
    static constexpr f64 value = N * factorial<N - 1>::value;
};

/**
 * @brief Evaluate the factorial of a number during compilation time
 */
template <>
struct factorial<0> {
    static constexpr f64 value = 1;
};

template<usize N>
struct inv_factorial {
    static constexpr f64 value = (1.0 / factorial<N>::value);
};

/**
 * @brief Evaluate the factorial of a number during compilation time
 */
template <usize N>
struct factorialf {
    static constexpr f32 value = N * factorialf<N - 1>::value;
};

/**
 * @brief Evaluate the factorial of a number during compilation time
 */
template <>
struct factorialf<0> {
    static constexpr f32 value = 1;
};

/**
 * @brief Evaluate the factorial of a number during compilation time
 */
template <usize N>
struct ufactorial {
    static constexpr u64 value = N * ufactorial<N - 1>::value;
};

/**
 * @brief Evaluate the factorial of a number during compilation time
 */
template <>
struct ufactorial<0> {
    static constexpr u64 value = 1;
};

constexpr f64 pi = M_PI;
constexpr f64 pi_2 = 2 * pi;
constexpr f64 pi_by_2 = pi * .5;
constexpr f64 pi_by_2_3 = pi * 1.5;
constexpr f32 pif = M_PIf;

/**
 * @brief compute normalized value for angle [in range (0, 2pi)], for floating numbers
 */
constexpr inline f32 normf(f32 a) {
    u64 multiple = (a / (2 * pif));
    return a - 2 * pif * multiple;
}

/**
 * @brief compute normalized value for angle [in range (0, 2pi)], for double
 */
constexpr inline f64 norm(f64 a) {
    u64 multiple = (a / (pi_2));
    return a - pi_2 * multiple;
}

constexpr inline f64 compute(f64 a) {
    f64 ans = a, asq = a * a;
    f64 ta = asq * a;
    f64 asqp = asq * asq;
    if (ta < 1e-6) [[unlikely]] return ans;
    ans += (ta * (asq - 20.0)) * inv_factorial<5>::value;
    ta *= asqp;
    if (ta < 1e-3) return ans;
    ans += (ta * (asq - 72.0)) * inv_factorial<9>::value;
    ta *= asqp;
    if (ta < 1) return ans;
    ans += (ta * (asq - 156.0)) * inv_factorial<13>::value;
    ta *= asqp;
    if (ta < 1e3) return ans;
    ans += (ta * (asq - 272.0)) * inv_factorial<17>::value;
    ta *= asqp;
    if (ta < 1e6) [[likely]] return ans;
    ans += (ta * (asq - 420.0)) * inv_factorial<21>::value;
    return ans;
}

/**
 * @brief compute sine (can be computed in compile time)
 * @param a angle in radian
 */
constexpr f64 sin(f64 a) {
    a = norm(std::abs(a));
    bool v = a > pi;
    i8 sgn = v ? -1 : 1;
    a = v ? a - pi : a;
    f64 ans = a, asq = a * a;
    f64 ta = asq * a;
    f64 asqp = asq * asq;
    if (ta < 1e-6) [[unlikely]] return ans;
    ans += (ta * (asq - 20.0)) * inv_factorial<5>::value;
    ta *= asqp;
    if (ta < 1e-3) return ans;
    ans += (ta * (asq - 72.0)) * inv_factorial<9>::value;
    ta *= asqp;
    if (ta < 1) return ans;
    ans += (ta * (asq - 156.0)) * inv_factorial<13>::value;
    ta *= asqp;
    if (ta < 1e3) [[likely]] return ans;
    ans += (ta * (asq - 272.0)) * inv_factorial<17>::value;
    ta *= asqp;
    if (ta < 1e6) [[likely]] return ans;
    ans += (ta * (asq - 420.0)) * inv_factorial<21>::value;
    return ans * sgn;
}

/**
 * @brief compute cosine  (can be computed in compile time)
 * @param a angle in radian
 */
inline constexpr f64 cos(f64 a) {
    a = norm(std::abs(a));
    f64 sgn = a > pi_by_2 && a < pi_by_2_3 ? -1 : 1;
    return compute(a + pi_by_2) * sgn;
}

/**
 * @brief compute tangent  (can be computed in compile time)
 * @param a angle in radian
 */
inline constexpr f64 tan(f64 a) {
    return sin(a) / cos(a);
}

/**
 * @brief compute sine  (can be computed in compile time)
 * @param a angle in radian
 */
constexpr f32 sinf(f32 a) {
    a = normf(a);
    bool v = a > pif;
    i8 sgn = v ? -1 : 1;
    a = v ? a - pif : a;
    f32 ans = a, ta = a * a * a;
    f32 asq = a * a;
    f32 asqp = asq * asq;
    ans += (ta * (asq - 20.0)) / factorialf<5>::value;
    ta *= asqp;
    ans += (ta * (asq - 72.0f)) / factorialf<9>::value;
    ta *= asqp;
    ans += (ta * (asq - 156.0f)) / factorialf<13>::value;
    ta *= asqp;
    ans += (ta * (asq - 272.0f)) / factorialf<17>::value;
    return ans * sgn;
}

/**
 * @brief compute cosine (can be computed in compile time)
 * @param a angle in radian
 */
constexpr f32 cosf(f32 a) {
    a = normf(a);
    f32 ans = 1, ta = a * a;
    f32 asq = ta;
    f32 asqp = asq * asq;
    ans += (ta * (asq - 12.0f)) / factorialf<4>::value;
    ta *= asqp;
    ans += (ta * (asq - 56.0f)) / factorialf<8>::value;
    ta *= asqp;
    ans += (ta * (asq - 132.0f)) / factorialf<12>::value;
    ta *= asqp;
    ans += (ta * (asq - 240.0f)) / factorialf<16>::value;
    return ans;
}

/**
 * @brief compute tangent (can be computed in compile time)
 * @param a angle in radian
 */
inline constexpr f32 tanf(f32 a) {
    return sinf(a) / cosf(a);
}

constexpr f64 exp(f64 a) {
    f64 y = 0, ynext = 0;
    return 0;
}


static_assert(ufactorial<10>::value == 3628800);
static_assert(ufactorial<12>::value == 3628800 * 11 * 12);

}
#endif
