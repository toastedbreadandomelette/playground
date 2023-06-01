#pragma once

#include <iostream>
#include <cmath>

#define PI 3.14159265

class complex {
    double r, i;
    static const double degtorad, radtodeg;
public:
    explicit complex(): r(0), i(0) {}
    
    complex(double r, double i): r(r), i(i) { }

    /**
     * @brief Real value of complex number
     */
    inline double real() const { return r; }

    /**
     * @brief Imaginary value of complex number
     */
    inline double img() const { return i; }

    /**
     * @brief Abs square
     */
    inline double abs_sq() const { return r*r + i*i; }

    /**
     * @brief Abs
     */
    inline double abs() const { return sqrt(r*r + i*i); }

    inline complex operator+(const complex other) const { return complex(r+other.r, i+other.i); }
    inline complex operator-(const complex other) const { return complex(r-other.r, i-other.i); }
    inline complex operator*(const complex other) const { return complex(r*other.r - i*other.i, r*other.i + i*other.r); }
    inline complex operator/(const complex other) const { return complex(r*other.r / other.abs_sq(), i*other.i / other.abs_sq()); }

    inline complex &operator+=(const complex other) { r += other.r, i += other.i; return *this; }
    inline complex &operator-=(const complex other) { r -= other.r, i -= other.i; return *this; }
    inline complex &operator/=(const complex other) { r *= (other.r/other.abs_sq()), i *= (other.i/other.abs_sq()); return *this; }

    inline complex &operator*=(const complex other) {
        const auto tr = r*other.r - i*other.i;
        i = r*other.i + i*other.r;
        r = tr;
        return *this;
    }

    static inline complex from_polar_rd(const double radius, const double angle) {
        return complex(radius*cos(angle), radius*sin(angle));
    }

    static inline complex from_polar_dg(const double radius, const double angle) {
        const double rad = angle * degtorad;
        return complex(radius*cos(rad), radius*sin(rad));
    }

    /**
     * @brief return pair<Radius, Angle(In Radians)>
     */
    inline std::pair<double, double> polar_rd() const { return { abs(), atan(i/r) }; }

    /**
     * @brief return polar form of complex values pair<Radius, Angle(In Degrees)>
     */
    inline std::pair<double, double> polar_dg() const { return { abs(), atan(i/r) * radtodeg }; }

    inline complex pow(const double number) const {
        auto angle = (i/r)*number;
        while (angle > 2*PI) { angle -= 2*PI; }
        return from_polar_rd(::pow(abs(), number), angle);
    }

    inline bool operator==(const complex other) const { return r == other.r && i == other.i; }

    friend std::ostream &operator<<(std::ostream&op, const complex c) {
        op << c.r;
        if (c.i) { op << (c.i >= 0 ? '+' : '\0') << c.i << "i"; }
        return op;
    }
};

const double complex::radtodeg = 180.0f/PI, complex::degtorad = PI/180.0f;
