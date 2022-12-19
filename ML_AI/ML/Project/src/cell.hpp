#pragma once
#ifndef _CELL_HPP_
#define _CELL_HPP_
#include <variant>
#include <iostream>
#include <functional>
#include "md_dyn_array.hpp"
#include "md_static_array.hpp"
#include <iomanip>
#include <algorithm>

// template <typename _T>
// class MdDynArray;

typedef struct None {} None;

/**
 * @brief Output operator for displaying None Type
 * @param op Output stream object
 * @param none None object
 * @return op
 */
std::ostream &operator<<(std::ostream &op, const None &none);

typedef struct Err { uint8_t err; } Err;

/// @brief Variant of a type (only works on C++17 and above, use flag `-std=c++17`)
/// @todo perform basic add, subtract operation
typedef std::variant<int64_t, uint64_t, double, std::string, None> Cell;

/**
 * @brief Output operator for displaying Variant type
 * @param op Output stream object
 * @param value value of variant type
 * @return op
 */
std::ostream &operator<<(std::ostream &op, const Cell &value);

// Error enum to display the error messages
enum Error {
    END,
    UNEXPECTED_TOKEN,
    INVALID_OPERATION
};

// Enum for marking data type for each column
enum DataType {
    INTEGER = 0b00001,
    UNSIGNED_INTEGER = 0x00010,
    REAL_NUMBER = 0x00100,
    STRING = 0x01000,
    NONE = 0x10000
};

struct ColView;
struct Table;

#endif
