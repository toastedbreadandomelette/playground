#include "../md_dyn_array.hpp"

class LinearRegression {
    // __y independent variables
    MdDynArray<double> __y;
    // __x dependent variables 
    MdDynArray<MdDynArray<double>> __x;
public:
    LinearRegression() { }

};
