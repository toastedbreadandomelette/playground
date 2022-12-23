#pragma once
#ifndef _MD_ST_REFERENCE_HPP_
#define _MD_ST_REFERENCE_HPP_

template <typename _T>
class MdStaticArray<_T>::reference {
    void *operator new(size_t size);

    template <typename _T1>
    friend class MdStaticArray;
public:
    MdStaticArray<_T> *__array_reference;
    size_t size;
    size_t offset;
    uint16_t shp_offset;
    reference(MdStaticArray<_T>&__other, const size_t offst) {
        __array_reference = &__other;
        offset = offst;
        size = __other.get_size() / __other.shape[0];
        shp_offset = 1;
    }

    reference() { }

    reference(const reference &__other, const size_t offst) {
        __array_reference = __other.__array_reference;
        offset = offst;
        size = __other.size / (__other.__array_reference->shape[__other.shp_offset]);
        shp_offset = __other.shp_offset + 1;
    }

    template <typename _T1>
    reference &operator=(MdStaticArray<_T1> &value) {
        static_assert(value.get_size() == size);
        // assert shape sizes are equal too
        for (size_t i = 0; i < size; ++i) {
            __array_reference->__array[offset + i] = value.__array[i];
        }

        return *this;
    }

    template <typename _T1>
    reference &operator=(_T1 __other) {
        for (size_t i = 0; i < size; ++i) {
            __array_reference->__array[offset + i] = __other;
        }
        return *this;
    }

    inline reference operator[](const size_t index) {
        if (shp_offset >= __array_reference->shp_size) {
            char c[100];
            sprintf(c, "Expected dimension %d, found %d", shp_offset + 1, __array_reference->shp_size);
            throw std::runtime_error(c);
        }
        return reference(*this, offset + index * __array_reference->skip_vec[shp_offset]);
    }

    friend std::ostream &operator<<(std::ostream &op, const reference &ot) {
        op << ot.__array_reference->__array[ot.offset];
        return op;
    }

    operator _T() const {
        return __array_reference->__array[offset];
    }

    template <typename _T1>
    inline auto operator+(const MdStaticArray<_T1>&__other) {
        return MdStaticArray(*__array_reference, offset, shp_offset) + __other;
    }

    inline auto operator+(const reference&__other) {
        return MdStaticArray(*__array_reference, offset, shp_offset) + MdStaticArray(*__other.__array_reference, __other.offset, __other.shp_offset);
    }

    template <typename _T1, class = typename std::enable_if<!std::is_same<_T1, reference>::value>::type>
    inline auto operator+(const _T1&__other) {
        return MdStaticArray(*__array_reference, offset, shp_offset) + __other;
    }

    template <typename _T1>
    inline auto operator-(const MdStaticArray<_T1>&__other) {
        return MdStaticArray(*__array_reference, offset, shp_offset) - __other;
    }

    inline auto operator-(const reference&__other) {
        return MdStaticArray(*__array_reference, offset, shp_offset) - MdStaticArray(*__other.__array_reference, __other.offset, __other.shp_offset);
    }

    template <typename _T1, class = typename std::enable_if<!std::is_same<_T1, reference>::value>::type>
    inline auto operator-(const _T1&__other) {
        return MdStaticArray(*__array_reference, offset, shp_offset) - __other;
    }

    template <typename _T1>
    inline auto operator*(const MdStaticArray<_T1>&__other) {
        return MdStaticArray(*__array_reference, offset, shp_offset) * __other;
    }

    inline auto operator*(const reference&__other) {
        return MdStaticArray(*__array_reference, offset, shp_offset) * MdStaticArray(*__other.__array_reference, __other.offset, __other.shp_offset);
    }

    template <typename _T1, class = typename std::enable_if<!std::is_same<_T1, reference>::value>::type>
    inline auto operator*(const _T1&__other) {
        return MdStaticArray(*__array_reference, offset, shp_offset) * __other;
    }

    template <typename _T1>
    inline auto operator/(const MdStaticArray<_T1>&__other) {
        return MdStaticArray(*__array_reference, offset, shp_offset) / __other;
    }

    inline auto operator/(const reference&__other) {
        return MdStaticArray(*__array_reference, offset, shp_offset) / MdStaticArray(*__other.__array_reference, __other.offset, __other.shp_offset);
    }

    template <typename _T1, class = typename std::enable_if<!std::is_same<_T1, reference>::value>::type>
    inline auto operator/(const _T1&__other) {
        return MdStaticArray(*__array_reference, offset, shp_offset) / __other;
    }

    template <typename _T1>
    inline auto operator%(const MdStaticArray<_T1>&__other) {
        return MdStaticArray(*__array_reference, offset, shp_offset) % __other;
    }

    inline auto operator%(const reference&__other) {
        return MdStaticArray(*__array_reference, offset, shp_offset) % MdStaticArray(*__other.__array_reference, __other.offset, __other.shp_offset);
    }

    template <typename _T1, class = typename std::enable_if<!std::is_same<_T1, reference>::value>::type>
    inline auto operator%(const _T1&__other) {
        return MdStaticArray(*__array_reference, offset, shp_offset) % __other;
    }
    
    template <typename _T1>
    inline auto operator&(const MdStaticArray<_T1>&__other) {
        return MdStaticArray(*__array_reference, offset, shp_offset) & __other;
    }

    inline auto operator&(const reference&__other) {
        return MdStaticArray(*__array_reference, offset, shp_offset) & MdStaticArray(*__other.__array_reference, __other.offset, __other.shp_offset);
    }

    template <typename _T1, class = typename std::enable_if<!std::is_same<_T1, reference>::value>::type>
    inline auto operator&(const _T1&__other) {
        return MdStaticArray(*__array_reference, offset, shp_offset) & __other;
    }

    template <typename _T1>
    reference &operator+=(const MdStaticArray<_T1>&__other) {
        MdStaticArray(*__array_reference, offset, shp_offset).__add_self_internal(__other);
        return *this;
    }

    reference &operator+=(const reference &__other) {
        // Not suitable: temporary constructor will free array offset instantly, think of something else.
        MdStaticArray(*__array_reference, offset, shp_offset).
            __add_self_internal(MdStaticArray(*__other.__array_reference, __other.offset, __other.shp_offset));
        return *this;
    }

    template <typename _T1, typename std::enable_if<!std::is_same<_T1, reference>::value>::type>
    reference &operator+=(const _T1&__other) {
        MdStaticArray(*__array_reference, offset, shp_offset).__add_self_iinternal(__other);
        return *this;
    }

    template <typename _T1>
    reference &operator-=(const MdStaticArray<_T1>&__other) {
        MdStaticArray(*__array_reference, offset, shp_offset).__sub_self_internal(__other);
        return *this;
    }
    
    reference &operator-=(const reference &__other) {
        MdStaticArray(*__array_reference, offset, shp_offset).
            __sub_self_internal(MdStaticArray(*__other.__array_reference, __other.offset, __other.shp_offset));
        return *this;
    }

    template <typename _T1, typename std::enable_if<!std::is_same<_T1, reference>::value>::type>
    reference &operator-=(const _T1&__other) {
        MdStaticArray(*__array_reference, offset, shp_offset).__sub_self_iinternal(__other);
        return *this;
    }

    template <typename _T1>
    reference &operator*=(const MdStaticArray<_T1>&__other) {
        MdStaticArray(*__array_reference, offset, shp_offset).__mul_self_internal(__other);
        return *this;
    }
    
    reference &operator*=(const reference &__other) {
        MdStaticArray(*__array_reference, offset, shp_offset).
            __mul_self_internal(MdStaticArray(*__other.__array_reference, __other.offset, __other.shp_offset));
        return *this;
    }

    template <typename _T1, typename std::enable_if<!std::is_same<_T1, reference>::value>::type>
    reference &operator*=(const _T1&__other) {
        MdStaticArray(*__array_reference, offset, shp_offset).__mul_self_iinternal(__other);
        return *this;
    }

    template <typename _T1>
    reference &operator/=(const MdStaticArray<_T1>&__other) {
        MdStaticArray(*__array_reference, offset, shp_offset).__div_self_internal(__other);
        return *this;
    }
    
    reference &operator/=(const reference &__other) {
        MdStaticArray(*__array_reference, offset, shp_offset).
            __div_self_internal(MdStaticArray(*__other.__array_reference, __other.offset, __other.shp_offset));
        return *this;
    }

    template <typename _T1, typename std::enable_if<!std::is_same<_T1, reference>::value>::type>
    reference &operator/=(const _T1&__other) {
        MdStaticArray(*__array_reference, offset, shp_offset).__div_self_iinternal(__other);
        return *this;
    }

    template <typename _T1>
    reference &operator%=(const MdStaticArray<_T1>&__other) {
        MdStaticArray(*__array_reference, offset, shp_offset).__mod_self_internal(__other);
        return *this;
    }
    
    reference &operator%=(const reference &__other) {
        MdStaticArray(*__array_reference, offset, shp_offset).
            __mod_self_internal(MdStaticArray(*__other.__array_reference, __other.offset, __other.shp_offset));
        return *this;
    }

    template <typename _T1, typename std::enable_if<!std::is_same<_T1, reference>::value>::type>
    reference &operator%=(const _T1&__other) {
        MdStaticArray(*__array_reference, offset, shp_offset).__mod_self_iinternal(__other);
        return *this;
    }

    ~reference() {
        __array_reference = nullptr;
    }
};

#endif
