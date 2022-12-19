#include "md_dyn_array.hpp"

template <typename _T>
template <typename _T1, typename _T2>
MdDynArray<_T2> MdDynArray<_T>::__add_internal(const MdDynArray<_T1>&__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __array.size();
    MdDynArray<_T2> result(size);
    if (::thread_count == 1 || __array.size() <= threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] + __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] + __other.__array[index];
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

template <typename _T>
template <typename _T1, typename _T2>
MdDynArray<_T2> MdDynArray<_T>::__add_iinternal(const _T1 &__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __array.size();
    MdDynArray<_T2> result(size);
    if (::thread_count == 1 || __array.size() <= threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] + __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] + __other;
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

template <typename _T>
template <typename _T1, typename _T2>
MdDynArray<_T2> MdDynArray<_T>::__sub_iinternal(const _T1 &__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __array.size();
    MdDynArray<_T2> result(size);
    if (::thread_count == 1 || __array.size() <= threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] - __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] - __other;
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

template <typename _T>
template <typename _T1, typename _T2>
MdDynArray<_T2> MdDynArray<_T>::__sub_iointernal(const _T1 &__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __array.size();
    MdDynArray<_T2> result(size);
    if (::thread_count == 1 || __array.size() <= threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __other - __array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __other - __array[index];
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

//////////////////////////////////////////////////////////////////////////////////////

template <typename _T>
template <typename _T1, typename _T2>
MdDynArray<_T2> MdDynArray<_T>::__mul_internal(const MdDynArray<_T1>&__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __array.size();
    MdDynArray<_T2> result(size);
    if (::thread_count == 1 || __array.size() <= threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] * __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] * __other.__array[index];
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

template <typename _T>
template <typename _T1, typename _T2>
MdDynArray<_T2> MdDynArray<_T>::__mul_iinternal(const _T1 &__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __array.size();
    MdDynArray<_T2> result(size);
    if (::thread_count == 1 || __array.size() <= threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] * __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] * __other;
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

//////////////////////////////////////////////////////////////////////////////////////////////

template <typename _T>
template <typename _T1, typename _T2>
MdDynArray<_T2> MdDynArray<_T>::__div_internal(const MdDynArray<_T1>&__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __array.size();
    MdDynArray<_T2> result(size);
    if (::thread_count == 1 || __array.size() <= threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] / __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] / __other.__array[index];
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

template <typename _T>
template <typename _T1, typename _T2>
MdDynArray<_T2> MdDynArray<_T>::__div_iinternal(const _T1 &__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __array.size();
    MdDynArray<_T2> result(size);
    if (::thread_count == 1 || __array.size() <= threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] / __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] / __other;
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

template <typename _T>
template <typename _T1, typename _T2>
MdDynArray<_T2> MdDynArray<_T>::__div_iointernal(const _T1 &__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __array.size();
    MdDynArray<_T2> result(size);
    if (::thread_count == 1 || __array.size() <= threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __other / __array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __other / __array[index];
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

//////////////////////////////////////////////////////////////////////////////////////////////

template <typename _T>
template <typename _T1, typename _T2>
MdDynArray<_T2> MdDynArray<_T>::__mod_internal(const MdDynArray<_T1>&__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __array.size();
    MdDynArray<_T2> result(size);
    if (::thread_count == 1 || __array.size() <= threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] % __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] % __other.__array[index];
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

template <typename _T>
template <typename _T1, typename _T2>
MdDynArray<_T2> MdDynArray<_T>::__mod_iinternal(const _T1 &__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __array.size();
    MdDynArray<_T2> result(size);
    if (::thread_count == 1 || __array.size() <= threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] % __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] % __other;
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

template <typename _T>
template <typename _T1, typename _T2>
MdDynArray<_T2> MdDynArray<_T>::__mod_iointernal(const _T1 &__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __array.size();
    MdDynArray<_T2> result(size);
    if (::thread_count == 1 || __array.size() <= threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __other % __array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __other % __array[index];
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

//////////////////////////////////////////////////////////////////////////////////////////////

template <typename _T>
template <typename _T1>
void MdDynArray<_T>::__add_self_internal(const MdDynArray<_T1> &__other) {
    // assert that sizes are equal
    const size_t size = __array.size();
    if (::thread_count == 1 || __array.size() <= threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            __array[index] += __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::thread_count);
        auto _add_int = [this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                __array[index] += __other.__array[index];
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
}

template <typename _T>
template <typename _T1>
void MdDynArray<_T>::__sub_self_internal(const MdDynArray<_T1> &__other) {
    // assert that sizes are equal
    const size_t size = __array.size();
    if (::thread_count == 1 || __array.size() <= threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
                __array[index] -= __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::thread_count);
        auto _add_int = [this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                __array[index] -= __other.__array[index];
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
}

template <typename _T>
template <typename _T1>
void MdDynArray<_T>::__mul_self_internal(const MdDynArray<_T1> &__other) {
    // assert that sizes are equal
    const size_t size = __array.size();
    if (::thread_count == 1 || __array.size() <= threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            __array[index] *= __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::thread_count);
        auto _add_int = [this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                __array[index] *= __other.__array[index];
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
}

template <typename _T>
template <typename _T1>
void MdDynArray<_T>::__div_self_internal(const MdDynArray<_T1> &__other) {
    // assert that sizes are equal
    const size_t size = __array.size();
    if (::thread_count == 1 || __array.size() <= threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            __array[index] /= __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::thread_count);
        auto _add_int = [this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                __array[index] /= __other.__array[index];
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
}


template <typename _T>
template <typename _T1>
void MdDynArray<_T>::__mod_self_internal(const MdDynArray<_T1> &__other) {
    // assert that sizes are equal
    const size_t size = __array.size();
    if (::thread_count == 1 || __array.size() <= threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            __array[index] %= __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::thread_count);
        auto _add_int = [this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                __array[index] %= __other.__array[index];
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
}


//////////////////////////////////////////////////////////////////////////////////////////////

template <typename _T>
template <typename _T1>
void MdDynArray<_T>::__add_self_iinternal(const _T1 &__other) {
    // assert that sizes are equal
    const size_t size = __array.size();
    if (::thread_count == 1 || __array.size() <= threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            __array[index] += __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::thread_count);
        auto _add_int = [this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                __array[index] += __other;
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
}

template <typename _T>
template <typename _T1>
void MdDynArray<_T>::__sub_self_iinternal(const _T1 &__other) {
    // assert that sizes are equal
    const size_t size = __array.size();
    if (::thread_count == 1 || __array.size() <= threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
                __array[index] -= __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::thread_count);
        auto _add_int = [this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                __array[index] -= __other;
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
}

template <typename _T>
template <typename _T1>
void MdDynArray<_T>::__mul_self_iinternal(const _T1 &__other) {
    // assert that sizes are equal
    const size_t size = __array.size();
    if (::thread_count == 1 || __array.size() <= threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            __array[index] *= __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::thread_count);
        auto _add_int = [this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                __array[index] *= __other;
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
}

template <typename _T>
template <typename _T1>
void MdDynArray<_T>::__div_self_iinternal(const _T1 &__other) {
    // assert that sizes are equal
    const size_t size = __array.size();
    if (::thread_count == 1 || __array.size() <= threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            __array[index] /= __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::thread_count);
        auto _add_int = [this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                __array[index] /= __other;
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
}

template <typename _T>
template <typename _T1>
void MdDynArray<_T>::__mod_self_iinternal(const _T1 &__other) {
    // assert that sizes are equal
    const size_t size = __array.size();
    if (::thread_count == 1 || __array.size() <= threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            __array[index] %= __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::thread_count);
        auto _add_int = [this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                __array[index] %= __other;
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
}
