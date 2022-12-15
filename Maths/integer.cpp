#include <iostream>
#include <string>
#include <chrono>
#include "integer.hpp"

using namespace std::chrono;
int main () {
    integer p(0LL), q(11122312312312LL);
    time_point <system_clock> start, end; 
	duration <double> time;
    std::cout << p << '\n';
    start = system_clock::now();
    for (auto i = 0; i < 10005; ++i) {
        p += q;
    }
    end = system_clock::now();

    time = (end - start);
    std::cout << "Time: " << time.count() << "s\n";
    std::cout << p << '\n';
    return 0;
}
