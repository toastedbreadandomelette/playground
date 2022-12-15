#pragma once
#ifndef _MD_UTILITY_HPP_
#define _MD_UTILITY_HPP_
#include <cmath>
#include <functional>
#include "md_dyn_array.hpp"

template <typename _T, typename _func>
MdDynArray<_T> execute_mapping_fn(MdDynArray<_T> &__values, const _func& function_exec) {
    const size_t size = __values.get_size();
    MdDynArray<_T> result(size);
    const uint8_t thread_count = ::thread_count;
    const size_t threshold_size = ::threshold_size;
    if (thread_count == 1 || size <= threshold_size) {
        for (size_t index = 0; index < size; ++index) {
            result.__array[index] = function_exec(__values[index]);
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(thread_count);
        auto _add_int = [&result, &__values, &function_exec](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = function_exec(__values.__array[index]);
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

template <typename _T, typename _func>
_T execute_accumulate_fn(MdDynArray<_T> &__values, const _func& function_exec, const _T init) {
    const size_t size = __values.get_size();
    _T result = init;
    const uint8_t thread_count = ::thread_count;
    const size_t threshold_size = ::threshold_size;
    if (thread_count == 1 || size <= threshold_size) {
        for (size_t index = 0; index < size; ++index) {
            result = function_exec(result, __values[index]);
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(thread_count);
        std::vector<_T>__res_total(thread_count, init);
        auto _add_int = [&__res_total, &__values, &function_exec](const uint8_t thread_number, const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                __res_total[thread_number] = function_exec(__res_total[thread_number], __values[index]);
            }
        };

        const size_t block = size / thread_count;
        const uint8_t thread_but_one = thread_count - 1;
        for (int i = 0; i < thread_but_one; ++i) {
            st.emplace_back(std::thread(_add_int, i, block * i, block * (i + 1)));
        }

        st.emplace_back(std::thread(_add_int, thread_but_one, block * thread_but_one, size));

        for (auto &th: st) {
            th.join();
        }

        for (auto &result_th: __res_total) {
            result = function_exec(result, result_th);
        }
    }
    return result;
}

template <typename _T, typename _func, typename _merge_func>
_T execute_accumulate_n_merge_fn(MdDynArray<_T> &__values, const _func& function_exec, const _merge_func& merge_func, const _T init, const _T merge_init) {
    const size_t size = __values.get_size();
    _T result = merge_init;
    const uint8_t thread_count = ::thread_count;
    const size_t threshold_size = ::threshold_size;
    if (thread_count == 1 || size <= threshold_size) {
        for (size_t index = 0; index < size; ++index) {
            result = function_exec(result, __values[index]);
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(thread_count);
        std::vector<_T>__res_total(thread_count, init);
        auto _add_int = [&__res_total, &__values, &function_exec](const uint8_t thread_number, const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                __res_total[thread_number] = function_exec(__res_total[thread_number], __values[index]);
            }
        };

        const size_t block = size / thread_count;
        const uint8_t thread_but_one = thread_count - 1;
        for (int i = 0; i < thread_but_one; ++i) {
            st.emplace_back(std::thread(_add_int, i, block * i, block * (i + 1)));
        }

        st.emplace_back(std::thread(_add_int, thread_but_one, block * thread_but_one, size));

        for (auto &th: st) {
            th.join();
        }

        for (auto &result_th: __res_total) {
            result = merge_func(result, result_th);
        }
    }
    return result;
}

/**
 * @brief sqrt of all values in list __values
 * @param __values list of values
 * @returns List of values
 */
template <typename _T>
MdDynArray<_T> f_sqrt(MdDynArray<_T> &__values) {
    return execute_mapping_fn(__values, sqrt);
}

/**
 * @brief absolute values of all values in list __values
 * @param __values list of values
 * @returns List of values
 */
template <typename _T>
MdDynArray<_T> f_abs(MdDynArray<_T> &__values) {
    return execute_mapping_fn(__values, fabs);
}

/**
 * @brief Sum of all the values in the list __values
 * @param __values List of all values
 * @param init value to be initialized with
 * @return single number
 */
template <typename _T>
_T f_sum(MdDynArray<_T> &__values, _T init = 0) {
    return execute_accumulate_fn(__values, [](const _T prev_value, const _T current_value) { return prev_value + current_value; }, init);
}

/**
 * @brief Mean of all the values in the list __values
 * @param __values List of all values
 * @param init value to be initialized with
 * @return single number
 */
template <typename _T>
long double f_mean(MdDynArray<_T> &__values, _T init = 0) {
    return execute_accumulate_fn(__values, [](const _T prev_value, const _T current_value) { return prev_value + current_value; }, init) / (__values.get_size() * 1.0);
}

/**
 * @brief Root Mean Square of all the values in the list __values
 * @param __values List of all values
 * @param init value to be initialized with
 * @return single number
 */
template <typename _T>
long double f_rms(MdDynArray<_T> &__values, _T init = 0) {
    long double mean_sq = execute_accumulate_n_merge_fn(__values, 
        [](const _T prev_value, const _T current_value) {
            return prev_value + (current_value * current_value); 
        }, 
        [](const _T prev_value, const _T current_value) {
            return prev_value + current_value; 
        },
        (_T)init,
        (_T)init
    ) / (__values.get_size() * 1.0);
    return sqrt(mean_sq);
}

/**
 * @brief Standard Deviation of all the values in the list __values
 * @param __values List of all values
 * @param init value to be initialized with
 * @return single number
 */
template <typename _T>
long double f_std_dev(MdDynArray<_T> &__values) {
    long double fmean = f_mean(__values);
    long double mean_sq_err = execute_accumulate_n_merge_fn(
        __values,
        [&fmean](const _T prev_value, const _T current_value) {
            return prev_value + (fmean - current_value) * (fmean - current_value);
        },
        [](const _T prev_value, const _T current_value) {
            return prev_value + current_value;
        },
        (_T)0.0,
        (_T)0.0
    ) / (__values.get_size() * 1.0);
    return sqrt(mean_sq_err);
}

/**
 * @brief compute log of values to the base 10
 */
template <typename _T>
MdDynArray<_T> f_log_10(MdDynArray<_T> &__values) {
    return execute_mapping_fn(__values, [](const _T &__value) {
        return log10(__value);
    });
}

/**
 * @brief compute log of values to the base 2
 */
template <typename _T>
MdDynArray<_T> f_log_2(MdDynArray<_T> &__values) {
    return execute_mapping_fn(__values, [](const _T &__value) {
        return log2(__value);
    });
}

/**
 * @brief compute natural logarithm of valuess
 */
template <typename _T>
MdDynArray<_T> f_ln(MdDynArray<_T> &__values) {
    return execute_mapping_fn(__values, [](const _T &__value) {
        return log(__value);
    });
}

/**
 * @brief compute mod power of integers
 */
template <typename _T, typename std::enable_if<std::is_integral<_T>::value>::value>
MdDynArray<_T> f_mod_pow(MdDynArray<_T> &__values, size_t power, size_t _mod) {
    return execute_mapping_fn(__values, [power, _mod](const _T __value) {
        if (power == 0) return 1;
        if (power == 1) return __value;
        uint64_t result = 1, value = __value, pow = power;
        while (pow > 0) {
            if (pow & 1) {
                result *= value;
                result %= _mod;
            }
            value *= value;
            value %= _mod;
            pow >>= 1;
        }
        return result;
    });
}

/**
 * @brief compute power of values
 */
template <typename _T>
MdDynArray<_T> f_pow(MdDynArray<_T> &__values, double power) {
    return execute_mapping_fn(__values, [power](const _T __value) {
        return pow(__value, power);
    });
}

#endif