#pragma once
#ifndef _MD_DYN_ARRAY_HPP_
#define _MD_DYN_ARRAY_HPP_
#include <vector>
#include <thread>
#include <iostream>
#include <algorithm>

#define EN_IF(C) std::enable_if<C>::value
#define IS_ARITH(E) std::is_arithmetic<E>::value

#define OP_INTERNAL_MACRO(func) \
if constexpr (sizeof(_T1) > sizeof(_T)) { \
    if constexpr (std::is_floating_point<_T>::value && sizeof(_T1) == 4) { \
        return func(__other, (float)0); \
    } else if constexpr (std::is_floating_point<_T>::value && sizeof(_T1) == 8) { \
        return func(__other, (double)0); \
    } else { \
        return func(__other, (_T1)0); \
    } \
} else { \
    if constexpr (std::is_floating_point<_T>::value && sizeof(_T1) == 4) { \
        return func(__other, (double)0); \
    } else if constexpr (std::is_floating_point<_T>::value && sizeof(_T1) == 8) { \
        return func(__other, (double)0); \
    } else if constexpr (std::is_floating_point<_T>::value && sizeof(_T1) < 4) { \
        return func(__other, (float)0); \
    } else { \
        return func(__other, (_T)0); \
    } \
} \

static size_t threshold_size = 10000000;
static uint8_t thread_count = 16;

template <typename _T>
class MdDynArray {
public:
    std::vector<_T> __array;
    std::vector<size_t> shape;

    static void set_thread_count(const uint8_t value);
    
    static void set_threshold_size(const size_t size);

    MdDynArray(const size_t __size): __array(std::vector<_T>(__size)) {}

    MdDynArray(const size_t __size, const _T&value): __array(std::vector<_T>(__size, value)) {}

    MdDynArray(): __array(std::vector<_T>()) {}

    MdDynArray(const std::vector<_T>&__other, const std::vector<size_t> &shape): 
        __array(std::vector<_T>(__other)),
        shape(shape) {}
    
    MdDynArray(const std::vector<_T>&__other): 
        __array(std::vector<_T>(__other)) { shape.push_back(__other.size()); }

    MdDynArray(const MdDynArray& __other): __array(std::vector<_T>()) {
        __array = __other.__array;
        shape = __other.shape;
    }

    /**
     * @brief Assignment operator (direct vector assignment)
     */
    MdDynArray &operator=(const std::vector<_T>&__other) {
        __array = std::vector<_T>(__other);
        shape.push_back(__array.size());
        return *this;
    }

    /**
     * @brief Assignment operator 
     */
    MdDynArray &operator=(const MdDynArray& __other) {
        __array = __other.__array;
        return *this;
    }

    inline void append(const _T &value) const {
        __array.push_back(value);
    }

    inline void emplace(const _T &value) const {
        __array.emplace_back(value);
    }

    /**
     * @brief Add function, currently using threads
     * @param __other other array (might be of different type)
     * @returns new array of current type
     */
    template <typename _T1, typename _T2>
    MdDynArray<_T2> __add_internal(const MdDynArray<_T1> &__other, const _T2) const;

    /**
     * @brief Add function, currently using threads
     * @param __other other integer (might be of different type)
     * @returns new array of current type
    */
    template <typename _T1, typename _T2>
    MdDynArray<_T2> __add_iinternal(const _T1 &__other, const _T2) const;

    /**
     * @brief Subtract function, currently using threads
     * @param __other other array (might be of different type)
     * @returns new array of current type
     */
    template <typename _T1, typename _T2>
    MdDynArray<_T2> __sub_internal(const MdDynArray<_T1> &, const _T2) const;

    /**
     * @brief Subtract function, currently using threads
     * @param __other other integer (might be of different type)
     * @returns new array of current type
    */
    template <typename _T1, typename _T2>
    MdDynArray<_T2> __sub_iinternal(const _T1 &, const _T2) const;

    /**
     * @brief Subtract function, currently using threads
     * @param __other other integer (might be of different type)
     * @returns new array of current type
    */
    template <typename _T1, typename _T2>
    MdDynArray<_T2> __sub_iointernal(const _T1 &, const _T2) const;

    /**
     * @brief Multiplication function, currently using threads
     * @param __other other array (might be of different type)
     * @returns new array of current type
     */
    template <typename _T1, typename _T2>
    MdDynArray<_T2> __mul_internal(const MdDynArray<_T1> &, const _T2) const;

    /**
     * @brief Multiplication function, currently using threads
     * @param __other other integer (might be of different type)
     * @returns new array of current type
    */
    template <typename _T1, typename _T2>
    MdDynArray<_T2> __mul_iinternal(const _T1 &__other, const _T2) const;

    /**
     * @brief Division function, currently using threads
     * @param __other other array (might be of different type)
     * @returns new array of current type
     */
    template <typename _T1, typename _T2>
    MdDynArray<_T2> __div_internal(const MdDynArray<_T1> &, const _T2) const;

    /**
     * @brief Division function, currently using threads
     * @param __other other integer (might be of different type)
     * @returns new array of current type
     */
    template <typename _T1, typename _T2>
    MdDynArray<_T2> __div_iinternal(const _T1 &__other, const _T2) const;

    /**
     * @brief Division function, currently using threads
     * @param __other other integer (might be of different type)
     * @returns new array of current type
     */
    template <typename _T1, typename _T2>
    MdDynArray<_T2> __div_iointernal(const _T1 &__other, const _T2) const;

    /**
     * @brief Division function, currently using threads
     * @param __other other array (might be of different type)
     * @returns new array of current type
     */
    template <typename _T1, typename _T2>
    MdDynArray<_T2> __mod_internal(const MdDynArray<_T1> &, const _T2) const;

    /**
     * @brief Division function, currently using threads
     * @param __other other integer (might be of different type)
     * @returns new array of current type
     */
    template <typename _T1, typename _T2>
    MdDynArray<_T2> __mod_iinternal(const _T1 &__other, const _T2) const;

    /**
     * @brief Division function, currently using threads
     * @param __other other integer (might be of different type)
     * @returns new array of current type
     */
    template <typename _T1, typename _T2>
    MdDynArray<_T2> __mod_iointernal(const _T1 &__other, const _T2) const;

    /**
     * @brief Add to self, using multi-threading
     * @param __other other vector to add
     * @returns new array
     */
    template <typename _T1>
    void __add_self_internal(const MdDynArray<_T1> &__other);

    /**
     * @brief Subtract to self, using multi-threading
     * @param __other other vector to add
     * @returns new array
     */
    template <typename _T1>
    void __sub_self_internal(const MdDynArray<_T1> &__other);

    /**
     * @brief Multiply to self, using multi-threading
     * @param __other other vector to add
     * @returns new array
     */
    template <typename _T1>
    void __mul_self_internal(const MdDynArray<_T1> &__other);

    /**
     * @brief Multiply to self, using multi-threading
     * @param __other other vector to add
     * @returns new array
     */
    template <typename _T1>
    void __div_self_internal(const MdDynArray<_T1> &__other);

    /**
     * @brief Multiply to self, using multi-threading
     * @param __other other vector to add
     * @returns new array
     */
    template <typename _T1>
    void __mod_self_internal(const MdDynArray<_T1> &__other);

    /**
     * @brief Add to self, using multi-threading
     * @param __other other vector to add
     * @returns new array
     */
    template <typename _T1>
    void __add_self_iinternal(const _T1 &__other);

    /**
     * @brief Subtract to self, using multi-threading
     * @param __other other vector to add
     * @returns new array
     */
    template <typename _T1>
    void __sub_self_iinternal(const _T1 &__other);

    /**
     * @brief Multiply to self, using multi-threading
     * @param __other other vector to add
     * @returns new array
     */
    template <typename _T1>
    void __mul_self_iinternal(const _T1 &__other);

    /**
     * @brief Multiply to self, using multi-threading
     * @param __other other vector to add
     * @returns new array
     */
    template <typename _T1>
    void __div_self_iinternal(const _T1 &__other);
    
    /**
     * @brief Multiply to self, using multi-threading
     * @param __other other vector to add
     * @returns new array
     */
    template <typename _T1>
    void __mod_self_iinternal(const _T1 &__other);

    template <typename _T1>
    MdDynArray<bool> __comp_eq_internal(const MdDynArray<_T1> &__other) {
        // assert that sizes are equal
        const size_t size = __array.size();
        MdDynArray<bool> result(size);
        if (MdDynArray::thread_count == 1 || __array.size() <= threshold_size) {
            for (size_t index = 0; index < get_size(); ++index) {
                result.__array[index] = __array[index] == __other.__array[index];
            }
        } else {
            std::vector<std::thread> st;
            st.reserve(MdDynArray::thread_count);
            auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
                for (size_t index = start; index < end; ++index) {
                    result.__array[index] = __array[index] == __other.__array[index];
                }
            };

            const size_t block = size / thread_count;
            const uint8_t thread_but_one = thread_count - 1;
            for (int i = 0; i < thread_but_one; ++i) {
                st.emplace_back(std::thread(_add_int, block * i, block * (i + 1)));
            }

            st.emplace_back(std::thread(_add_int, block * thread_but_one, size));

            for (auto &th: st) {
                th.join();
            }
        }
        return result;
    }

    template <typename _T1>
    MdDynArray<bool> __comp_g_internal(const MdDynArray<_T1> &__other) {
        // assert that sizes are equal
        const size_t size = __array.size();
        MdDynArray<bool> result(size);
        if (MdDynArray::thread_count == 1 || __array.size() <= threshold_size) {
            for (size_t index = 0; index < get_size(); ++index) {
                result.__array[index] = __array[index] > __other.__array[index];
            }
        } else {
            std::vector<std::thread> st;
            st.reserve(MdDynArray::thread_count);
            auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
                for (size_t index = start; index < end; ++index) {
                    result.__array[index] = __array[index] > __other.__array[index];
                }
            };

            const size_t block = size / thread_count;
            const uint8_t thread_but_one = thread_count - 1;
            for (int i = 0; i < thread_but_one; ++i) {
                st.emplace_back(std::thread(_add_int, block * i, block * (i + 1)));
            }

            st.emplace_back(std::thread(_add_int, block * thread_but_one, size));

            for (auto &th: st) {
                th.join();
            }
        }
        return result;
    }

    template <typename _T1>
    MdDynArray<bool> __comp_geq_internal(const MdDynArray<_T1> &__other) {
        // assert that sizes are equal
        const size_t size = __array.size();
        MdDynArray<bool> result(size);
        if (MdDynArray::thread_count == 1 || __array.size() <= threshold_size) {
            for (size_t index = 0; index < get_size(); ++index) {
                result.__array[index] = __array[index] >= __other.__array[index];
            }
        } else {
            std::vector<std::thread> st;
            st.reserve(MdDynArray::thread_count);
            auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
                for (size_t index = start; index < end; ++index) {
                    result.__array[index] = __array[index] >= __other.__array[index];
                }
            };

            const size_t block = size / thread_count;
            const uint8_t thread_but_one = thread_count - 1;
            for (int i = 0; i < thread_but_one; ++i) {
                st.emplace_back(std::thread(_add_int, block * i, block * (i + 1)));
            }

            st.emplace_back(std::thread(_add_int, block * thread_but_one, size));

            for (auto &th: st) {
                th.join();
            }
        }
        return result;
    }

    template <typename _T1>
    MdDynArray<bool> __comp_l_internal(const MdDynArray<_T1> &__other) {
        // assert that sizes are equal
        const size_t size = __array.size();
        MdDynArray<bool> result(size);
        if (MdDynArray::thread_count == 1 || __array.size() <= threshold_size) {
            for (size_t index = 0; index < get_size(); ++index) {
                result.__array[index] = __array[index] < __other.__array[index];
            }
        } else {
            std::vector<std::thread> st;
            st.reserve(MdDynArray::thread_count);
            auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
                for (size_t index = start; index < end; ++index) {
                    result.__array[index] = __array[index] < __other.__array[index];
                }
            };

            const size_t block = size / thread_count;
            const uint8_t thread_but_one = thread_count - 1;
            for (int i = 0; i < thread_but_one; ++i) {
                st.emplace_back(std::thread(_add_int, block * i, block * (i + 1)));
            }

            st.emplace_back(std::thread(_add_int, block * thread_but_one, size));

            for (auto &th: st) {
                th.join();
            }
        }
        return result;
    }

    template <typename _T1>
    MdDynArray<bool> __comp_leq_internal(const MdDynArray<_T1> &__other) {
        // assert that sizes are equal
        const size_t size = __array.size();
        MdDynArray<bool> result(size);
        if (MdDynArray::thread_count == 1 || __array.size() <= threshold_size) {
            for (size_t index = 0; index < get_size(); ++index) {
                result.__array[index] = __array[index] <= __other.__array[index];
            }
        } else {
            std::vector<std::thread> st;
            st.reserve(MdDynArray::thread_count);
            auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
                for (size_t index = start; index < end; ++index) {
                    result.__array[index] = __array[index] <= __other.__array[index];
                }
            };

            const size_t block = size / thread_count;
            const uint8_t thread_but_one = thread_count - 1;
            for (int i = 0; i < thread_but_one; ++i) {
                st.emplace_back(std::thread(_add_int, block * i, block * (i + 1)));
            }

            st.emplace_back(std::thread(_add_int, block * thread_but_one, size));

            for (auto &th: st) {
                th.join();
            }
        }
        return result;
    }

    template <typename _T1>
    MdDynArray<bool> __comp_neq_internal(const MdDynArray<_T1> &__other) {
        // assert that sizes are equal
        const size_t size = __array.size();
        MdDynArray<bool> result(size);
        if (MdDynArray::thread_count == 1 || __array.size() <= threshold_size) {
            for (size_t index = 0; index < get_size(); ++index) {
                result.__array[index] = __array[index] != __other.__array[index];
            }
        } else {
            std::vector<std::thread> st;
            st.reserve(MdDynArray::thread_count);
            auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
                for (size_t index = start; index < end; ++index) {
                    result.__array[index] = __array[index] != __other.__array[index];
                }
            };

            const size_t block = size / thread_count;
            const uint8_t thread_but_one = thread_count - 1;
            for (int i = 0; i < thread_but_one; ++i) {
                st.emplace_back(std::thread(_add_int, block * i, block * (i + 1)));
            }

            st.emplace_back(std::thread(_add_int, block * thread_but_one, size));

            for (auto &th: st) {
                th.join();
            }
        }
        return result;
    }

    template <typename _T1>
    inline MdDynArray operator+(const _T1 &__other) const {
        OP_INTERNAL_MACRO(__add_iinternal)
    }

    template <typename _T1>
    inline auto operator+(const MdDynArray<_T1>&__other) const {
        OP_INTERNAL_MACRO(__add_internal)
    }

    template <typename _T1>
    inline MdDynArray operator-(const _T1 &__other) const {
        OP_INTERNAL_MACRO(__sub_iinternal)
    }

    template <typename _T1>
    inline auto operator-(const MdDynArray<_T1>&__other) const {
        OP_INTERNAL_MACRO(__sub_internal)
    }

    template <typename _T1>
    inline MdDynArray operator*(const _T1 &__other) const {
        OP_INTERNAL_MACRO(__mul_iinternal)
    }

    template <typename _T1>
    inline auto operator*(const MdDynArray<_T1>&__other) const {
        OP_INTERNAL_MACRO(__mul_internal)
    }

    template <typename _T1>
    inline MdDynArray operator/(const _T1 &__other) const {
        OP_INTERNAL_MACRO(__div_iinternal)
    }

    template <typename _T1>
    inline auto operator/(const MdDynArray<_T1>&__other) const {
        OP_INTERNAL_MACRO(__div_internal)
    }

    template <typename _T1>
    inline MdDynArray operator%(const _T1 &__other) const {
        OP_INTERNAL_MACRO(__mod_iinternal)
    }

    template <typename _T1>
    inline auto operator%(const MdDynArray<_T1>&__other) const {
        OP_INTERNAL_MACRO(__mod_internal)
    }

    inline MdDynArray &operator+=(const MdDynArray&__other) {
        __add_self_internal(__other);
        return *this;
    }

    inline MdDynArray &operator-=(const MdDynArray&__other) {
        __sub_self_internal(__other);
        return *this;
    }

    inline MdDynArray &operator*=(const MdDynArray&__other) {
        __mul_self_internal(__other);
        return *this;
    }

    inline MdDynArray &operator/=(const MdDynArray&__other) {
        __div_self_internal(__other);
        return *this;
    }

    template <typename _T1>
    inline MdDynArray<bool> operator==(const MdDynArray<_T1>&__other) {
        return __comp_eq_internal(__other);
    }

    template <typename _T1>
    inline MdDynArray<bool> operator>(const MdDynArray<_T1>&__other) {
        return __comp_g_internal(__other);
    }

    template <typename _T1>
    inline MdDynArray<bool> operator>=(const MdDynArray<_T1>&__other) {
        return __comp_geq_internal(__other);
    }

    template <typename _T1>
    inline MdDynArray<bool> operator<(const MdDynArray<_T1>&__other) {
        return __comp_l_internal(__other);
    }

    template <typename _T1>
    inline MdDynArray<bool> operator<=(const MdDynArray<_T1>&__other) {
        return __comp_leq_internal(__other);
    }

    template <typename _T1>
    inline MdDynArray<bool> operator!=(const MdDynArray<_T1>&__other) {
        return __comp_neq_internal(__other);
    }

    inline typename std::vector<_T>::reference operator[](const size_t index) {
        return __array[index];
    }

    inline size_t get_size() const {
        return __array.size();
    }
};

#include "md_dyn_array.tcc"

#define OP_INTERNAL_MACRO_EXT(func) \
if constexpr (sizeof(_T1) > sizeof(_T2)) { \
    if constexpr (std::is_floating_point<_T2>::value && sizeof(_T1) == 4) { \
        return first.func(__other, (float)0); \
    } else if constexpr (std::is_floating_point<_T2>::value && sizeof(_T1) == 8) { \
        return first.func(__other, (double)0); \
    } else { \
        return first.func(__other, (_T1)0); \
    } \
} else { \
    if constexpr (std::is_floating_point<_T2>::value && sizeof(_T1) == 4) { \
        return first.func(__other, (double)0); \
    } else if constexpr (std::is_floating_point<_T2>::value && sizeof(_T1) == 8) { \
        return first.func(__other, (double)0); \
    } else if constexpr (std::is_floating_point<_T2>::value && sizeof(_T1) < 4) { \
        return first.func(__other, (float)0); \
    } else { \
        return first.func(__other, (_T2)0); \
    } \
} \

template <typename _T>
void MdDynArray<_T>::set_thread_count(const uint8_t value) {
    thread_count = value;
}

template <typename _T>
void MdDynArray<_T>::set_threshold_size(const size_t value) {
    threshold_size = value;
}

template <typename _T1, typename _T2>
inline auto operator+(const _T1&__other, const MdDynArray<_T2> &first) {
    return first + __other;
}

template <typename _T1, typename _T2>
inline auto operator-(const _T1&__other, const MdDynArray<_T2> &first) {
    OP_INTERNAL_MACRO_EXT(__sub_iointernal)
}

template <typename _T1, typename _T2>
inline auto operator*(const _T1&__other, const MdDynArray<_T2> &first) {
    return first * __other;
}

template <typename _T1, typename _T2>
inline auto operator/(const _T1&__other, const MdDynArray<_T2> &first) {
    OP_INTERNAL_MACRO_EXT(__div_iointernal)
}

template <typename _T1, typename _T2>
inline auto operator%(const _T1&__other, const MdDynArray<_T2> &first) {
    OP_INTERNAL_MACRO_EXT(__mod_iointernal)
}

#undef EN_IF
#undef IS_ARITH

#endif
