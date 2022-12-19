#include "column_view.hpp"

Cell ColView::aggregate(const std::function<Cell(const Cell&, const Cell&)>&func, const Cell &init) {
    Cell start = init;
    for (auto &Table: table->table) {
        start = func(start, Table[col_index]);
    }

    return start;
}

Cell ColView::aggregate_mt(const std::function<Cell(const Cell&, const Cell&)>&func, const Cell &init) {
    std::vector<Cell> init_array(ColView::total_threads, init);
    std::vector<std::thread> thread_pool;

    auto aggregate_internal = [func, &init, &init_array, this] (const size_t start, const uint8_t thread_count) {
        for (size_t i = start; i < this->get_size(); i += thread_count) {
            init_array[start] = func(init_array[start], this->operator[](i));
        }
    };

    for (int i = 0; i < ColView::total_threads; ++i) {
        thread_pool.emplace_back(std::thread(aggregate_internal, i, ColView::total_threads));
    }

    for (auto &thrd: thread_pool) {
        thrd.join();
    }

    Cell final = init;
    for (int i = 0; i < ColView::total_threads; ++i) {
        final = func(final, init_array[i]);
    }

    return final;
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

    const size_t block = __size / ColView::total_threads;
    const uint8_t t_but_one = ColView::total_threads - 1;

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

    const size_t block = __size / ColView::total_threads;
    const uint8_t t_but_one = ColView::total_threads - 1;

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

    const size_t block = __size / ColView::total_threads;
    const uint8_t t_but_one = ColView::total_threads - 1;

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

    const size_t block = __size / ColView::total_threads;
    const uint8_t t_but_one = ColView::total_threads - 1;

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
// template MdDynArray<std::string> ColView::map(const std::function<std::string(const Cell&, const size_t, const ColView&)>&);

template MdDynArray<double> ColView::map();
template MdDynArray<int64_t> ColView::map();
template MdDynArray<uint64_t> ColView::map();
// template MdDynArray<std::string> ColView::map();

template MdDynArray<double> ColView::map_mt(const std::function<double(const Cell&, const size_t, const ColView&)>&);
template MdDynArray<int64_t> ColView::map_mt(const std::function<int64_t(const Cell&, const size_t, const ColView&)>&);
template MdDynArray<uint64_t> ColView::map_mt(const std::function<uint64_t(const Cell&, const size_t, const ColView&)>&);
// template MdDynArray<std::string> ColView::map_mt(const std::function<std::string(const Cell&, const size_t, const ColView&)>&);

template MdDynArray<double> ColView::map_mt();
template MdDynArray<int64_t> ColView::map_mt();
template MdDynArray<uint64_t> ColView::map_mt();
// template MdDynArray<std::string> ColView::map_mt();

template MdStaticArray<double> ColView::st_map(const std::function<double(const Cell&, const size_t, const ColView&)>&);
template MdStaticArray<int64_t> ColView::st_map(const std::function<int64_t(const Cell&, const size_t, const ColView&)>&);
template MdStaticArray<uint64_t> ColView::st_map(const std::function<uint64_t(const Cell&, const size_t, const ColView&)>&);
// template MdStaticArray<std::string> ColView::st_map(const std::function<std::string(const Cell&, const size_t, const ColView&)>&);

template MdStaticArray<double> ColView::st_map();
template MdStaticArray<int64_t> ColView::st_map();
template MdStaticArray<uint64_t> ColView::st_map();
// template MdStaticArray<std::string> ColView::st_map();

template MdStaticArray<double> ColView::st_map_mt(const std::function<double(const Cell&, const size_t, const ColView&)>&);
template MdStaticArray<int64_t> ColView::st_map_mt(const std::function<int64_t(const Cell&, const size_t, const ColView&)>&);
template MdStaticArray<uint64_t> ColView::st_map_mt(const std::function<uint64_t(const Cell&, const size_t, const ColView&)>&);
// template MdStaticArray<std::string> ColView::st_map_mt(const std::function<std::string(const Cell&, const size_t, const ColView&)>&);

template MdStaticArray<double> ColView::st_map_mt();
template MdStaticArray<int64_t> ColView::st_map_mt();
template MdStaticArray<uint64_t> ColView::st_map_mt();
// template MdStaticArray<std::string> ColView::st_map_mt();
