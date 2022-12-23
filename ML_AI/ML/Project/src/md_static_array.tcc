#include "md_static_array.hpp"

template <typename _T>
template <typename _T1, typename _T2>
MdStaticArray<_T2> MdStaticArray<_T>::__add_internal(const MdStaticArray<_T1>&__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __size;
    MdStaticArray<_T2> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || __size <= s_threshold_size) {
        for (size_t index = 0; index < size; ++index) {
            result.__array[index] = __array[index] + __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] + __other.__array[index];
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

template <typename _T>
template <typename _T1, typename _T2>
MdStaticArray<_T2> MdStaticArray<_T>::__add_iinternal(const _T1 &__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __size;
    MdStaticArray<_T2> result(__size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || __size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] + __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] + __other;
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

template <typename _T>
template <typename _T1, typename _T2>
MdStaticArray<_T2> MdStaticArray<_T>::__sub_iinternal(const _T1 &__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __size;
    MdStaticArray<_T2> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || __size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] - __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] - __other;
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

template <typename _T>
template <typename _T1, typename _T2>
MdStaticArray<_T2> MdStaticArray<_T>::__sub_iointernal(const _T1 &__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __size;
    MdStaticArray<_T2> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || __size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __other - __array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __other - __array[index];
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

//////////////////////////////////////////////////////////////////////////////////////

template <typename _T>
template <typename _T1, typename _T2>
MdStaticArray<_T2> MdStaticArray<_T>::__mul_internal(const MdStaticArray<_T1>&__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __size;
    MdStaticArray<_T2> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || __size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] * __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] * __other.__array[index];
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

template <typename _T>
template <typename _T1, typename _T2>
MdStaticArray<_T2> MdStaticArray<_T>::__mul_iinternal(const _T1 &__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __size;
    MdStaticArray<_T2> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || __size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] * __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] * __other;
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

//////////////////////////////////////////////////////////////////////////////////////////////

template <typename _T>
template <typename _T1, typename _T2>
MdStaticArray<_T2> MdStaticArray<_T>::__div_internal(const MdStaticArray<_T1>&__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __size;
    MdStaticArray<_T2> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || __size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] / __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] / __other.__array[index];
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

template <typename _T>
template <typename _T1, typename _T2>
MdStaticArray<_T2> MdStaticArray<_T>::__div_iinternal(const _T1 &__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __size;
    MdStaticArray<_T2> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || __size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] / __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] / __other;
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

template <typename _T>
template <typename _T1, typename _T2>
MdStaticArray<_T2> MdStaticArray<_T>::__div_iointernal(const _T1 &__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __size;
    MdStaticArray<_T2> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || __size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __other / __array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __other / __array[index];
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

//////////////////////////////////////////////////////////////////////////////////////////////

template <typename _T>
template <typename _T1, typename _T2>
MdStaticArray<_T2> MdStaticArray<_T>::__mod_internal(const MdStaticArray<_T1>&__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __size;
    MdStaticArray<_T2> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || __size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] % __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] % __other.__array[index];
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

template <typename _T>
template <typename _T1, typename _T2>
MdStaticArray<_T2> MdStaticArray<_T>::__mod_iinternal(const _T1 &__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __size;
    MdStaticArray<_T2> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || __size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] % __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] % __other;
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

template <typename _T>
template <typename _T1, typename _T2>
MdStaticArray<_T2> MdStaticArray<_T>::__mod_iointernal(const _T1 &__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = __size;
    MdStaticArray<_T2> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || __size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __other % __array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __other % __array[index];
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

//////////////////////////////////////////////////////////////////////////////////////////////

template <typename _T>
template <typename _T1>
void MdStaticArray<_T>::__add_self_internal(const MdStaticArray<_T1> &__other) {
    // assert that sizes are equal
    const size_t size = __size;
    if (::s_thread_count == 1 || __size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            __array[index] += __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                __array[index] += __other.__array[index];
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
}

template <typename _T>
template <typename _T1>
void MdStaticArray<_T>::__sub_self_internal(const MdStaticArray<_T1> &__other) {
    // assert that sizes are equal
    const size_t size = __size;
    if (::s_thread_count == 1 || __size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            __array[index] -= __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                __array[index] -= __other.__array[index];
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
}

template <typename _T>
template <typename _T1>
void MdStaticArray<_T>::__mul_self_internal(const MdStaticArray<_T1> &__other) {
    // assert that sizes are equal
    const size_t size = __size;
    if (::s_thread_count == 1 || __size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            __array[index] *= __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                __array[index] *= __other.__array[index];
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
}

template <typename _T>
template <typename _T1>
void MdStaticArray<_T>::__div_self_internal(const MdStaticArray<_T1> &__other) {
    // assert that sizes are equal
    const size_t size = __size;
    if (::s_thread_count == 1 || __size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            __array[index] /= __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                __array[index] /= __other.__array[index];
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
}


template <typename _T>
template <typename _T1>
void MdStaticArray<_T>::__mod_self_internal(const MdStaticArray<_T1> &__other) {
    // assert that sizes are equal
    const size_t size = __size;
    if (::s_thread_count == 1 || __size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            __array[index] %= __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                __array[index] %= __other.__array[index];
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
}

//////////////////////////////////////////////////////////////////////////////////////////////

template <typename _T>
template <typename _T1>
void MdStaticArray<_T>::__add_self_iinternal(const _T1 &__other) {
    // assert that sizes are equal
    const size_t size = __size;
    if (::s_thread_count == 1 || __size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            __array[index] += __other;
            // std::cout << "here " << index << " " << size << std::endl;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                __array[index] += __other;
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
}

template <typename _T>
template <typename _T1>
void MdStaticArray<_T>::__sub_self_iinternal(const _T1 &__other) {
    // assert that sizes are equal
    const size_t size = __size;
    if (::s_thread_count == 1 || __size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
                __array[index] -= __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                __array[index] -= __other;
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
}

template <typename _T>
template <typename _T1>
void MdStaticArray<_T>::__mul_self_iinternal(const _T1 &__other) {
    // assert that sizes are equal
    const size_t size = __size;
    if (::s_thread_count == 1 || __size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            __array[index] *= __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                __array[index] *= __other;
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
}

template <typename _T>
template <typename _T1>
void MdStaticArray<_T>::__div_self_iinternal(const _T1 &__other) {
    // assert that sizes are equal
    const size_t size = __size;
    if (::s_thread_count == 1 || __size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            __array[index] /= __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                __array[index] /= __other;
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
}

template <typename _T>
template <typename _T1>
void MdStaticArray<_T>::__mod_self_iinternal(const _T1 &__other) {
    // assert that sizes are equal
    const size_t size = __size;
    if (::s_thread_count == 1 || __size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            __array[index] %= __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                __array[index] %= __other;
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
}

//////////////////////////////////////////////////////////////////////////////////////////////

template <typename _T>
template <typename _T1>
MdStaticArray<bool> MdStaticArray<_T>::__comp_leq_internal(const MdStaticArray<_T1> &__other) const {
    // assert that sizes are equal
    const size_t size = get_size();
    MdStaticArray<bool> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] <= __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
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


template <typename _T>
template <typename _T1>
MdStaticArray<bool> MdStaticArray<_T>::__comp_geq_internal(const MdStaticArray<_T1> &__other) const {
    // assert that sizes are equal
    const size_t size = get_size();
    MdStaticArray<bool> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] >= __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
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


template <typename _T>
template <typename _T1>
MdStaticArray<bool> MdStaticArray<_T>::__comp_eq_internal(const MdStaticArray<_T1> &__other) const {
    // assert that sizes are equal
    const size_t size = get_size();
    MdStaticArray<bool> result(size);
    result.init_shape(shape, shp_size);
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


template <typename _T>
template <typename _T1>
MdStaticArray<bool> MdStaticArray<_T>::__comp_l_internal(const MdStaticArray<_T1> &__other) const {
    // assert that sizes are equal
    const size_t size = get_size();
    MdStaticArray<bool> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] < __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
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


template <typename _T>
template <typename _T1>
MdStaticArray<bool> MdStaticArray<_T>::__comp_g_internal(const MdStaticArray<_T1> &__other) const {
    // assert that sizes are equal
    const size_t size = get_size();
    MdStaticArray<bool> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] > __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
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


template <typename _T>
template <typename _T1>
MdStaticArray<bool> MdStaticArray<_T>::__comp_neq_internal(const MdStaticArray<_T1> &__other) const {
    // assert that sizes are equal
    const size_t size = get_size();
    MdStaticArray<bool> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] != __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
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

//////////////////////////////////////////////////////////////////////////////////////////////

template <typename _T>
template <typename _T1>
MdStaticArray<bool> MdStaticArray<_T>::__comp_leq_iinternal(const _T1 &__other) const {
    // assert that sizes are equal
    const size_t size = get_size();
    MdStaticArray<bool> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] <= __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] <= __other;
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


template <typename _T>
template <typename _T1>
MdStaticArray<bool> MdStaticArray<_T>::__comp_geq_iinternal(const _T1 &__other) const {
    // assert that sizes are equal
    const size_t size = get_size();
    MdStaticArray<bool> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] >= __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
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


template <typename _T>
template <typename _T1>
MdStaticArray<bool> MdStaticArray<_T>::__comp_eq_iinternal(const _T1 &__other) const {
    // assert that sizes are equal
    const size_t size = get_size();
    MdStaticArray<bool> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] == __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] == __other;
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


template <typename _T>
template <typename _T1>
MdStaticArray<bool> MdStaticArray<_T>::__comp_l_iinternal(const _T1 &__other) const {
    // assert that sizes are equal
    const size_t size = get_size();
    MdStaticArray<bool> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] < __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] < __other;
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


template <typename _T>
template <typename _T1>
MdStaticArray<bool> MdStaticArray<_T>::__comp_g_iinternal(const _T1 &__other) const {
    // assert that sizes are equal
    const size_t size = get_size();
    MdStaticArray<bool> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] > __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] > __other;
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


template <typename _T>
template <typename _T1>
MdStaticArray<bool> MdStaticArray<_T>::__comp_neq_iinternal(const _T1 &__other) const {
    // assert that sizes are equal
    const size_t size = get_size();
    MdStaticArray<bool> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] != __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] != __other;
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

template <typename _T>
template <typename _T1, typename _T2>
MdStaticArray<_T2> MdStaticArray<_T>::__and_bit_internal(const MdStaticArray<_T1> &__other, const _T2) const {
    // assert that sizes are equal
    const size_t size = get_size();
    MdStaticArray<_T2> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] & __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] & __other.__array[index];
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

template <typename _T>
template <typename _T1, typename _T2>
MdStaticArray<_T2> MdStaticArray<_T>::__and_bit_iinternal(const _T1 &__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = get_size();
    MdStaticArray<_T2> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] & __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index]  & __other;
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

template <typename _T>
template <typename _T1, typename _T2>
MdStaticArray<_T2> MdStaticArray<_T>::__or_bit_internal(const MdStaticArray<_T1> &__other, const _T2) const {
    // assert that sizes are equal
    const size_t size = get_size();
    MdStaticArray<_T2> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] | __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] | __other.__array[index];
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

template <typename _T>
template <typename _T1, typename _T2>
MdStaticArray<_T2> MdStaticArray<_T>::__or_bit_iinternal(const _T1 &__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = get_size();
    MdStaticArray<_T2> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] | __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] ^ __other;
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

template <typename _T>
template <typename _T1, typename _T2>
MdStaticArray<_T2> MdStaticArray<_T>::__xor_bit_internal(const MdStaticArray<_T1> &__other, const _T2) const {
    // assert that sizes are equal
    const size_t size = get_size();
    MdStaticArray<_T2> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] | __other.__array[index];
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] ^ __other.__array[index];
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

template <typename _T>
template <typename _T1, typename _T2>
MdStaticArray<_T2> MdStaticArray<_T>::__xor_bit_iinternal(const _T1 &__other, const _T2 as) const {
    // assert that sizes are equal
    const size_t size = get_size();
    MdStaticArray<_T2> result(size);
    result.init_shape(shape, shp_size);
    if (::s_thread_count == 1 || size <= s_threshold_size) {
        for (size_t index = 0; index < get_size(); ++index) {
            result.__array[index] = __array[index] ^ __other;
        }
    } else {
        std::vector<std::thread> st;
        st.reserve(::s_thread_count);
        auto _add_int = [&result, this, &__other](const size_t start, const size_t end) {
            for (size_t index = start; index < end; ++index) {
                result.__array[index] = __array[index] ^ __other;
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
