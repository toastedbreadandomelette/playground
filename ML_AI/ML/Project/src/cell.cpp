#include "cell.hpp"
#include <algorithm>
#include <thread>
#include <cmath>

#define TOTAL_THREADS 8

std::ostream &operator<<(std::ostream &op, const None &none) {
    op << "None";
    return op;
}

std::ostream &operator<<(std::ostream &op, const Cell &value) {
    std::visit([&op](const auto &x) { op << x; }, value);
    return op;
}
//----------------------- + operator ----------------------//

/**
 * @brief operator +
 */
Cell operator+(const Cell&first, const Cell&second) {
    switch(first.index()) {
        case 0:  return second + std::get<int64_t>(first);
        case 1:  return second + std::get<uint64_t>(first);
        case 2:  return second + std::get<double>(first);
        default: return Cell(None{});
    }
}

/**
 * @brief operator +
 */
Cell operator+(const Cell&first, const int64_t second) {
    switch(first.index()) {
        case 0:  return Cell(std::get<int64_t>(first) + second);
        case 1:  return Cell(std::get<uint64_t>(first) + second);
        case 2:  return Cell(std::get<double>(first) + second);
        default: return Cell(None{});
    }
}

/**
 * @brief operator +
 */
Cell operator+(const Cell&first, const uint64_t second) {
    switch(first.index()) {
        case 0:  return Cell(std::get<int64_t>(first) + second);
        case 1:  return Cell(std::get<uint64_t>(first) + second);
        case 2:  return Cell(std::get<double>(first) + second);
        default: return Cell(None{});
    }
}

/**
 * @brief operator +
 */
Cell operator+(const Cell&first, const double second) {
    switch(first.index()) {
        case 0:  return Cell(std::get<int64_t>(first) + second);
        case 1:  return Cell(std::get<uint64_t>(first) + second);
        case 2:  return Cell(std::get<double>(first) + second);
        default: return Cell(None{});
    }
}

/**
 * @brief operator +
 */
Cell operator+(const Cell&first, const std::string&second) {
    switch(first.index()) {
        case 3:  return Cell(std::get<std::string>(first) + second);
        default: return Cell(None{});
    }
}

//----------------------- - operator ----------------------//

/**
 * @brief operator -
 */
Cell operator-(const Cell&first, const int64_t second) {
    switch(first.index()) {
        case 0:  return Cell(std::get<int64_t>(first) - second);
        case 1:  return Cell(std::get<uint64_t>(first) - second);
        case 2:  return Cell(std::get<double>(first) - second);
        default: return Cell(None{});
    }
}

/**
 * @brief operator -
 */
Cell operator-(const Cell&first, const uint64_t second) {
    switch(first.index()) {
        case 0:  return Cell(std::get<int64_t>(first) - second);
        case 1:  return Cell(std::get<uint64_t>(first) - second);
        case 2:  return Cell(std::get<double>(first) - second);
        default: return Cell(None{});
    }
}

/**
 * @brief operator -
 */
Cell operator-(const Cell&first, const double second) {
    switch(first.index()) {
        case 0:  return Cell(std::get<int64_t>(first) - second);
        case 1:  return Cell(std::get<uint64_t>(first) - second);
        case 2:  return Cell(std::get<double>(first) - second);
        default: return Cell(None{});
    }
}

/**
 * @brief operator *
 */
Cell operator*(const Cell&first, const int64_t second) {
    switch(first.index()) {
        case 0:  return Cell(std::get<int64_t>(first) * second);
        case 1:  return Cell(std::get<uint64_t>(first) * second);
        case 2:  return Cell(std::get<double>(first) * second);
        default: return Cell(None{});
    }
}

/**
 * @brief operator *
 */
Cell operator*(const Cell&first, const uint64_t second) {
    switch(first.index()) {
        case 0:  return Cell(std::get<int64_t>(first) * second);
        case 1:  return Cell(std::get<uint64_t>(first) * second);
        case 2:  return Cell(std::get<double>(first) * second);
        default: return Cell(None{});
    }
}

/**
 * @brief operator *
 */
Cell operator*(const Cell&first, const double second) {
    switch(first.index()) {
        case 0:  return Cell(std::get<int64_t>(first) * second);
        case 1:  return Cell(std::get<uint64_t>(first) * second);
        case 2:  return Cell(std::get<double>(first) * second);
        default: return Cell(None{});
    }
}

/**
 * @brief operator /
 */
Cell operator/(const Cell&first, const int64_t second) {
    switch(first.index()) {
        case 0:  return Cell(std::get<int64_t>(first) / second);
        case 1:  return Cell(std::get<uint64_t>(first) / second);
        case 2:  return Cell(std::get<double>(first) / second);
        default: return Cell(None{});
    }
}

/**
 * @brief operator /
 */
Cell operator/(const Cell&first, const uint64_t second) {
    switch(first.index()) {
        case 0:  return Cell(std::get<int64_t>(first) / second);
        case 1:  return Cell(std::get<uint64_t>(first) / second);
        case 2:  return Cell(std::get<double>(first) / second);
        default: return Cell(None{});
    }
}

/**
 * @brief operator /
 */
Cell operator/(const Cell&first, const double second) {
    switch(first.index()) {
        case 0:  return Cell(std::get<int64_t>(first) / second);
        case 1:  return Cell(std::get<uint64_t>(first) / second);
        case 2:  return Cell(std::get<double>(first) / second);
        default: return Cell(None{});
    }
}

