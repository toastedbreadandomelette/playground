#pragma once

#ifndef _MD_STATIC_ARRAY_HPP_
#define _MD_STATIC_ARRAY_HPP_
#include <vector>
#include <thread>
#include <iostream>
#include <algorithm>

#define EN_IF(C) std::enable_if<C>::type

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

static size_t s_threshold_size = 10000000;
static uint8_t s_thread_count = 16;

template <typename _T>
class MdStaticArray {
public:
    _T *__array;
    std::vector<size_t> shape;
    size_t __size;

    static void set_s_thread_count(const uint8_t value);
    
    static void set_threshold_size(const size_t size);

    MdStaticArray(const size_t size) {
        __array = (_T*)malloc(size*sizeof(_T));
        __size = size;
        shape.push_back(size);
    }

    MdStaticArray(const size_t size, const _T&value) {
        __array = (_T*)malloc(size*sizeof(_T));
        __size = size;
        for (size_t index = 0; index < size; ++index) {
            __array[index] = value;
        }
        shape.push_back(size);
    }

    MdStaticArray(const std::vector<size_t> &_shape, const _T&value) {
        size_t overall_size = 1;
        for (auto &dim: _shape) {
            overall_size *= dim;
            shape.push_back(dim);
        }
        __array = (_T*)malloc(overall_size*sizeof(_T));
        __size = overall_size;
        for (size_t index = 0; index < overall_size; ++index) {
            __array[index] = value;
        }
    }

    MdStaticArray() { __array = nullptr; }

    MdStaticArray(const std::vector<_T>&__other) {
        __array = (_T*) malloc(__other.size()*sizeof(_T));
        __size = __other.size();
        size_t index = 0;
        for (auto &elem: __other) {
            __array[index++] = elem;
        }
        shape.push_back(__size);
    }

    MdStaticArray(MdStaticArray& __other) {
        __array = (_T*) malloc(__other.get_size()*sizeof(_T));
        __size = __other.get_size();
        const auto shp = __other.get_shape();
        shape.insert(shape.end(), shp.begin(), shp.end());
        for (size_t index = 0; index < __other.get_size(); ++index) {
            __array[index] = __other.__array[index];
        }
    }

    inline std::vector<size_t> get_shape() {
        return shape;
    }

    /**
     * @brief Assignment operator (direct vector assignment)
     */
    MdStaticArray &operator=(const std::vector<_T>&__other) {
        __array = (_T*) malloc(__other.size()*sizeof(_T));
        __size = __other.size();
        size_t index = 0;
        for (auto &elem: __other) {
            __array[index++] = elem;
        }
        return *this;
    }

    /**
     * @brief Assignment operator 
     */
    MdStaticArray &operator=(const MdStaticArray& __other) {
        __array = (_T*) malloc(__other.get_size()*sizeof(_T));
        __size = __other.get_size();
        for (size_t index = 0; index < __other.get_size(); ++index) {
            __array[index] = __other.__array[index];
        }
        return *this;
    }

    /**
     * @brief Add function, currently using threads
     * @param __other other array (might be of different type)
     * @returns new array of current type
     */
    template <typename _T1, typename _T2>
    MdStaticArray<_T2> __add_internal(const MdStaticArray<_T1> &__other, const _T2) const;

    /**
     * @brief Add function, currently using threads
     * @param __other other integer (might be of different type)
     * @returns new array of current type
    */
    template <typename _T1, typename _T2>
    MdStaticArray<_T2> __add_iinternal(const _T1 &__other, const _T2) const;

    /**
     * @brief Subtract function, currently using threads
     * @param __other other array (might be of different type)
     * @returns new array of current type
     */
    template <typename _T1, typename _T2>
    MdStaticArray<_T2> __sub_internal(const MdStaticArray<_T1> &, const _T2) const;

    /**
     * @brief Subtract function, currently using threads
     * @param __other other integer (might be of different type)
     * @returns new array of current type
    */
    template <typename _T1, typename _T2>
    MdStaticArray<_T2> __sub_iinternal(const _T1 &, const _T2) const;

    /**
     * @brief Subtract function, currently using threads
     * @param __other other integer (might be of different type)
     * @returns new array of current type
    */
    template <typename _T1, typename _T2>
    MdStaticArray<_T2> __sub_iointernal(const _T1 &, const _T2) const;

    /**
     * @brief Multiplication function, currently using threads
     * @param __other other array (might be of different type)
     * @returns new array of current type
     */
    template <typename _T1, typename _T2>
    MdStaticArray<_T2> __mul_internal(const MdStaticArray<_T1> &, const _T2) const;

    /**
     * @brief Multiplication function, currently using threads
     * @param __other other integer (might be of different type)
     * @returns new array of current type
    */
    template <typename _T1, typename _T2>
    MdStaticArray<_T2> __mul_iinternal(const _T1 &__other, const _T2) const;

    /**
     * @brief Division function, currently using threads
     * @param __other other array (might be of different type)
     * @returns new array of current type
     */
    template <typename _T1, typename _T2>
    MdStaticArray<_T2> __div_internal(const MdStaticArray<_T1> &, const _T2) const;

    /**
     * @brief Division function, currently using threads
     * @param __other other integer (might be of different type)
     * @returns new array of current type
     */
    template <typename _T1, typename _T2>
    MdStaticArray<_T2> __div_iinternal(const _T1 &__other, const _T2) const;

    /**
     * @brief Division function, currently using threads
     * @param __other other integer (might be of different type)
     * @returns new array of current type
     */
    template <typename _T1, typename _T2>
    MdStaticArray<_T2> __div_iointernal(const _T1 &__other, const _T2) const;

    /**
     * @brief Division function, currently using threads
     * @param __other other array (might be of different type)
     * @returns new array of current type
     */
    template <typename _T1, typename _T2>
    MdStaticArray<_T2> __mod_internal(const MdStaticArray<_T1> &, const _T2) const;

    /**
     * @brief Division function, currently using threads
     * @param __other other integer (might be of different type)
     * @returns new array of current type
     */
    template <typename _T1, typename _T2>
    MdStaticArray<_T2> __mod_iinternal(const _T1 &__other, const _T2) const;

    /**
     * @brief Division function, currently using threads
     * @param __other other integer (might be of different type)
     * @returns new array of current type
     */
    template <typename _T1, typename _T2>
    MdStaticArray<_T2> __mod_iointernal(const _T1 &__other, const _T2) const;

    /**
     * @brief Add to self, using multi-threading
     * @param __other other vector to add
     * @returns new array
     */
    template <typename _T1>
    void __add_self_internal(const MdStaticArray<_T1> &__other);

    /**
     * @brief Subtract to self, using multi-threading
     * @param __other other vector to add
     * @returns new array
     */
    template <typename _T1>
    void __sub_self_internal(const MdStaticArray<_T1> &__other);

    /**
     * @brief Multiply to self, using multi-threading
     * @param __other other vector to add
     * @returns new array
     */
    template <typename _T1>
    void __mul_self_internal(const MdStaticArray<_T1> &__other);

    /**
     * @brief Multiply to self, using multi-threading
     * @param __other other vector to add
     * @returns new array
     */
    template <typename _T1>
    void __div_self_internal(const MdStaticArray<_T1> &__other);

    /**
     * @brief Multiply to self, using multi-threading
     * @param __other other vector to add
     * @returns new array
     */
    template <typename _T1>
    void __mod_self_internal(const MdStaticArray<_T1> &__other);

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
    MdStaticArray<bool> __comp_eq_internal(const MdStaticArray<_T1> &__other) {
        // assert that sizes are equal
        const size_t size = size;
        MdStaticArray<bool> result(size);
        if (::s_thread_count == 1 || size <= s_threshold_size) {
            for (size_t index = 0; index < get_size(); ++index) {
                result.__array[index] = __array[index] == __other.__array[index];
            }
        } else {
            std::vector<std::thread> st;
            st.reserve(::s_thread_count);
            auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
                for (size_t index = start; index < end; ++index) {
                    result.__array[index] = __array[index] == __other.__array[index];
                }
            };

            const size_t block = size / s_thread_count;
            const uint8_t thread_but_one = s_thread_count - 1;
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
    MdStaticArray<bool> __comp_g_internal(const MdStaticArray<_T1> &__other) {
        // assert that sizes are equal
        const size_t size = __array.size();
        MdStaticArray<bool> result(size);
        if (MdStaticArray::s_thread_count == 1 || __array.size() <= s_threshold_size) {
            for (size_t index = 0; index < get_size(); ++index) {
                result.__array[index] = __array[index] > __other.__array[index];
            }
        } else {
            std::vector<std::thread> st;
            st.reserve(MdStaticArray::s_thread_count);
            auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
                for (size_t index = start; index < end; ++index) {
                    result.__array[index] = __array[index] > __other.__array[index];
                }
            };

            const size_t block = size / s_thread_count;
            const uint8_t thread_but_one = s_thread_count - 1;
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
    MdStaticArray<bool> __comp_geq_internal(const MdStaticArray<_T1> &__other) {
        // assert that sizes are equal
        const size_t size = __array.size();
        MdStaticArray<bool> result(size);
        if (MdStaticArray::s_thread_count == 1 || __array.size() <= s_threshold_size) {
            for (size_t index = 0; index < get_size(); ++index) {
                result.__array[index] = __array[index] >= __other.__array[index];
            }
        } else {
            std::vector<std::thread> st;
            st.reserve(MdStaticArray::s_thread_count);
            auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
                for (size_t index = start; index < end; ++index) {
                    result.__array[index] = __array[index] >= __other.__array[index];
                }
            };

            const size_t block = size / s_thread_count;
            const uint8_t thread_but_one = s_thread_count - 1;
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
    MdStaticArray<bool> __comp_l_internal(const MdStaticArray<_T1> &__other) {
        // assert that sizes are equal
        const size_t size = __array.size();
        MdStaticArray<bool> result(size);
        if (MdStaticArray::s_thread_count == 1 || __array.size() <= s_threshold_size) {
            for (size_t index = 0; index < get_size(); ++index) {
                result.__array[index] = __array[index] < __other.__array[index];
            }
        } else {
            std::vector<std::thread> st;
            st.reserve(MdStaticArray::s_thread_count);
            auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
                for (size_t index = start; index < end; ++index) {
                    result.__array[index] = __array[index] < __other.__array[index];
                }
            };

            const size_t block = size / s_thread_count;
            const uint8_t thread_but_one = s_thread_count - 1;
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
    MdStaticArray<bool> __comp_leq_internal(const MdStaticArray<_T1> &__other) {
        // assert that sizes are equal
        const size_t size = __array.size();
        MdStaticArray<bool> result(size);
        if (MdStaticArray::s_thread_count == 1 || __array.size() <= s_threshold_size) {
            for (size_t index = 0; index < get_size(); ++index) {
                result.__array[index] = __array[index] <= __other.__array[index];
            }
        } else {
            std::vector<std::thread> st;
            st.reserve(MdStaticArray::s_thread_count);
            auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
                for (size_t index = start; index < end; ++index) {
                    result.__array[index] = __array[index] <= __other.__array[index];
                }
            };

            const size_t block = size / s_thread_count;
            const uint8_t thread_but_one = s_thread_count - 1;
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
    MdStaticArray<bool> __comp_neq_internal(const MdStaticArray<_T1> &__other) {
        // assert that sizes are equal
        const size_t size = __array.size();
        MdStaticArray<bool> result(size);
        if (MdStaticArray::s_thread_count == 1 || __array.size() <= s_threshold_size) {
            for (size_t index = 0; index < get_size(); ++index) {
                result.__array[index] = __array[index] != __other.__array[index];
            }
        } else {
            std::vector<std::thread> st;
            st.reserve(MdStaticArray::s_thread_count);
            auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
                for (size_t index = start; index < end; ++index) {
                    result.__array[index] = __array[index] != __other.__array[index];
                }
            };

            const size_t block = size / s_thread_count;
            const uint8_t thread_but_one = s_thread_count - 1;
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
    inline auto operator+(const _T1 &__other) const {
        OP_INTERNAL_MACRO(__add_iinternal)
    }

    template <typename _T1>
    inline auto operator+(const MdStaticArray<_T1>&__other) const {
        OP_INTERNAL_MACRO(__add_internal)
    }

    template <typename _T1>
    inline auto operator-(const _T1 &__other) const {
        OP_INTERNAL_MACRO(__sub_iinternal)
    }

    template <typename _T1>
    inline auto operator-(const MdStaticArray<_T1>&__other) const {
        OP_INTERNAL_MACRO(__sub_internal)
    }
    
    template <typename _T1>
    inline auto operator*(const MdStaticArray<_T1>&__other) const {
        OP_INTERNAL_MACRO(__mul_internal)
    }

    template <typename _T1>
    inline auto operator*(const _T1 &__other) const {
        OP_INTERNAL_MACRO(__mul_iinternal)
    }

    template <typename _T1>
    inline auto operator/(const _T1 &__other) const {
        OP_INTERNAL_MACRO(__div_iinternal)
    }

    template <typename _T1>
    inline auto operator/(const MdStaticArray<_T1>&__other) const {
        OP_INTERNAL_MACRO(__div_internal)
    }

    template <typename _T1>
    inline auto operator%(const _T1 &__other) const {
        OP_INTERNAL_MACRO(__mod_iinternal)
    }

    template <typename _T1>
    inline auto operator%(const MdStaticArray<_T1>&__other) const {
        OP_INTERNAL_MACRO(__mod_internal)
    }

    inline MdStaticArray &operator+=(const MdStaticArray&__other) {
        __add_self_internal(__other);
        return *this;
    }

    inline MdStaticArray &operator-=(const MdStaticArray&__other) {
        __sub_self_internal(__other);
        return *this;
    }

    inline MdStaticArray &operator*=(const MdStaticArray&__other) {
        __mul_self_internal(__other);
        return *this;
    }

    inline MdStaticArray &operator/=(const MdStaticArray&__other) {
        __div_self_internal(__other);
        return *this;
    }

    template <typename _T1>
    inline MdStaticArray<bool> operator==(const MdStaticArray<_T1>&__other) {
        return __comp_eq_internal(__other);
    }

    template <typename _T1>
    inline MdStaticArray<bool> operator>(const MdStaticArray<_T1>&__other) {
        return __comp_g_internal(__other);
    }

    template <typename _T1>
    inline MdStaticArray<bool> operator>=(const MdStaticArray<_T1>&__other) {
        return __comp_geq_internal(__other);
    }

    template <typename _T1>
    inline MdStaticArray<bool> operator<(const MdStaticArray<_T1>&__other) {
        return __comp_l_internal(__other);
    }

    template <typename _T1>
    inline MdStaticArray<bool> operator<=(const MdStaticArray<_T1>&__other) {
        return __comp_leq_internal(__other);
    }

    template <typename _T1>
    inline MdStaticArray<bool> operator!=(const MdStaticArray<_T1>&__other) {
        return __comp_neq_internal(__other);
    }

    inline _T &operator[](const size_t index) {
        return __array[index];
    }

    inline size_t get_size() const {
        return __size;
    }
};

#include "md_static_array.tcc"

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
void MdStaticArray<_T>::set_s_thread_count(const uint8_t value) {
    s_thread_count = value;
}

template <typename _T>
void MdStaticArray<_T>::set_threshold_size(const size_t value) {
    s_threshold_size = value;
}

template <typename _T1, typename _T2, class = typename EN_IF(IS_ARITH(_T1))>
inline auto operator+(const _T1&__other, const MdStaticArray<_T2> &first) {
    return first + __other;
}

template <typename _T1, typename _T2, class = typename EN_IF(IS_ARITH(_T1))>
inline auto operator-(const _T1&__other, const MdStaticArray<_T2> &first) {
    OP_INTERNAL_MACRO_EXT(__sub_iointernal)
}

template <typename _T1, typename _T2, class = typename EN_IF(IS_ARITH(_T1))>
inline auto operator*(const _T1&__other, const MdStaticArray<_T2> &first) {
    return first * __other;
}

template <typename _T1, typename _T2, class = typename EN_IF(IS_ARITH(_T1))>
inline auto operator/(const _T1&__other, const MdStaticArray<_T2> &first) {
    OP_INTERNAL_MACRO_EXT(__div_iointernal)
}

template <typename _T1, typename _T2, class = typename EN_IF(IS_ARITH(_T1))>
inline auto operator%(const _T1&__other, const MdStaticArray<_T2> &first) {
    OP_INTERNAL_MACRO_EXT(__mod_iointernal)
}

#endif
