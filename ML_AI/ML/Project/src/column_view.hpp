#pragma once
#ifndef _COLUMN_VIEW_HPP_
#define _COLUMN_VIEW_HPP_

#include "cell.hpp"
#include "tables.hpp"
#include "md_static_array.hpp"
#include "md_dyn_array.hpp"

struct Table;

struct ColView {
public:
    // Pointer of the header name
    const char *name;
    // Index of the string (easy to identify from the table)
    uint16_t col_index;
    // Reference of the table ()
    Table *table;

    static const int total_threads = 8;

    /**
     * @brief returns the size of array
     */
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
    MdStaticArray<_T> st_map(const std::function<_T(const Cell&, const size_t, const ColView&)>&func);

    /**
     * @brief Maps the values and returns the new array.
     * Uses multi-threading
     */
    template <typename _T>
    MdStaticArray<_T> st_map_mt(const std::function<_T(const Cell&, const size_t, const ColView&)>&func);

    /**
     * @brief get values of a cell
     */
    template <typename _T>
    friend _T get_values(const Cell &);

    // /**
    //  * @brief get values of a cell
    //  */
    // friend std::string get_values(const Cell &);

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
    MdStaticArray<_T> st_map();

    /**
     * @brief Maps the values and returns the new array.
     */
    template <typename _T>
    MdStaticArray<_T> st_map_mt();

};

template <typename _T, class = typename std::enable_if<std::is_arithmetic<_T>::value>::type>
_T get_values(const Cell &__value) {
    switch(__value.index()) {
        case 0:   return std::get<int64_t>(__value);
        case 1:   return std::get<uint64_t>(__value);
        case 2:   return std::get<double>(__value);
        default:  return 0;
    }
}

template <typename _T, class = typename std::enable_if<std::is_same<_T, std::string>::value>::type>
std::string get_values(const Cell &__value) {
    switch(__value.index()) {
        case 3 :  return std::get<std::string>(__value);
        default:  return 0;
    }
}

#endif
