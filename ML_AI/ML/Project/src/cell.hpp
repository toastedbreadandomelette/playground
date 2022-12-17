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

/**
 * @namespace Table
 */
namespace Table {

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

/**
 * @brief Add operator overloading in C++
 * @param first first Cell
 * @param second second cell
 * @returns Addition of these cells
 */
Cell operator+(const Cell&first, const Cell&second);
Cell operator+(const Cell&first, const int64_t second);
Cell operator+(const Cell&first, const uint64_t second);
Cell operator+(const Cell&first, const double second);

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

/**
 * @brief Table struct for easier data handling
 */
struct Table {
public:
    // Header for the table
    std::vector<std::string> header;
    // Actual table
    std::vector<Cell*> table;
    // Column type marking
    std::vector<uint8_t> type;
    // Table size
    uint16_t col_size;
    // Display info
    uint8_t first_last_display = 5;

    // Get size
    inline size_t get_size() const {
        return table.size();
    }

    // get col size
    inline size_t get_col_size() const {
        return col_size;
    }

    /**
     * @brief Get header column
     * @param val string
     * @returns Column View of the Table
     * @note Changing views within ColView results in changes in 
     * main table (to perform, clone)
     */
    ColView operator[](const std::string &val);

    friend std::ostream &operator <<(std::ostream &op, const Table &table) {
        for (auto &header_name: table.header) {
            op << std::setw(15) << header_name;
        }
        op << "\n";
        if (table.table.size() > table.first_last_display * 2) {
            for (int i = 0; i < table.first_last_display; ++i) {
                for (auto j = 0; j < table.col_size; ++j) {
                    op << std::setw(15) << table.table[i][j];
                }
                op << '\n';
            }
            op << '\n';
            for (int i = 0; i < 3; ++i) {
                for (auto j = 0; j < table.col_size; ++j) {
                    op << std::setw(15) << "...." ;
                }
                op << '\n';
            }
            op << '\n';
            for (int i = table.table.size() - table.first_last_display; i < table.table.size(); ++i) {
                for (auto j = 0; j < table.col_size; ++j) {
                    op << std::setw(15) << table.table[i][j];
                }
                op << '\n';
            }
            op << '\n';
        } else {
            for (int i = 0; i < table.table.size(); ++i) {
                for (auto j = 0; j < table.col_size; ++j) {
                    op << std::setw(15) << table.table[i][j];
                }
                op << '\n';
            }
        }
        return op;
    }
};

struct ColView {
public:
    // Pointer of the header name
    const char *name;
    // Index of the string (easy to identify from the table)
    uint16_t col_index;
    // Reference of the table ()
    Table *table;

    inline size_t get_size() const {
        return table->get_size();
    }
    /**
     * @brief return index of the value
     */
    inline Cell &operator[](const size_t index) {
        return table->table[index][col_index];
    }

    /**
     * @brief Handle delete for ColView
     */
    ~ColView() {
        name = nullptr;
        table = nullptr;
    }

    /**
     * @brief Aggregates the values by combining previous values defined by the function
     * @param function that aggregates previous value and current value
     * @param init initalizing value before combining
     * @returns Result
     */
    Cell aggregate(const std::function<Cell(const Cell&, const Cell&)>&func, const Cell &init);
    
    /**
     * @brief Aggregates (with multi-threading) the values by combining previous defined by the function.
     * @param function that aggregates previous value and current value
     * @param init initalizing value before combining
     * @returns Result
     */
    Cell aggregate_mt(const std::function<Cell(const Cell&, const Cell&)>&func, const Cell &init);

    /**
     * @brief Maps the values and returns the new array.
     */
    template <typename _T>
    MdDynArray<_T> map(const std::function<_T(const Cell&, const size_t, const ColView&)>&func);

    /**
     * @brief Maps the values and returns the new array.
     * Uses multi-threading
     */
    template <typename _T>
    MdDynArray<_T> map_mt(const std::function<_T(const Cell&, const size_t, const ColView&)>&func);

    /**
     * @brief Maps the values and returns the new array.
     */
    template <typename _T>
    MdStaticArray<_T> map(const std::function<_T(const Cell&, const size_t, const ColView&)>&func);

    /**
     * @brief Maps the values and returns the new array.
     * Uses multi-threading
     */
    template <typename _T>
    MdStaticArray<_T> map_mt(const std::function<_T(const Cell&, const size_t, const ColView&)>&func);

    template <typename _T>
    _T get_values(const Cell &);

    /**
     * @brief Maps the values and returns the new array.
     */
    template <typename _T>
    MdDynArray<_T> map();

    /**
     * @brief Maps the values and returns the new array.
     */
    template <typename _T>
    MdDynArray<_T> map_mt();
    
    /**
     * @brief Maps the values and returns the new array.
     */
    template <typename _T>
    MdStaticArray<_T> map();

    /**
     * @brief Maps the values and returns the new array.
     */
    template <typename _T>
    MdStaticArray<_T> map_mt();
};


} // namespace table

#endif
