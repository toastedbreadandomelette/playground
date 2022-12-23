#include <iostream>
#include "csv_reader.hpp"
#include "cell.hpp"
#include "column_view.hpp"
#include <chrono>
#include "md_utility.hpp"

int main (int argc, const char **argv) {
    auto start = std::chrono::system_clock::now();
    // if (argc != 2) {
    //     std::cout << "Error, no filename mentioned\n";
    //     exit(1);
    // }

    // Table table = read_csv(argv[1], 2);
    auto end = std::chrono::system_clock::now();
    std::chrono::duration<double> time = end - start;
    // std::cout << time.count() << "s" << std::endl;
    size_t sz = 600;
    MdStaticArray<uint64_t> d(std::vector<size_t>({sz, sz, sz}), 10);
    MdStaticArray<int> e(std::vector<size_t>({sz, sz}), 19);
    for (int i = 0; i < d.get_size(); ++i) {
        d[i / (sz * sz)][(i / sz) % sz][i % sz] = i + 1;
    }

    for (int i = 0; i < 36; ++i) {
        std::cout << d[1][(i / sz) % sz][i % sz] << " ";
    }

    std::cout << std::endl;
    start = std::chrono::system_clock::now();
    MdStaticArray result = d[1] * pow(243, 23);
    end = std::chrono::system_clock::now();
    
    for (int i = 0; i < 36; ++i) {
        std::cout << d[1][(i / sz) % sz][i % sz] << " ";
    }
    std::cout << std::endl;
    for (int i = 0; i < 36; ++i) {
        std::cout << result[(i / sz) % sz][i % sz] << " ";
    }

    std::cout << std::endl;
    time = end - start;
    std::cout << time.count() << "s" << std::endl;
    
    // std::cout << table << std::endl;
    // MdStaticArray<double> sepal_length = table["sepal.length"].st_map<double>();
    // MdStaticArray<double> sepal_width = table["sepal.width"].st_map<double>();
    // MdStaticArray<double> petal_length = table["petal.length"].st_map<double>();
    // MdStaticArray<double> petal_width = table["petal.width"].st_map<double>();
    // auto cd = table["sepal.length"] + table["petal.length"];

    // auto split = table.split_dep_and_indep_variables<double, std::string>("variety");

    // auto cd = "Versicolor" == split.second;

    // for (int i = 0; i < cd.get_size(); ++i) {
    //     std::cout << cd[i] << " " << split.second[i] << "\n";
    // }

    // std::cout << sepal_length.get_size() << " " << sepal_width.get_size() << " " << petal_length.get_size() << " " << petal_width.get_size() << std::endl;

    return 0;
}
