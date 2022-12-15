#ifndef __INTEGER_HPP__
#define __INTEGER_HPP__

#include <string>
#include <vector>
#include <algorithm>
#include <ostream>

class integer;

template <>
struct std::is_integral<integer> : std::true_type {};
template <>
struct std::is_arithmetic<integer> : std::true_type {};

using uint = unsigned int;

void mul_by_2(std::string &A) {
    unsigned char sum = 0, carry = 0;
    const char _z = '0';
    for (auto &x: A) {
        sum = carry;
        sum += (x<<1);
        sum -= (_z<<1);
        carry = sum >= 10;
        if (carry) {
            sum -= 10;
        }
        x = sum;
        x += _z;
    }
    if (carry) {
        A.push_back(carry + _z);
    }
}

class integer {
    std::vector<uint> number;
    bool is_negative;
    const static uint mod = 0x3FFFFFFF, shft_by = 30;

    /**
     * @brief Adds two string denoting integer.
     * 
     * @param add_to 
     * @param add_by 
     * @return void 
     */
    void add(std::string &add_to, const std::string &add_by) {
        auto f = add_to.begin(), fe = add_to.end();
        auto s = add_by.begin(), se = add_by.end();
        unsigned char sum = 0, carry = 0;
        const char _z = '0';
        for (; f != fe && s != se; ++f, ++s) {
            sum = carry;
            sum += *f;
            sum += *s;
            sum -= (_z<<1);
            carry = (sum >= 10);
            if (carry) {
                sum -= 10;
            }
            *f = (sum + _z);
        }
        while (f != fe) {
            sum = carry;
            sum += *f;
            sum -= _z;
            carry = sum / 10;
            sum %= 10;
            *f = (sum + _z);
            ++f;
        }
        while (s != se) {
            sum = carry;
            sum += *s;
            sum -= _z;
            carry = sum / 10;
            sum %= 10;
            add_to.push_back(sum + _z);
            ++s;
        }
        if (carry != 0) {
            add_to.push_back(carry + _z);
        }
    }

    /**
     * @brief Display integer.
     * @param void
     * @return const std::string 
     */
    const std::string display_integer(void) {
        std::string pow_2 = "1", result = "0";
        const auto eiter = number.size();
        uint temp, it;
        for (auto iter = 0; iter < eiter; ++iter) {
            temp = number[iter];
            it = 29;
            while (it <= 29) {
                if (temp & 1) {
                    add(result, pow_2);
                }
                temp >>= 1;
                mul_by_2(pow_2);
                --it;
            }
        }
        std::reverse(result.begin(), result.end());
        return result;
    }

    /**
     * @brief Add self to other integer
     * @param __other 
     * @return integer 
     */
    void add_int(const integer &__other) {
        auto f = number.begin();
        const auto fe = number.end();
        auto s = __other.number.begin();
        const auto se = __other.number.end();
        uint sum = 0U, carry = 0U;

        for (; f != fe && s != se; ++f, ++s) {
            // sum = (carry + *f + *s);
            sum = carry;
            sum += *f;
            sum += *s;
            carry = sum >> shft_by;
            sum &= mod;
            *f = sum;
        }
        
        while (f != fe) {
            sum = carry;
            sum += *f;
            carry = sum >> shft_by;
            sum &= mod;
            *f = sum;
            ++f;
        }
        
        while (s != se) {
            sum = carry;
            sum += *s;
            carry = sum >> shft_by;
            sum &= mod;
            number.emplace_back(sum);
            ++s;
        }
        
        if (carry != 0) {
            number.emplace_back(carry);
        }
    }

    /**
     * @brief Add self to other integer
     * @param __other 
     * @return integer 
     */
    const integer add_int_and_create(const integer &__other) {
        auto f = number.begin();
        const auto fe = number.end();
        auto s = __other.number.begin();
        const auto se = __other.number.end();
        
        integer new_integer;

        uint sum = 0, carry = 0;
        for (; f != fe && s != se; ++f, ++s) {
            sum = (carry + *f + *s);
            carry = sum >> shft_by;
            sum &= mod;
            new_integer.number.emplace_back(sum);
        }
        
        while (f != fe) {
            sum = (carry + *f);
            carry = sum >> shft_by;
            sum &= mod;
            new_integer.number.emplace_back(sum);
            ++f;
        }
        
        while (s != se) {
            sum = (carry + *s);
            carry = sum >> shft_by;
            sum &= mod;
            new_integer.number.emplace_back(sum);
            ++s;
        }
        
        if (carry != 0) {
            new_integer.number.emplace_back(sum);
        }
        return new_integer;
    }

public:

    /**
     * @brief Copy constructor
     */
    integer(const integer &__other): number(__other.number), is_negative(__other.is_negative) {}

    /**
     * @brief Construct a new integer object
     * 
     */
    integer(): number(), is_negative(false) {}

    /**
     * @brief Construct a new integer object
     * 
     * @tparam _t 
     * @tparam std::enable_if<std::is_integral<_t>::value, _t>::type 
     * @param value 
     */
    template <typename _t, typename = typename std::enable_if<std::is_integral<_t>::value, _t>::type>
    integer(_t value) { 
        is_negative = value < 0;
        if (is_negative) value = -value;

        number.emplace_back((value & mod));
        if ((value >> 30) != 0) number.emplace_back((value >> 30) & mod);
        if ((value >> 60) != 0) number.emplace_back((value >> 60) & 0xFF);
    }

    /**
     * @brief 
     * 
     * @param __other 
     * @return integer& 
     */
    integer &operator=(const integer &__other) = default;

    /**
     * @brief self add operator for integer
     * 
     * @param __other different integer.
     * @return integer& 
     */
    inline integer &operator+=(const integer &__other) {
        add_int(__other);
        return *this;
    }

    /**
     * @brief Add operator overload for integer
     * 
     * @param __other other integer
     * @return integer
     */
    inline integer operator+(const integer &__other) {
        return add_int_and_create(__other);
    }

    /**
     * @brief ostream operator overload
     * 
     * @param op standard ostream operator
     * @param p integer to display
     * @return std::ostream& 
     */
    friend std::ostream &operator<<(std::ostream &op, integer &p) {
        op << p.display_integer();
        return op;
    }
};

#endif
