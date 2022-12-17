#include "cell.hpp"
#include <algorithm>
#include <thread>
#include <cmath>

#define TOTAL_THREADS 8

namespace Table {

ColView Table::operator[](const std::string &val) {
    uint16_t index = std::find(header.begin(), header.end(), val) - header.begin();
    return { header[index].c_str(), index, this };
}

std::ostream &operator<<(std::ostream &op, const None &none) {
    op << "None";
    return op;
}

std::ostream &operator<<(std::ostream &op, const Cell &value) {
    std::visit([&op](const auto &x) { op << x; }, value);
    return op;
}

Cell ColView::aggregate(const std::function<Cell(const Cell&, const Cell&)>&func, const Cell &init) {
    Cell start = init;
    // todo: make it multi-threaded
    for (auto &Table: table->table) {
        start = func(start, Table[col_index]);
    }

    return start;
}

Cell ColView::aggregate_mt(const std::function<Cell(const Cell&, const Cell&)>&func, const Cell &init) {
    std::vector<Cell> init_array(TOTAL_THREADS, init);
    std::vector<std::thread> thread_pool;

    auto aggregate_internal = [func, &init, &init_array, this] (const size_t start, const uint8_t thread_count) {
        for (size_t i = start; i < this->get_size(); i += thread_count) {
            init_array[start] = func(init_array[start], this->operator[](i));
        }
    };

    for (int i = 0; i < TOTAL_THREADS; ++i) {
        thread_pool.emplace_back(std::thread(aggregate_internal, i, TOTAL_THREADS));
    }

    for (auto &thrd: thread_pool) {
        thrd.join();
    }

    Cell final = init;
    for (int i = 0; i < TOTAL_THREADS; ++i) {
        final = func(final, init_array[i]);
    }

    return final;
}

template <typename _T>
_T ColView::get_values(const Cell &__value) {
    switch(__value.index()) {
        case 0:  return std::get<int64_t>(__value);
        case 1:  return std::get<uint64_t>(__value);
        case 2:  return std::get<double>(__value);
        default:  return 0;
    }
}

template <typename _T>
MdDynArray<_T> ColView::map(const std::function<_T(const Cell&, const size_t, const ColView&)>&func) {
    const size_t __size = this->get_size();
    MdDynArray<_T> value(__size);
    const ColView &__ptr = *this;
    for (size_t ptr = 0; ptr < __size; ++ptr) {
        value[ptr] = func(this->operator[](ptr), ptr, __ptr);
    }

    return value;
}

template <typename _T>
MdDynArray<_T> ColView::map_mt(const std::function<_T(const Cell&, const size_t, const ColView&)>&func) {
    const size_t __size = this->get_size();
    MdDynArray<_T> value(__size);
    std::vector<std::thread> thread_pool;

    auto mp_internal = [&func, this, &value](const size_t start, const size_t end) {
        const ColView &__ptr = *this;
        for (size_t ptr = start; ptr < end; ++ptr) {
            value[ptr] = func(this->operator[](ptr), ptr, __ptr);
        }
    };

    const size_t block = __size / TOTAL_THREADS;
    const uint8_t t_but_one = TOTAL_THREADS - 1;

    for (int i = 0; i < t_but_one; ++i) {
        thread_pool.emplace_back(std::thread(mp_internal, block * i, block * (i + 1)));
    }

    thread_pool.emplace_back(std::thread(mp_internal, block * t_but_one, __size));

    for (auto &thrd: thread_pool) {
        thrd.join();
    }

    return value;
}

template <typename _T>
MdDynArray<_T> ColView::map() {
    const size_t __size = this->get_size();
    MdDynArray<_T> value(__size);
    const ColView &__ptr = *this;
    for (size_t ptr = 0; ptr < __size; ++ptr) {
        value[ptr] = get_values<_T>(this->operator[](ptr));
    }

    return value;
}


template <typename _T>
MdDynArray<_T> ColView::map_mt() {
    const size_t __size = this->get_size();
    MdDynArray<_T> value(__size);
    std::vector<std::thread> thread_pool;

    auto mp_internal = [this, &value](const size_t start, const size_t end) {
        const ColView &__ptr = *this;
        for (size_t ptr = start; ptr < end; ++ptr) {
            value[ptr] = get_values<_T>(this->operator[](ptr));
        }
    };

    const size_t block = __size / TOTAL_THREADS;
    const uint8_t t_but_one = TOTAL_THREADS - 1;

    for (int i = 0; i < t_but_one; ++i) {
        thread_pool.emplace_back(std::thread(mp_internal, block * i, block * (i + 1)));
    }

    thread_pool.emplace_back(std::thread(mp_internal, block * t_but_one, __size));

    for (auto &thrd: thread_pool) {
        thrd.join();
    }

    return value;
}


template <typename _T>
MdStaticArray<_T> ColView::st_map(const std::function<_T(const Cell&, const size_t, const ColView&)>&func) {
    const size_t __size = this->get_size();
    MdStaticArray<_T> value(__size);
    const ColView &__ptr = *this;
    for (size_t ptr = 0; ptr < __size; ++ptr) {
        value[ptr] = func(this->operator[](ptr), ptr, __ptr);
    }

    return value;
}

template <typename _T>
MdStaticArray<_T> ColView::st_map_mt(const std::function<_T(const Cell&, const size_t, const ColView&)>&func) {
    const size_t __size = this->get_size();
    MdStaticArray<_T> value(__size);
    std::vector<std::thread> thread_pool;

    auto mp_internal = [&func, this, &value](const size_t start, const size_t end) {
        const ColView &__ptr = *this;
        for (size_t ptr = start; ptr < end; ++ptr) {
            value[ptr] = func(this->operator[](ptr), ptr, __ptr);
        }
    };

    const size_t block = __size / TOTAL_THREADS;
    const uint8_t t_but_one = TOTAL_THREADS - 1;

    for (int i = 0; i < t_but_one; ++i) {
        thread_pool.emplace_back(std::thread(mp_internal, block * i, block * (i + 1)));
    }

    thread_pool.emplace_back(std::thread(mp_internal, block * t_but_one, __size));

    for (auto &thrd: thread_pool) {
        thrd.join();
    }

    return value;
}

template <typename _T>
MdStaticArray<_T> ColView::st_map() {
    const size_t __size = this->get_size();
    MdStaticArray<_T> value(__size);
    const ColView &__ptr = *this;
    for (size_t ptr = 0; ptr < __size; ++ptr) {
        value[ptr] = get_values<_T>(this->operator[](ptr));
    }

    return value;
}


template <typename _T>
MdStaticArray<_T> ColView::st_map_mt() {
    const size_t __size = this->get_size();
    MdStaticArray<_T> value(__size);
    std::vector<std::thread> thread_pool;

    auto mp_internal = [this, &value](const size_t start, const size_t end) {
        const ColView &__ptr = *this;
        for (size_t ptr = start; ptr < end; ++ptr) {
            value[ptr] = get_values<_T>(this->operator[](ptr));
        }
    };

    const size_t block = __size / TOTAL_THREADS;
    const uint8_t t_but_one = TOTAL_THREADS - 1;

    for (int i = 0; i < t_but_one; ++i) {
        thread_pool.emplace_back(std::thread(mp_internal, block * i, block * (i + 1)));
    }

    thread_pool.emplace_back(std::thread(mp_internal, block * t_but_one, __size));

    for (auto &thrd: thread_pool) {
        thrd.join();
    }

    return value;
}

template MdDynArray<double> ColView::map(const std::function<double(const Cell&, const size_t, const ColView&)>&);
template MdDynArray<int64_t> ColView::map(const std::function<int64_t(const Cell&, const size_t, const ColView&)>&);
template MdDynArray<uint64_t> ColView::map(const std::function<uint64_t(const Cell&, const size_t, const ColView&)>&);

template MdDynArray<double> ColView::map();
template MdDynArray<int64_t> ColView::map();
template MdDynArray<uint64_t> ColView::map();

template MdDynArray<double> ColView::map_mt(const std::function<double(const Cell&, const size_t, const ColView&)>&);
template MdDynArray<int64_t> ColView::map_mt(const std::function<int64_t(const Cell&, const size_t, const ColView&)>&);
template MdDynArray<uint64_t> ColView::map_mt(const std::function<uint64_t(const Cell&, const size_t, const ColView&)>&);

template MdDynArray<double> ColView::map_mt();
template MdDynArray<int64_t> ColView::map_mt();
template MdDynArray<uint64_t> ColView::map_mt();

template MdStaticArray<double> ColView::st_map(const std::function<double(const Cell&, const size_t, const ColView&)>&);
template MdStaticArray<int64_t> ColView::st_map(const std::function<int64_t(const Cell&, const size_t, const ColView&)>&);
template MdStaticArray<uint64_t> ColView::st_map(const std::function<uint64_t(const Cell&, const size_t, const ColView&)>&);

template MdStaticArray<double> ColView::st_map();
template MdStaticArray<int64_t> ColView::st_map();
template MdStaticArray<uint64_t> ColView::st_map();

template MdStaticArray<double> ColView::st_map_mt(const std::function<double(const Cell&, const size_t, const ColView&)>&);
template MdStaticArray<int64_t> ColView::st_map_mt(const std::function<int64_t(const Cell&, const size_t, const ColView&)>&);
template MdStaticArray<uint64_t> ColView::st_map_mt(const std::function<uint64_t(const Cell&, const size_t, const ColView&)>&);

template MdStaticArray<double> ColView::st_map_mt();
template MdStaticArray<int64_t> ColView::st_map_mt();
template MdStaticArray<uint64_t> ColView::st_map_mt();

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

} /// namespace Table
