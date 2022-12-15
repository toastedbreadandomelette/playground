#include <vector>
#include <cassert>
#include <functional>
#include <iostream>

/**
 * @brief Combine values for an item in index 
 * 
 * @tparam _T general array type
 * @param __f fenwick tree
 * @param index index at which current value is situated
 * @param __in input array
 * @param other other value
 * @param func combining function
 * @returns void
 */
template<typename _T>
void combine(std::vector<_T>&__f,
             size_t index,
             size_t size,
             const _T&other, 
             const std::function<_T(const _T&A, const _T&B)>&func) {
    for (; index < size; index = index | (index + 1)) {
        __f[index] = func(__f[index], other);
    }
}

/**
 * @brief Create a fen tree object
 * 
 * @tparam _T template parameter for input array
 * @param __in input array
 * @param func combining function
 * @param init initializing function
 * @return std::vector<_T> fenwick tree.
 */
template<typename _T>
std::vector<_T> create_fen_tree(const std::vector<_T>&__in, const std::function<_T(const _T&A, const _T&B)>&func, _T init) {
    _T result = init;
    std::vector<_T>fen_tree(__in.size(), init);
    for (size_t i = 0; i < __in.size(); ++i) {
        combine<_T>(fen_tree, i, __in.size(), __in[i], func);
    }
    return fen_tree;
}

/**
 * @brief Returns the combined value of first n value in array
 * 
 * @tparam _T template type for array input
 * @param __f fenwick tree
 * @param right rightmost index of a fenwick tree to query for
 * @param func combining function
 * @param init initializing value for result to start
 * @return _T the result value
 */
template<typename _T>
_T first_n_query(std::vector<_T>&__f, size_t right, const std::function<_T(const _T&A, const _T&B)>&func, _T init) {
    _T res = init;
    for (; right < __f.size(); right = (right & (right + 1)) - 1) {
        res = func(res, __f[right]);
    }
    return res;
}

/**
 * @brief Return range query on the range [l, r]
 * For this inverse operator is necessary, since r already considers values from [0, l]
 * as well. For this, we compute [0, l-1] and remove these values to get the answer in
 * range [l, r].
 * 
 * for e.g., a = {8, 2, 1, 10, 5, 19, 7, 8, 12, 256, 234, 102, 11, 3}
 * 
 * range_query(left=6, right=12, func=add(a, b), inv_func=sub(a, b))
 * 
 * will be first_n_query(12)=675, first_n_query(5)=45
 * 
 * => range_query=630
 * 
 * @tparam _T template parameter of input array
 * @param __f fenwick tree
 * @param left left index as starting point of value
 * @param right right index as ending point of value
 * @param func combining function
 * @param inv_func inverse function to remove calculated value
 * @param init 
 * @return _T 
 */
template<typename _T>
_T range_query(std::vector<_T>&__f,
               size_t left, size_t right,
               const std::function<_T(const _T&A, const _T&B)>&func,
               const std::function<_T(const _T&A, const _T&B)>&inv_func,
               _T init) {
    return inv_func(first_n_query(__f, right, func, init), first_n_query(__f, left - 1, func, init));
}

/**
 * @brief Update the value at a given position by changing the value by 
 * using func.
 * 
 * Note: Changing a certain element *to* a certain value is also possible:
 * FOR A SINGLE ELEMENT ONLY, only difference would be changing the value
 * 
 * @tparam _T type array
 * @param __f fenwick tree input
 * @param pos position for updating the value
 * @param func combining function for update
 * @param value value to update
 */
template<typename _T>
void single_update_by_using(std::vector<_T>&__f,
               std::vector<_T>&__in,
               size_t pos,
               const std::function<_T(const _T&A, const _T&B)>&func,
               _T value) {
    __in[pos] =func(__in[pos], value);
    for (; pos < __f.size(); pos = pos | (pos + 1)) {
        __f[pos] = func(__f[pos], value);
    }
}

/**
 * @brief Update the value at a given position to a certain value
 * 
 * @tparam _T type array
 * @param __f fenwick tree input
 * @param __in input of an array
 * @param pos position for updating the value
 * @param func combining function for update
 * @param inv_func inverse operator for removing values from fen_tree
 * @param value value to update
 */
template<typename _T>
void single_update_to(std::vector<_T>&__f,
               std::vector<_T>&__in,
               size_t pos,
               const std::function<_T(const _T&A, const _T&B)>&func,
               const std::function<_T(const _T&A, const _T&B)>&inv_func,
               _T value) {
    _T previous = __in[pos];
    __in[pos] = value;
    for (; pos < __f.size(); pos = pos | (pos + 1)) {
        __f[pos] = inv_func(__f[pos], previous);
        __f[pos] = func(__f[pos], value);
    }
}

int main () {
    auto func = [](const int &a, const int &b) {
        return a^b;
    };
    auto inv_func = [](const int &a, const int &b) {
        return a^b;
    };

    std::vector<int> mp = {8, 2, 1, 10, 5, 19, 7, 8, 12, 256, 234, 102, 11, 3};
    std::vector<int> fen = create_fen_tree<int>(mp, func, 0);

    int ans = range_query<int>(fen, 6, 12, func, inv_func, 0);
    std::cout << ans << std::endl;
    ans = first_n_query<int>(fen, 4, func, 0);
    std::cout << ans << std::endl;
    ans = range_query<int>(fen, 10, 12, func, inv_func, 0);
    std::cout << ans << std::endl;

    single_update_to<int>(fen, mp, 3, func, inv_func, 20);

    ans = first_n_query<int>(fen, 4, func, 0);
    std::cout << ans << std::endl;

    return 0;
}
