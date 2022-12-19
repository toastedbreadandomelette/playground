#include <iostream>
#include "csv_reader.hpp"
#include "cell.hpp"
#include "column_view.hpp"
#include <chrono>
#include "md_utility.hpp"
#include <iomanip>
#include <cstring>

int main (int argc, const char **argv) {
    auto start = std::chrono::system_clock::now();
    if (argc != 2) {
        std::cout << "Error, no filename mentioned\n";
        exit(1);
    }

    Table table = read_csv(argv[1], 2);
    auto end = std::chrono::system_clock::now();
    std::chrono::duration<double> time = end - start;
    // std::cout << time.count() << "s" << std::endl;
    // MdStaticArray<uint64_t> d(200000000);
    // MdStaticArray<uint64_t>::set_s_thread_count(16);
    // for (int i = 0; i < d.get_size(); ++i) {
    //     d[i] = i + 1;
    // }

    // start = std::chrono::system_clock::now();
    // auto c = d * d;
    // end = std::chrono::system_clock::now();
    // for (int i = 0; i < 100; ++i) {
    //     std::cout << c[i] << " ";
    // }

    // std::cout << std::endl;
    // time = end - start;
    // std::cout << time.count() << "s" << std::endl;
    
    // std::cout << table << std::endl;
    // MdStaticArray<double> sepal_length = table["sepal.length"].st_map<double>();
    // MdStaticArray<double> sepal_width = table["sepal.width"].st_map<double>();
    // MdStaticArray<double> petal_length = table["petal.length"].st_map<double>();
    // MdStaticArray<double> petal_width = table["petal.width"].st_map<double>();
    // auto cd = table["sepal.length"] + table["petal.length"];

    auto split = table.split_dep_and_indep_variables<double, std::string>("variety");

    auto cd = "Versicolor" == split.second;

    for (int i = 0; i < cd.get_size(); ++i) {
        std::cout << cd[i] << " " << split.second[i] << "\n";
    }

    // std::cout << sepal_length.get_size() << " " << sepal_width.get_size() << " " << petal_length.get_size() << " " << petal_width.get_size() << std::endl;

    return 0;
}
